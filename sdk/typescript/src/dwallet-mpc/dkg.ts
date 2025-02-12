// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Transaction } from '@mysten/sui/transactions';

import type { Config, StartSessionEvent } from './globals.js';

const IKA_PACKAGE_ID = '0x66dca2cee84af8b507879dd7745672bdaa089fa98e5cb98165e657ec466b908e';
const IKA_SYSTEM_PACKAGE_ID = '0x9b4ad924399f991023b9d053d4a81d880973d51c3e08bfa0c1ffb03e8f9d8436';
const DWALLET_ECDSAK1_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1';
const IKA_SYSTEM_OBJ_ID = '0x3eff62e4dfcbca5f92e5f7241041db2bfc0a0a64e15f047238805e3e9c15debe';
const DWALLET_NETWORK_VERSION = 0;
const SUI_PACKAGE_ID = '0x2';
const IKA_COIN_OBJECT_PATH = `${IKA_PACKAGE_ID}::ika::IKA`;

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

async function getDwalletSecp256k1ObjID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: IKA_SYSTEM_OBJ_ID,
	});
	let innerSystemState = await c.client.getDynamicFieldObject({
		parentId: IKA_SYSTEM_OBJ_ID,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	// @ts-ignore
	return innerSystemState.data.content.fields.value.fields.dwallet_2pc_mpc_secp256k1_id;
}

async function getNetworkDecryptionKeyID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: IKA_SYSTEM_OBJ_ID,
	});
	let innerSystemState = await c.client.getDynamicFieldObject({
		parentId: IKA_SYSTEM_OBJ_ID,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	// @ts-ignore
	return innerSystemState.data.content.fields.value.fields.dwallet_network_decryption_key.fields
		.dwallet_network_decryption_key_id;
}

async function getInitialSharedVersion(c: Config, objectID: string): Promise<number> {
	let obj = await c.client.getObject({
		id: objectID,
		options: {
			showOwner: true,
		},
	});
	// @ts-ignore
	return obj.data.owner.Shared.initial_shared_version;
}
