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
import type { ActiveDWallet, DWallet, EncryptedDWalletData } from './globals.js';
import {
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	// getDwalletSecp256k1ObjID, // No longer directly used
	getDWalletStateArg,
	// getInitialSharedVersion, // No longer directly used
	executeTransactionAndGetMainEvent,
	getNetworkDecryptionKeyID,
	getObjectWithType,
	handleIKACoin,
	isActiveDWallet,
	isMoveObject,
	MPCKeyScheme,
	waitForCondition,
} from './globals.js';
import type { Config } from './globals.ts';

interface StartDKGFirstRoundEvent {
	event_data: {
		dwallet_id: string;
		dwallet_cap_id: string;
		dwallet_network_encryption_key_id: string;
	};
	session_id: string;
}

interface StartDKGSecondRoundEvent {
	event_data: {
		encrypted_user_secret_key_share_id: string;
	};
	session_id: string;
}

function isStartDKGSecondRoundEvent(obj: any): obj is StartDKGSecondRoundEvent {
	return !!obj?.event_data?.encrypted_user_secret_key_share_id && !!obj?.session_id;
}

interface DKGSecondRoundMoveResponse {
	dwallet: ActiveDWallet;
	encrypted_user_secret_key_share_id: string;
}

interface DKGSecondRoundResponse {
	moveResponse: DKGSecondRoundMoveResponse;
	secretShare: Uint8Array;
}

interface WaitingForUserDWallet {
	state: {
		fields: {
			first_round_output: Uint8Array;
		};
	};
}

function isStartDKGFirstRoundEvent(obj: any): obj is StartDKGFirstRoundEvent {
	return (
		!!obj?.event_data?.dwallet_id &&
		!!obj?.session_id &&
		!!obj?.event_data?.dwallet_cap_id &&
		!!obj?.event_data?.dwallet_network_encryption_key_id
	);
}

export async function createDWallet(
	conf: Config,
	networkDecryptionKeyPublicOutput: Uint8Array,
): Promise<DWallet> {
	const firstRoundOutputResult = await launchDKGFirstRound(conf);
	const classGroupsSecpKeyPair = await getOrCreateClassGroupsKeyPair(conf);
	const secondRoundResponse = await launchDKGSecondRound(
		conf,
		firstRoundOutputResult,
		networkDecryptionKeyPublicOutput,
		classGroupsSecpKeyPair,
	);
	await acceptEncryptedUserShare(conf, {
		dwallet_id: secondRoundResponse.moveResponse.dwallet.id.id,
		encrypted_user_secret_key_share_id:
			secondRoundResponse.moveResponse.encrypted_user_secret_key_share_id,
	});
	return {
		dwalletID: firstRoundOutputResult.dwalletID,
		dwallet_cap_id: firstRoundOutputResult.dwalletCapID,
		secret_share: secondRoundResponse.secretShare,
		output: secondRoundResponse.moveResponse.dwallet.state.fields.public_output,
		encrypted_secret_share_id: secondRoundResponse.moveResponse.encrypted_user_secret_key_share_id,
	};
}

export async function launchDKGSecondRound(
	conf: Config,
	firstRoundOutputResult: DKGFirstRoundOutputResult,
	networkDecryptionKeyPublicOutput: Uint8Array,
	classGroupsSecpKeyPair: ClassGroupsSecpKeyPair,
): Promise<DKGSecondRoundResponse> {
	const [centralizedPublicKeyShareAndProof, centralizedPublicOutput, centralizedSecretKeyShare] =
		create_dkg_centralized_output(
			networkDecryptionKeyPublicOutput,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(firstRoundOutputResult.output),
			// Remove the 0x prefix.
			firstRoundOutputResult.sessionID.slice(2),
		);

	const encryptedUserShareAndProof = encrypt_secret_share(
		centralizedSecretKeyShare,
		classGroupsSecpKeyPair.encryptionKey,
		networkDecryptionKeyPublicOutput,
	);

	const secondRoundMoveResponse = await dkgSecondRoundMoveCall(
		conf,
		firstRoundOutputResult,
		centralizedPublicKeyShareAndProof,
		encryptedUserShareAndProof,
		centralizedPublicOutput,
	);
	return {
		moveResponse: secondRoundMoveResponse,
		secretShare: centralizedSecretKeyShare,
	};
}

