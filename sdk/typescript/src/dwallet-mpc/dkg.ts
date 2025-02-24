// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Buffer } from 'buffer';
import {
	create_dkg_centralized_output,
	encrypt_secret_share,
} from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import type { ClassGroupsSecpKeyPair } from './encrypt-user-share.ts';
import { getOrCreateClassGroupsKeyPair } from './encrypt-user-share.ts';
import {
	checkpointCreationTime,
	Config,
	delay,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	DWALLET_NETWORK_VERSION,
	getDwalletSecp256k1ObjID,
	getDWalletSecpState,
	getInitialSharedVersion,
	isAddressObjectOwner,
	isIKASystemStateInner,
	isMoveObject,
	MPCKeyScheme,
	SharedObjectData,
	SUI_PACKAGE_ID,
} from './globals.ts';

interface StartDKGFirstRoundEvent {
	event_data: {
		dwallet_id: string;
		dwallet_cap_id: string;
		dwallet_network_decryption_key_id: string;
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

interface ActiveDWallet {
	state: {
		fields: {
			public_output: Uint8Array;
		};
	};
}

function isStartDKGFirstRoundEvent(obj: any): obj is StartDKGFirstRoundEvent {
	return (
		!!obj?.event_data?.dwallet_id &&
		!!obj?.session_id &&
		!!obj?.event_data?.dwallet_cap_id &&
		!!obj?.event_data?.dwallet_network_decryption_key_id
	);
}

export async function createDWallet(conf: Config, protocolPublicParameters: Uint8Array) {
	let firstRoundOutputResult = await launchDKGFirstRound(conf);
	let classGroupsSecpKeyPair = await getOrCreateClassGroupsKeyPair(conf);
	return await launchDKGSecondRound(
		conf,
		firstRoundOutputResult,
		protocolPublicParameters,
		classGroupsSecpKeyPair,
	);
}

export async function launchDKGSecondRound(
	conf: Config,
	firstRoundOutputResult: DKGFirstRoundOutputResult,
	protocolPublicParameters: Uint8Array,
	classGroupsSecpKeyPair: ClassGroupsSecpKeyPair,
) {
	const [centralizedPublicKeyShareAndProof, centralizedPublicOutput, centralizedSecretKeyShare] =
		create_dkg_centralized_output(
			protocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(firstRoundOutputResult.output),
			// Remove the 0x prefix.
			firstRoundOutputResult.sessionID.slice(2),
		);
	let dWalletStateData = await getDWalletSecpState(conf);

	console.log(
		'centralizedPublicKeyShareAndProof',
		Buffer.from(centralizedPublicKeyShareAndProof).toString('base64'),
	);
	console.log('centralizedPublicOutput', Buffer.from(centralizedPublicOutput).toString('base64'));
	console.log(
		'centralizedSecretKeyShare',
		Buffer.from(centralizedSecretKeyShare).toString('base64'),
	);
	console.log('first round output', Buffer.from(firstRoundOutputResult.output).toString('base64'));
	console.log('sessionID', firstRoundOutputResult.sessionID);

	// TODO (#672): Fix the encrypt_secret_share wasm function.
	// const encryptedUserShareAndProof = encrypt_secret_share(
	// 	centralizedSecretKeyShare,
	// 	classGroupsSecpKeyPair.encryptionKey,
	// );
	//
	// await dkgSecondRoundMoveCall(
	// 	conf,
	// 	dWalletStateData,
	// 	firstRoundOutputResult,
	// 	centralizedPublicKeyShareAndProof,
	// 	encryptedUserShareAndProof,
	// 	centralizedPublicOutput,
	// );
}

/**
 * Creates a valid mock output of the first DKG blockchain round.
 */
export async function createDKGFirstRoundOutputMock(
	conf: Config,
	mockOutput: Uint8Array,
): Promise<string> {
	// create_first_round_dwallet_mock(self: &mut DWallet2PcMpcSecp256K1, first_round_output: vector<u8>, dwallet_network_decryption_key_id: ID, ctx: &mut TxContext): DWalletCap
	const tx = new Transaction();
	let dwalletStateObjData = await getDWalletSecpState(conf);
	let stateArg = tx.sharedObjectRef({
		objectId: dwalletStateObjData.object_id,
		initialSharedVersion: dwalletStateObjData.initial_shared_version,
		mutable: true,
	});
	let firstRoundOutputArg = tx.pure(bcs.vector(bcs.u8()).serialize(mockOutput));
	let networkDecryptionKeyID = await getNetworkDecryptionKeyID(conf);
	let networkDecryptionKeyIDArg = tx.pure.id(networkDecryptionKeyID);
	let dwalletCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::create_first_round_dwallet_mock`,
		arguments: [stateArg, firstRoundOutputArg, networkDecryptionKeyIDArg],
	});
	tx.transferObjects([dwalletCap], conf.suiClientKeypair.toSuiAddress());
	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	const createdDWalletCap = result?.effects?.created?.find(
		(obj) =>
			isAddressObjectOwner(obj.owner) &&
			obj.owner.AddressOwner === conf.suiClientKeypair.toSuiAddress(),
	);
	if (!dwalletCap || createdDWalletCap === undefined) {
		throw new Error('Unable to create the DWallet cap');
	}
	await delay(checkpointCreationTime);
	let dwalletCapObj = await conf.client.getObject({
		id: createdDWalletCap.reference.objectId,
		options: { showContent: true },
	});
	let dwalletCapObj = dwalletCapObj?.data?.content;
	if (!isMoveObject(dwalletCapObj)) {
		throw new Error('Invalid DWallet cap object');
	}
	let dwalletCapFields = dwalletCapObj.fields;
	if (!isDWalletCap(dwalletCapFields)) {
		throw new Error('Invalid DWallet cap fields');
	}

	return {
		dwalletCapID: createdDWalletCap.reference.objectId,
		dwalletID: dwalletCapFields.dwallet_id,
	}
	let dwalletID = dwalletCapFields.dwallet_id;

	return createdDWalletCap.reference.objectId;
}

interface DWalletCap {
	dwallet_id: string;
}

function isDWalletCap(obj: any): obj is DWalletCap {
	return !!obj?.dwallet_id;
}

export async function dkgSecondRoundMoveCall(
	conf: Config,
	dWalletStateData: SharedObjectData,
	firstRoundOutputResult: DKGFirstRoundOutputResult,
	centralizedPublicKeyShareAndProof: Uint8Array,
	encryptedUserShareAndProof: Uint8Array,
	centralizedPublicOutput: Uint8Array,
) {
	const tx = new Transaction();
	let dwalletStateArg = tx.sharedObjectRef({
		objectId: dWalletStateData.object_id,
		initialSharedVersion: dWalletStateData.initial_shared_version,
		mutable: true,
	});
	let dwalletCapArg = tx.object(firstRoundOutputResult.dwalletCapID);
	let centralizedPublicKeyShareAndProofArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(centralizedPublicKeyShareAndProof),
	);
	let encryptedCentralizedSecretShareAndProofArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(encryptedUserShareAndProof),
	);
	let encryptionKeyAddressArg = tx.pure.id(conf.encryptedSecretShareSigningKeypair.toSuiAddress());
	let userPublicOutputArg = tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicOutput));
	let signerPublicKeyArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(conf.suiClientKeypair.getPublicKey().toRawBytes()),
	);

	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::request_dkg_second_round`,
		arguments: [
			dwalletStateArg,
			dwalletCapArg,
			centralizedPublicKeyShareAndProofArg,
			encryptedCentralizedSecretShareAndProofArg,
			encryptionKeyAddressArg,
			userPublicOutputArg,
			signerPublicKeyArg,
		],
	});
	let result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	if (result.errors !== undefined) {
		throw new Error(`DKG second round failed with errors ${result.errors}`);
	}
	return await waitForDKGSecondRoundOutput(conf, firstRoundOutputResult.dwalletID);
}

