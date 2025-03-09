// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import {
	create_dkg_centralized_output,
	encrypt_secret_share,
} from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import type { ClassGroupsSecpKeyPair } from './encrypt-user-share.js';
import { getOrCreateClassGroupsKeyPair } from './encrypt-user-share.js';
import {
	checkpointCreationTime,
	delay,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	DWALLET_NETWORK_VERSION,
	fetchCompletedEvent,
	getDwalletSecp256k1ObjID,
	getDWalletSecpState,
	getInitialSharedVersion,
	getObjectWithType,
	isActiveDWallet,
	isAddressObjectOwner,
	isDWalletCap,
	isIKASystemStateInner,
	isMoveObject,
	isStartSessionEvent,
	MPCKeyScheme,
	SUI_PACKAGE_ID,
} from './globals.js';
import type { Config, SharedObjectData } from './globals.ts';

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

interface DWallet {
	dwallet_id: string;
	dwallet_cap_id: string;
	secret_share: Uint8Array;
}

function isStartDKGFirstRoundEvent(obj: any): obj is StartDKGFirstRoundEvent {
	return (
		!!obj?.event_data?.dwallet_id &&
		!!obj?.session_id &&
		!!obj?.event_data?.dwallet_cap_id &&
		!!obj?.event_data?.dwallet_network_decryption_key_id
	);
}

export async function createDWallet(
	conf: Config,
	protocolPublicParameters: Uint8Array,
): Promise<DWallet> {
	const firstRoundOutputResult = await launchDKGFirstRound(conf);
	const classGroupsSecpKeyPair = await getOrCreateClassGroupsKeyPair(conf);
	const dwalletOutput = await launchDKGSecondRound(
		conf,
		firstRoundOutputResult,
		protocolPublicParameters,
		classGroupsSecpKeyPair,
	);
	return {
		dwallet_id: firstRoundOutputResult.dwalletID,
		dwallet_cap_id: firstRoundOutputResult.dwalletCapID,
		secret_share: dwalletOutput.secretShare,
	};
}

interface SecondResult {
	dwalletOutput: Uint8Array;
	secretShare: Uint8Array;
}

export async function launchDKGSecondRound(
	conf: Config,
	firstRoundOutputResult: DKGFirstRoundOutputResult,
	protocolPublicParameters: Uint8Array,
	classGroupsSecpKeyPair: ClassGroupsSecpKeyPair,
): Promise<SecondResult> {
	const [centralizedPublicKeyShareAndProof, centralizedPublicOutput, centralizedSecretKeyShare] =
		create_dkg_centralized_output(
			protocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(firstRoundOutputResult.output),
			// Remove the 0x prefix.
			firstRoundOutputResult.sessionID.slice(2),
		);
	const dWalletStateData = await getDWalletSecpState(conf);

	const encryptedUserShareAndProof = encrypt_secret_share(
		centralizedSecretKeyShare,
		classGroupsSecpKeyPair.encryptionKey,
	);

	const completionEvent = await dkgSecondRoundMoveCall(
		conf,
		dWalletStateData,
		firstRoundOutputResult,
		centralizedPublicKeyShareAndProof,
		encryptedUserShareAndProof,
		centralizedPublicOutput,
	);
	return {
		dwalletOutput: completionEvent.public_output,
		secretShare: centralizedSecretKeyShare,
	};
}

/**
 * Creates a valid mock output of the first DKG blockchain round.
 */