export async function dkgSecondRoundMoveCall(
	conf: Config,
	firstRoundOutputResult: DKGFirstRoundOutputResult,
	centralizedPublicKeyShareAndProof: Uint8Array,
	encryptedUserShareAndProof: Uint8Array,
	centralizedPublicOutput: Uint8Array,
): Promise<DKGSecondRoundMoveResponse> {
	const tx = new Transaction();
	const dwalletStateArg = await getDWalletStateArg(conf, tx, true);
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

	const emptyIKACoin = handleIKACoin(tx, conf);
	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_dwallet_dkg_second_round`,
		arguments: [
			dwalletStateArg,
			dwalletCapArg,
			centralizedPublicKeyShareAndProofArg,
			encryptedCentralizedSecretShareAndProofArg,
			encryptionKeyAddressArg,
			userPublicOutputArg,
			signerPublicKeyArg,
			emptyIKACoin,
			tx.gas,
		],
	});

	const startSessionEvent = await executeTransactionAndGetMainEvent<StartDKGSecondRoundEvent>(
		conf,
		tx,
		isStartDKGSecondRoundEvent,
		'DKG second round failed',
	);

	const dwallet = await getObjectWithType(conf, firstRoundOutputResult.dwalletID, isActiveDWallet);
	return {
		dwallet,
		encrypted_user_secret_key_share_id:
			startSessionEvent.event_data.encrypted_user_secret_key_share_id,
	};
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
	const emptyIKACoin = handleIKACoin(tx, c);
	const networkDecryptionKeyID = await getNetworkDecryptionKeyID(c);
	// dwalletSecp256k1ID is implicitly handled by getDWalletStateArg
	// const dwalletSecp256k1ID = await getDwalletSecp256k1ObjID(c); 
	const dwalletStateArg = await getDWalletStateArg(c, tx, true);
	const dwalletCap = tx.moveCall({
		target: `${c.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_dwallet_dkg_first_round`,
		arguments: [
			dwalletStateArg,
			tx.pure.id(networkDecryptionKeyID),
			tx.pure.u32(0),
			emptyIKACoin,
			tx.gas,
		],
	});
	tx.transferObjects([dwalletCap], c.suiClientKeypair.toSuiAddress());

	const startDKGEvent = await executeTransactionAndGetMainEvent<StartDKGFirstRoundEvent>(
		c,
		tx,
		isStartDKGFirstRoundEvent,
		'DKG first round failed',
	);

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
	return waitForCondition(
		conf,
		async () => {
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
			return null;
		},
		'timeout: unable to fetch the DWallet object',
	);
}

export async function acceptEncryptedUserShare(
	conf: Config,
	encryptedDWalletData: EncryptedDWalletData,
): Promise<void> {
	const dwallet = await getObjectWithType(conf, encryptedDWalletData.dwallet_id, isActiveDWallet);
	const dwalletOutput = dwallet.state.fields.public_output;
	const signedPublicOutput = await conf.encryptedSecretShareSigningKeypair.sign(
		new Uint8Array(dwalletOutput),
	);
	const tx = new Transaction();
	const dwalletStateArg = await getDWalletStateArg(conf, tx, true);
	const dwalletIDArg = tx.pure.id(encryptedDWalletData.dwallet_id);
	const encryptedUserSecretKeyShareIDArg = tx.pure.id(
		encryptedDWalletData.encrypted_user_secret_key_share_id,
	);
	const userOutputSignatureArg = tx.pure(bcs.vector(bcs.u8()).serialize(signedPublicOutput));
	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::accept_encrypted_user_share`,
		arguments: [
			dwalletStateArg,
			dwalletIDArg,
			encryptedUserSecretKeyShareIDArg,
			userOutputSignatureArg,
		],
	});
	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});
	if (result.events?.length === 0) {
		throw new Error('failed to accept encrypted user share');
	}
}