interface DKGFirstRoundOutputResult {
	sessionID: string;
	output: Uint8Array;
	dwalletCapID: string;
	dwalletID: string;
}

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round,
 * and as input for the centralized party round.
 */
async function launchDKGFirstRound(c: Config): Promise<DKGFirstRoundOutputResult> {
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
	tx.transferObjects([dwalletCap], c.suiClientKeypair.toSuiAddress());
	tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::destroy_zero`,
		arguments: [emptyIKACoin],
		typeArguments: [`${c.ikaConfig.ika_package_id}::ika::IKA`],
	});
	const result = await c.client.signAndExecuteTransaction({
		signer: c.suiClientKeypair,
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
		dwalletCapID: startDKGEvent.event_data.dwallet_cap_id,
		dwalletID,
	};
}

function isWaitingForUserDWallet(obj: any): obj is WaitingForUserDWallet {
	return obj?.state?.fields?.first_round_output !== undefined;
}

function isActiveDWallet(obj: any): obj is ActiveDWallet {
	return obj?.state?.fields?.public_output !== undefined;
}

async function waitForDKGSecondRoundOutput(conf: Config, dwalletID: string): Promise<Uint8Array> {
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
			if (isActiveDWallet(dwalletMoveObject)) {
				return dwalletMoveObject.state.fields.public_output;
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
