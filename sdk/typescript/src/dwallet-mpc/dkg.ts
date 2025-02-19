// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { create_dkg_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { Transaction } from '@mysten/sui/transactions';
import { delay } from 'msw';

import type { Config } from './globals.js';
import {
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	DWALLET_NETWORK_VERSION,
	MPCKeyScheme,
	SUI_PACKAGE_ID,
} from './globals.js';

/**
 * Represents the Move `SystemInnerV1` struct.
 */
interface IKASystemStateInner {
	fields: {
		value: {
			fields: {
				dwallet_2pc_mpc_secp256k1_id: string;
				dwallet_network_decryption_key: {
					fields: {
						dwallet_network_decryption_key_id: string;
					};
				};
			};
		};
	};
}

/**
 * Represents a Move shared object owner.
 */
interface SharedObjectOwner {
	Shared: {
		// The object version when it became shared.
		initial_shared_version: number;
	};
}

interface StartDKGFirstRoundEvent {
	event_data: {
		dwallet_id: string;
	};
	session_id: string;
}

interface WaitingForUserDWallet {
	state: {
		fields: {
			first_round_output: Uint8Array;
		};
	};
}

function isStartDKGFirstRoundEvent(obj: any): obj is StartDKGFirstRoundEvent {
	return obj?.event_data?.dwallet_id !== undefined && obj?.session_id !== undefined;
}

export async function createDWallet(conf: Config, protocolPublicParameters: Uint8Array) {
	let firstRoundOutputResult = await launchDKGFirstRound(conf);
	console.log('First round output:', firstRoundOutputResult);
	const [
		centralizedPublicKeyShareAndProof,
		centralizedPublicOutput,
		centralizedSecretKeyShare,
		serializedPublicKeys,
	] = create_dkg_centralized_output(
		protocolPublicParameters,
		MPCKeyScheme.Secp256k1,
		Uint8Array.from(firstRoundOutputResult.output),
		// Remove the 0x prefix.
		firstRoundOutputResult.sessionID.slice(2),
	);

	console.log({
		centralizedPublicKeyShareAndProof,
		centralizedPublicOutput,
		centralizedSecretKeyShare,
		serializedPublicKeys,
	});
}

interface DKGFirstRoundOutputResult {
	sessionID: string;
	output: Uint8Array;
}

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round,
 * and as input for the centralized party round.
 */
export async function launchDKGFirstRound(c: Config): Promise<DKGFirstRoundOutputResult> {
	const tx = new Transaction();
	let emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${c.ikaConfig.ika_package_id}::ika::IKA`],
	});
	let networkDecryptionKeyID = await getNetworkDecryptionKeyID(c);
	let dwalletSecp256k1ID = await getDwalletSecp256k1ObjID(c);
	let dwalletCap = tx.moveCall({
		target: `${c.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::request_dkg_first_round`,
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
		typeArguments: [`${c.ikaConfig.ika_package_id}::ika::IKA`],
	});
	const result = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	let startDKGEvent = result.events?.at(0)?.parsedJson;
	if (!isStartDKGFirstRoundEvent(startDKGEvent)) {
		throw new Error('invalid start DKG first round event');
	}
	let dwalletID = startDKGEvent.event_data.dwallet_id;
	let output = await waitForDKGFirstRoundOutput(c, dwalletID);
	return {
		sessionID: startDKGEvent.session_id,
		output: output,
	};
}

function isWaitingForUserDWallet(obj: any): obj is WaitingForUserDWallet {
	return obj?.state?.fields?.first_round_output !== undefined;
}

interface MoveObject {
	fields: any;
}

function isMoveObject(obj: any): obj is MoveObject {
	return obj?.fields !== undefined;
}

async function waitForDKGFirstRoundOutput(conf: Config, dwalletID: string): Promise<Uint8Array> {
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await delay(5_000);
		let dwallet = await conf.client.getObject({
			id: dwalletID,
			options: {
				showContent: true,
			},
		});
		if (isMoveObject(dwallet?.data?.content)) {
			let dwalletMoveObject = dwallet?.data?.content?.fields;
			if (isWaitingForUserDWallet(dwalletMoveObject)) {
				return dwalletMoveObject.state.fields.first_round_output;
			}
		}
	}
	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch the DWallet object within ${
			conf.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

function isIKASystemStateInner(obj: any): obj is IKASystemStateInner {
	return (
		obj?.fields?.value?.fields?.dwallet_network_decryption_key !== undefined &&
		obj?.fields?.value?.fields?.dwallet_2pc_mpc_secp256k1_id !== undefined
	);
}

async function getDwalletSecp256k1ObjID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: c.ikaConfig.ika_system_obj_id,
	});
	let innerSystemState = await c.client.getDynamicFieldObject({
		parentId: c.ikaConfig.ika_system_obj_id,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	if (!isIKASystemStateInner(innerSystemState.data?.content)) {
		throw new Error('Invalid inner system state');
	}
	return innerSystemState.data?.content?.fields.value.fields.dwallet_2pc_mpc_secp256k1_id;
}

async function getNetworkDecryptionKeyID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: c.ikaConfig.ika_system_obj_id,
	});
	let innerSystemState = await c.client.getDynamicFieldObject({
		parentId: c.ikaConfig.ika_system_obj_id,
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
