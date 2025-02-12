// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Transaction } from '@mysten/sui/transactions';

import {
	Config,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	DWALLET_NETWORK_VERSION,
	IKA_COIN_OBJECT_PATH,
	IKA_SYSTEM_OBJ_ID,
	IKA_SYSTEM_PACKAGE_ID,
	StartSessionEvent,
	SUI_PACKAGE_ID,
} from './globals.js';

interface SuiInnerSystemState {
	fields: {
		value: {
			fields: {
				dwallet_2pc_mpc_secp256k1_id: string;
			};
		};
	}
}

interface IKASystemStateInner {
	fields: {
		value: {
			fields: {
				dwallet_network_decryption_key: {
					fields: {
						dwallet_network_decryption_key_id: string;
					};
				};
			};
		};
	};
}

interface SharedObjectOwner {
	Shared: {
		initial_shared_version: number;
	};
}

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round,
 * and as input for the centralized party round.
 */
export async function launchDKGFirstRound(c: Config) {
	const tx = new Transaction();
	let emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [IKA_COIN_OBJECT_PATH],
	});
	let networkDecryptionKeyID = await getNetworkDecryptionKeyID(c);
	let dwalletSecp256k1ID = await getDwalletSecp256k1ObjID(c);
	let dwalletCap = tx.moveCall({
		target: `${IKA_SYSTEM_PACKAGE_ID}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::request_dkg_first_round`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dwalletSecp256k1ID,
				initialSharedVersion: await getInitialSharedVersion(c, dwalletSecp256k1ID),
				mutable: true,
			}),
			tx.pure.id(networkDecryptionKeyID),
			emptyIKACoin,
			tx.gas,
		],
	});
	tx.transferObjects([dwalletCap], c.keypair.toSuiAddress());
	tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::destroy_zero`,
		arguments: [emptyIKACoin],
		typeArguments: [IKA_COIN_OBJECT_PATH],
	});
	const result = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	let sessionID = (result.events?.at(0)?.parsedJson as StartSessionEvent).session_id;
	console.log(`Session ID: ${sessionID}`);
	// TODO (#631): Use the session ID to fetch the DKG first round completion event.
}

function isSuiInnerSystemState(obj: any): obj is SuiInnerSystemState {
	return obj?.fields?.value?.fields?.dwallet_2pc_mpc_secp256k1_id !== undefined;
}

async function getDwalletSecp256k1ObjID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: IKA_SYSTEM_OBJ_ID,
	});
	let innerSystemState = await c.client.getDynamicFieldObject({
		parentId: IKA_SYSTEM_OBJ_ID,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	if (!isSuiInnerSystemState(innerSystemState.data?.content)) {
		throw new Error('Invalid inner system state');
	}
	return innerSystemState.data?.content?.fields.value.fields.dwallet_2pc_mpc_secp256k1_id;
}

function isIKASystemStateInner(obj: any): obj is IKASystemStateInner {
	return obj?.fields?.value?.fields?.dwallet_network_decryption_key !== undefined;
}

async function getNetworkDecryptionKeyID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: IKA_SYSTEM_OBJ_ID,
	});
	let innerSystemState = await c.client.getDynamicFieldObject({
		parentId: IKA_SYSTEM_OBJ_ID,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	if (!isIKASystemStateInner(innerSystemState.data?.content)) {
		throw new Error('Invalid inner system state');
	}

	return innerSystemState.data.content.fields.value.fields.dwallet_network_decryption_key.fields
		.dwallet_network_decryption_key_id;
}

function isSharedObjectOwner(obj: any): obj is SharedObjectOwner {
	return obj?.Shared?.initial_shared_version !== undefined;
}

async function getInitialSharedVersion(c: Config, objectID: string): Promise<number> {
	let obj = await c.client.getObject({
		id: objectID,
		options: {
			showOwner: true,
		},
	});
	let owner = obj.data?.owner;
	if (!owner || !isSharedObjectOwner(owner)) {
		throw new Error('Object is not shared');
	}
	return owner.Shared?.initial_shared_version;
}
