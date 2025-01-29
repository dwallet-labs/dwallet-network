// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import {
	create_dkg_centralized_output,
	encrypt_secret_share,
} from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import { PERA_SYSTEM_STATE_OBJECT_ID } from '../utils/index.js';
import { EncryptedUserShare } from './encrypt-user-share.js';
import type { Config, CreatedDwallet, DWallet } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletMoveType,
	dWalletPackageID,
	fetchCompletedEvent,
	fetchObjectFromEvent,
	isDWallet,
	MPCKeyScheme,
	packageId,
} from './globals.js';

const completedDKGSecondRoundEventMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedDKGSecondRoundEvent`;
const startDKGFirstRoundEventMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::StartDKGFirstRoundEvent`;
const dkgFirstRoundOutputEvent = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::DKGFirstRoundOutputEvent`;

interface CompletedDKGSecondRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_cap_id: string;
	dwallet_id: string;
	value: number[];
}

interface StartDKGFirstRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_cap_id: string;
}

interface DKGFirstRoundOutputEvent {
	output: number[];
	session_id: string;
	output_object_id: string;
}

interface DKGFirstRoundOutput extends DKGFirstRoundOutputEvent {
	dwallet_cap_id: string;
}

export async function createDWallet(
	conf: Config,
	protocolPublicParameters: Uint8Array,
	activeEncryptionKeyTableID: string,
): Promise<CreatedDwallet> {
	const dkgFirstRoundOutput = await launchDKGFirstRound(conf);
	// todo(scaly): need to clarify here.
	const [centralizedPublicKeyShareAndProof, centralizedPublicOutput, centralizedPrivateKeyShare] =
		create_dkg_centralized_output(
			protocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dkgFirstRoundOutput.output),
			// Remove the 0x prefix.
			dkgFirstRoundOutput.session_id.slice(2),
		);

	// Encrypt the dWallet secret share to use it later by only
	// holding our Ika ed25519 keypair (the encryption key is derived from Ika keypair).
	const encryptedUserShare = EncryptedUserShare.fromConfig(conf);
	const derivedClassGroupsKeyPair = await encryptedUserShare.getOrCreateClassGroupsKeyPair(
		conf.keypair,
		activeEncryptionKeyTableID,
	);
	const encryptedUserKeyShareAndProofOfEncryption = encrypt_secret_share(
		new Uint8Array(centralizedPrivateKeyShare),
		new Uint8Array(derivedClassGroupsKeyPair.encryptionKey),
	);
	const signedCentralizedPublicOutput = await conf.keypair.sign(
		new Uint8Array(centralizedPublicOutput),
	);
	const dwallet = await launchDKGSecondRound(
		conf,
		dkgFirstRoundOutput,
		centralizedPublicKeyShareAndProof,
		encryptedUserKeyShareAndProofOfEncryption,
		derivedClassGroupsKeyPair.objectID,
		signedCentralizedPublicOutput,
		conf.keypair.getPublicKey().toRawBytes(),
		centralizedPublicOutput,
	);

	return {
		id: dwallet.id.id,
		centralizedDKGPublicOutput: centralizedPublicOutput,
		centralizedDKGPrivateOutput: centralizedPrivateKeyShare,
		decentralizedDKGOutput: dwallet.decentralized_output,
		dwalletCapID: dwallet.dwallet_cap_id,
		dwalletMPCNetworkKeyVersion: dwallet.dwallet_mpc_network_key_version,
	};
}

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round.
 */
async function launchDKGFirstRound(c: Config) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_first_round`,
		arguments: [
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: false,
			}),
		],
	});
	const result = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	const sessionData = result.events?.find(
		(event) =>
			event.type === startDKGFirstRoundEventMoveType && isStartDKGFirstRoundEvent(event.parsedJson),
	)?.parsedJson as StartDKGFirstRoundEvent;
	const completionEvent = await fetchCompletedEvent<DKGFirstRoundOutputEvent>(
		c,
		sessionData.session_id,
		dkgFirstRoundOutputEvent,
		isDKGFirstRoundOutputEvent,
	);
	return {
		...completionEvent,
		dwallet_cap_id: sessionData.dwallet_cap_id,
	};
}

async function launchDKGSecondRound(
	c: Config,
	firstRoundOutput: DKGFirstRoundOutput,
	centralizedPublicKeyShareAndProof: Uint8Array,
	encryptedSecretShareAndProof: Uint8Array,
	encryptionKeyID: string,
	signedCentralizedPublicOutput: Uint8Array,
	srcIkaPubkey: Uint8Array,
	centralizedPublicOutput: Uint8Array,
) {
	const tx = new Transaction();

	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.object(firstRoundOutput.dwallet_cap_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicKeyShareAndProof)),
			tx.object(firstRoundOutput.output_object_id),
			tx.pure.id(firstRoundOutput.session_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptedSecretShareAndProof)),
			tx.object(encryptionKeyID),
			tx.pure(bcs.vector(bcs.u8()).serialize(signedCentralizedPublicOutput)),
			tx.pure(bcs.vector(bcs.u8()).serialize(srcIkaPubkey)),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicOutput)),
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: false,
			}),
		],
	});

	await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	return await dWalletFromEvent(c, firstRoundOutput);
}

async function dWalletFromEvent(conf: Config, firstRound: DKGFirstRoundOutput): Promise<DWallet> {
	function isCompletedDKGSecondRoundEvent(event: any): event is CompletedDKGSecondRoundEvent {
		return (
			event &&
			event.session_id &&
			event.initiator &&
			event.dwallet_cap_id &&
			event.dwallet_id &&
			Array.isArray(event.value)
		);
	}

	return fetchObjectFromEvent<CompletedDKGSecondRoundEvent, DWallet>({
		conf,
		eventType: completedDKGSecondRoundEventMoveType,
		objectType: dWalletMoveType,
		isEvent: isCompletedDKGSecondRoundEvent,
		isObject: isDWallet,
		filterEvent: (event) => event.dwallet_cap_id === firstRound.dwallet_cap_id,
		getObjectId: (event) => event.dwallet_id,
	});
}

function isStartDKGFirstRoundEvent(obj: any): obj is StartDKGFirstRoundEvent {
	return obj && 'session_id' in obj && 'initiator' in obj && 'dwallet_cap_id' in obj;
}

function isDKGFirstRoundOutputEvent(obj: any): obj is DKGFirstRoundOutputEvent {
	return 'output' in obj && 'session_id' in obj && 'output_object_id' in obj;
}