export async function createDKGFirstRoundOutputMock(
	conf: Config,
	mockOutput: Uint8Array,
): Promise<DKGFirstRoundOutputResult> {
	const tx = new Transaction();
	const dwalletStateObjData = await getDWalletSecpState(conf);
	const stateArg = tx.sharedObjectRef({
		objectId: dwalletStateObjData.object_id,
		initialSharedVersion: dwalletStateObjData.initial_shared_version,
		mutable: true,
	});
	const firstRoundOutputArg = tx.pure(bcs.vector(bcs.u8()).serialize(mockOutput));
	const networkDecryptionKeyID = await getNetworkDecryptionKeyID(conf);
	const networkDecryptionKeyIDArg = tx.pure.id(networkDecryptionKeyID);
	const dwalletCap = tx.moveCall({
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
	const dwalletCapObj = await getObjectWithType(
		conf,
		createdDWalletCap.reference.objectId,
		isDWalletCap,
	);
	return {
		dwalletCapID: createdDWalletCap.reference.objectId,
		dwalletID: dwalletCapObj.dwallet_id,
		sessionID: '',
		output: mockOutput,
	};
}

/**
 * Creates a valid mock output of the first DKG blockchain round.
 */
export async function mockCreateDWallet(
	conf: Config,
	mockOutput: Uint8Array,
): Promise<DKGFirstRoundOutputResult> {
	const tx = new Transaction();
	const dwalletStateObjData = await getDWalletSecpState(conf);
	const stateArg = tx.sharedObjectRef({
		objectId: dwalletStateObjData.object_id,
		initialSharedVersion: dwalletStateObjData.initial_shared_version,
		mutable: true,
	});
	const firstRoundOutputArg = tx.pure(bcs.vector(bcs.u8()).serialize(mockOutput));
	const networkDecryptionKeyID = await getNetworkDecryptionKeyID(conf);
	const networkDecryptionKeyIDArg = tx.pure.id(networkDecryptionKeyID);
	const dwalletCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::mock_create_dwallet`,
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
	const dwalletCapObj = await getObjectWithType(
		conf,
		createdDWalletCap.reference.objectId,
		isDWalletCap,
	);

	return {
		dwalletCapID: createdDWalletCap.reference.objectId,
		dwalletID: dwalletCapObj.dwallet_id,
		sessionID: '',
		output: mockOutput,
	};
}

export async function dkgSecondRoundMoveCall(
	conf: Config,
	dWalletStateData: SharedObjectData,
	firstRoundOutputResult: DKGFirstRoundOutputResult,
	centralizedPublicKeyShareAndProof: Uint8Array,
	encryptedUserShareAndProof: Uint8Array,
	centralizedPublicOutput: Uint8Array,
): Promise<CompletedDKGSecondRoundEvent> {
	const tx = new Transaction();
	const dwalletStateArg = tx.sharedObjectRef({
		objectId: dWalletStateData.object_id,
		initialSharedVersion: dWalletStateData.initial_shared_version,
		mutable: true,
	});
	const dwalletCapArg = tx.object(firstRoundOutputResult.dwalletCapID);
	const centralizedPublicKeyShareAndProofArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(centralizedPublicKeyShareAndProof),
	);
	const encryptedCentralizedSecretShareAndProofArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(encryptedUserShareAndProof),
	);
	const encryptionKeyAddressArg = tx.pure.id(
		conf.encryptedSecretShareSigningKeypair.toSuiAddress(),
	);
	const userPublicOutputArg = tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicOutput));
	const signerPublicKeyArg = tx.pure(
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
	const result = await conf.client.signAndExecuteTransaction({
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
	const startSessionEvent = result.events?.at(0)?.parsedJson;
	if (!isStartSessionEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}
	const completionEvent = await fetchCompletedEvent(
		conf,
		startSessionEvent.session_id,
		isCompletedDKGSecondRoundEvent,
	);
	return completionEvent;
}

function isCompletedDKGSecondRoundEvent(obj: any): obj is CompletedDKGSecondRoundEvent {
	return (
		obj.dwallet_id !== undefined &&
		obj.public_output !== undefined &&
		obj.encrypted_user_secret_key_share_id !== undefined &&
		obj.session_id !== undefined
	);
}

interface CompletedDKGSecondRoundEvent {
	dwallet_id: string;
	public_output: Uint8Array;
	encrypted_user_secret_key_share_id: string;
	session_id: string;
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
	const emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${c.ikaConfig.ika_package_id}::ika::IKA`],
	});
	const networkDecryptionKeyID = await getNetworkDecryptionKeyID(c);
	const dwalletSecp256k1ID = await getDwalletSecp256k1ObjID(c);
	const dwalletCap = tx.moveCall({
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
	const startDKGEvent = result.events?.at(0)?.parsedJson;
	if (!isStartDKGFirstRoundEvent(startDKGEvent)) {
		throw new Error('invalid start DKG first round event');
	}
	const dwalletID = startDKGEvent.event_data.dwallet_id;
	const output = await waitForDKGFirstRoundOutput(c, dwalletID);
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

async function waitForDKGFirstRoundOutput(conf: Config, dwalletID: string): Promise<Uint8Array> {
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await delay(5_000);
		const dwallet = await conf.client.getObject({
			id: dwalletID,
			options: {
				showContent: true,
			},
		});
		if (isMoveObject(dwallet?.data?.content)) {
			const dwalletMoveObject = dwallet?.data?.content?.fields;
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
	const innerSystemState = await c.client.getDynamicFieldObject({
		parentId: c.ikaConfig.ika_system_obj_id,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	if (!isIKASystemStateInner(innerSystemState.data?.content)) {
		throw new Error('Invalid inner system state');
	}

	return innerSystemState.data.content.fields.value.fields.dwallet_network_decryption_key.fields
		.dwallet_network_decryption_key_id;
}

// public fun accept_encrypted_user_share(
// 	self: &mut DWallet2PcMpcSecp256K1,
// 	dwallet_id: ID,
// 	encrypted_user_secret_key_share_id: ID,
// 	user_output_signature: vector<u8>,
// ) {

export async function acceptEncryptedUserShare(
	conf: Config,
	completedDKGSecondRoundEvent: CompletedDKGSecondRoundEvent,
): Promise<void> {
	let signedPubkeys = await conf.encryptedSecretShareSigningKeypair.sign(
		completedDKGSecondRoundEvent.public_output,
	);
	let dWalletStateData = await getDWalletSecpState(conf);
	const tx = new Transaction();
	const dwalletStateArg = tx.sharedObjectRef({
		objectId: dWalletStateData.object_id,
		initialSharedVersion: dWalletStateData.initial_shared_version,
		mutable: true,
	});
	let dwalletIDArg = tx.pure.id(completedDKGSecondRoundEvent.dwallet_id);
	let encryptedUserSecretKeyShareIDArg = tx.pure.id(
		completedDKGSecondRoundEvent.encrypted_user_secret_key_share_id,
	);
	let userOutputSignatureArg = tx.pure(bcs.vector(bcs.u8()).serialize(signedPubkeys));
	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::accept_encrypted_user_share`,
		arguments: [
			dwalletStateArg,
			dwalletIDArg,
			encryptedUserSecretKeyShareIDArg,
			userOutputSignatureArg,
		],
	});
	await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
}
