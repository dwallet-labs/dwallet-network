// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import {
	create_dkg_centralized_output,
	encrypt_secret_share,
} from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import type { PublicKey } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';
import { EncryptedUserShare } from './encrypt-user-share.js';
import type { Config, DWallet, dWalletWithSecretKeyShare } from './globals.js';
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

// todo(zeev): doc.

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

/**
 * An event emitted when the first round of the DKG process is completed.
 * This event is emitted by the blockchain to notify the user about
 * the completion of the first round.
 * The user should catch this event to generate inputs for
 * the second round and call the `launch_dkg_second_round()` function.
 */
interface DKGFirstRoundOutputEvent {
	output: number[];
	session_id: string;
	output_object_id: string;
}

interface DKGFirstRoundOutput extends DKGFirstRoundOutputEvent {
	dwallet_cap_id: string;
}

export async function createDWallet(
	c: Config,
	protocolPublicParameters: Uint8Array,
	activeEncryptionKeyTableID: string,
): Promise<dWalletWithSecretKeyShare> {
	const dkgFirstRoundOutput = await launchDKGFirstRound(c);
	// centralizedPublicOutput: centralized_public_key_share + public_key + decentralized_party_public_key_share.
	const [centralizedPublicKeyShareAndProof, centralizedPublicOutput, centralizedSecretKeyShare] =
		create_dkg_centralized_output(
			protocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dkgFirstRoundOutput.output),
			// Remove the 0x prefix.
			dkgFirstRoundOutput.session_id.slice(2),
		);

	// Encrypt the dWallet secret share to use it later by only
	// holding our Ika ed25519 keypair (the encryption key is derived from Ika keypair).
	const encryptedUserShare = EncryptedUserShare.fromConfig(c);
	const derivedClassGroupsKeyPair = await encryptedUserShare.getOrCreateClassGroupsKeyPair(
		c.keypair,
		activeEncryptionKeyTableID,
	);
	const encryptedCentralizedSecretKeyShareAndProofOfEncryption = encrypt_secret_share(
		new Uint8Array(centralizedSecretKeyShare),
		new Uint8Array(derivedClassGroupsKeyPair.encryptionKey),
	);
	const dwallet = await launchDKGSecondRound(
		c,
		dkgFirstRoundOutput,
		centralizedPublicKeyShareAndProof,
		encryptedCentralizedSecretKeyShareAndProofOfEncryption,
		derivedClassGroupsKeyPair.objectID,
		centralizedPublicOutput,
		c.keypair.getPublicKey(),
	);

	return {
		centralizedSecretKeyShare: centralizedSecretKeyShare,
		...dwallet,
		// todo(zeev): remove the following lines
		// id: dwallet.id.id,
		// centralizedDKGPublicOutput: centralizedPublicOutput,
		// centralizedDKGPrivateOutput: centralizedSecretKeyShare,
		// decentralizedDKGOutput: dwallet.decentralized_output,
		// dWalletCapID: dwallet.dwallet_cap_id,
		// dWalletMPCNetworkDecryptionKeyVersion: dwallet.dwallet_mpc_network_decryption_key_version,
	};
}

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round,
 * and as input for the centralized party round.
 */
async function launchDKGFirstRound(c: Config) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_first_round`,
		arguments: [],
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
	encryptedCentralizedSecretShareAndProof: Uint8Array,
	encryptionKeyID: string,
	centralizedPublicOutput: Uint8Array,
	initiatorPubKey: PublicKey,
) {
	const centralizedPublicOutputSignature = await c.keypair.sign(
		new Uint8Array(centralizedPublicOutput),
	);
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.object(firstRoundOutput.dwallet_cap_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicKeyShareAndProof)),
			tx.object(firstRoundOutput.output_object_id),
			tx.pure.id(firstRoundOutput.session_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptedCentralizedSecretShareAndProof)),
			tx.object(encryptionKeyID),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicOutput)),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicOutputSignature)),
			tx.pure(bcs.vector(bcs.u8()).serialize(initiatorPubKey.toRawBytes())),
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
