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
import { PERA_SYSTEM_STATE_OBJECT_ID } from '../utils/index.js';
import { EncryptedUserShare } from './encrypt-user-share.js';
import type { Config, DWallet, DWalletWithSecretKeyShare } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletMoveType,
	dWalletPackageID,
	fetchCompletedEvent,
	fetchObjectFromEvent,
	isDWallet,
	MPCKeyScheme,
} from './globals.js';

const completedDKGSecondRoundEventMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedDKGSecondRoundEvent`;
const startDKGFirstRoundEventMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::StartDKGFirstRoundEvent`;
const dkgFirstRoundOutputEventMoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::DKGFirstRoundOutputEvent`;

/**
 * Event emitted to start the first round of the DKG process.
 *
 * This event is captured by the blockchain, which uses it to
 * initiate the first round of the DKG.
 */
interface StartDKGFirstRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_cap_id: string;
}

function isStartDKGFirstRoundEvent(obj: any): obj is StartDKGFirstRoundEvent {
	return (
		typeof obj === 'object' &&
		obj !== null &&
		typeof obj.session_id === 'string' &&
		typeof obj.initiator === 'string' &&
		typeof obj.dwallet_cap_id === 'string'
	);
}

/**
 * Event emitted upon the completing the second (and final) round of the
 * Distributed Key Generation (DKG).
 *
 * This event provides all necessary data generated from the second
 * round of the DKG process.
 * Emitted to notify the centralized party.
 */
interface CompletedDKGSecondRoundEvent {
	// A unique identifier for the DKG session, linking all related events and actions.
	session_id: string;

	// The address of the user who initiated the DKG process.
	initiator: string;

	// The unique identifier of the dWallet capability associated with the session.
	dwallet_cap_id: string;

	// The ID of the dWallet created as a result of the DKG process.
	dwallet_id: string;

	// The public decentralized output for the second round of the DKG process.
	decentralized_public_output: number[];
}

/**
 * An event emitted when the first round of the DKG process is completed.
 * This event is emitted by the blockchain to notify the user about
 * the completion of the first round.
 * The user should catch this event to generate inputs for
 * the second round and call the `launch_dkg_second_round()` function.
 */
interface DKGFirstRoundOutputEvent {
	decentralized_public_output: number[];
	session_id: string;
	output_object_id: string;
}

interface DKGFirstRoundResult extends DKGFirstRoundOutputEvent {
	dwallet_cap_id: string;
}

function isDKGFirstRoundOutputEvent(obj: any): obj is DKGFirstRoundOutputEvent {
	return (
		typeof obj === 'object' &&
		obj !== null &&
		typeof obj.session_id === 'string' &&
		typeof obj.output_object_id === 'string' &&
		Array.isArray(obj.decentralized_public_output)
	);
}

export async function createDWallet(
	c: Config,
	protocolPublicParameters: Uint8Array,
	activeEncryptionKeyTableID: string,
): Promise<DWalletWithSecretKeyShare> {
	const dkgFirstRoundResult = await launchDKGFirstRound(c);
	// centralizedPublicOutput: centralized_public_key_share + public_key + decentralized_party_public_key_share.
	const [centralizedPublicKeyShareAndProof, centralizedPublicOutput, centralizedSecretKeyShare, serializedPublicKeys] =
		create_dkg_centralized_output(
			protocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dkgFirstRoundResult.decentralized_public_output),
			// Remove the 0x prefix.
			dkgFirstRoundResult.session_id.slice(2),
		);

	// Encrypt the dWallet secret share to use it later by only
	// holding our Ika ED25519 keypair (the encryption key is derived from the Ika keypair).
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
		dkgFirstRoundResult,
		centralizedPublicKeyShareAndProof,
		encryptedCentralizedSecretKeyShareAndProofOfEncryption,
		derivedClassGroupsKeyPair.objectID,
		centralizedPublicOutput,
		serializedPublicKeys,
		c.keypair.getPublicKey(),
	);

	return {
		centralizedSecretKeyShare: centralizedSecretKeyShare,
		...dwallet,
	};
}

/**
 * Starts the first round of the DKG protocol to create a new dWallet.
 * The output of this function is being used to generate the input for the second round,
 * and as input for the centralized party round.
 */
async function launchDKGFirstRound(c: Config): Promise<DKGFirstRoundResult> {
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
	const startDKGFirstRoundEvent = result.events?.find(
		(event) =>
			event.type === startDKGFirstRoundEventMoveType && isStartDKGFirstRoundEvent(event.parsedJson),
	)?.parsedJson as StartDKGFirstRoundEvent;
	const dkgFirstRoundOutputEvent = await fetchCompletedEvent<DKGFirstRoundOutputEvent>(
		c,
		startDKGFirstRoundEvent.session_id,
		dkgFirstRoundOutputEventMoveType,
		isDKGFirstRoundOutputEvent,
	);
	return {
		...dkgFirstRoundOutputEvent,
		dwallet_cap_id: startDKGFirstRoundEvent.dwallet_cap_id,
	};
}

async function launchDKGSecondRound(
	c: Config,
	firstRoundResult: DKGFirstRoundResult,
	centralizedPublicKeyShareAndProof: Uint8Array,
	encryptedCentralizedSecretShareAndProof: Uint8Array,
	encryptionKeyID: string,
	centralizedPublicOutput: Uint8Array,
	public_keys: number[],
	initiatorPubKey: PublicKey,
) {
	const signedPublicKeys = await c.keypair.sign(new Uint8Array(public_keys));
	const tx  = new Transaction();

	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.object(firstRoundResult.dwallet_cap_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicKeyShareAndProof)),
			tx.object(firstRoundResult.output_object_id),
			tx.pure.id(firstRoundResult.session_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptedCentralizedSecretShareAndProof)),
			tx.object(encryptionKeyID),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicOutput)),
			tx.pure(bcs.vector(bcs.u8()).serialize(signedPublicKeys)),
			tx.pure(bcs.vector(bcs.u8()).serialize(initiatorPubKey.toRawBytes())),
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
	return await dWalletFromEvent(c, firstRoundResult);
}

async function dWalletFromEvent(conf: Config, firstRound: DKGFirstRoundResult): Promise<DWallet> {
	function isCompletedDKGSecondRoundEvent(event: any): event is CompletedDKGSecondRoundEvent {
		return (
			event &&
			event.session_id &&
			event.initiator &&
			event.dwallet_cap_id &&
			event.dwallet_id &&
			Array.isArray(event.decentralized_public_output)
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
