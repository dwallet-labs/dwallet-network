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
import { getOrCreateEncryptionKey } from './encrypt-user-share.js';
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
	let [publicKeyShareAndProof, centralizedPublicOutput, centralizedPrivateOutput] =
		create_dkg_centralized_output(
			protocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dkgFirstRoundOutput.output),
			// Remove the 0x prefix.
			dkgFirstRoundOutput.session_id.slice(2),
		);
	let encryptionKey = await getOrCreateEncryptionKey(conf, activeEncryptionKeyTableID);
	let encryptedUserShareAndProof = encrypt_secret_share(
		new Uint8Array(centralizedPrivateOutput),
		new Uint8Array(encryptionKey.encryptionKey),
	);
	let signedPublicShare = await conf.keypair.sign(new Uint8Array(centralizedPublicOutput));

	let dwallet = await launchDKGSecondRound(
		conf,
		dkgFirstRoundOutput,
		publicKeyShareAndProof,
		encryptedUserShareAndProof,
		encryptionKey.objectID,
		signedPublicShare,
		conf.keypair.getPublicKey().toRawBytes(),
		centralizedPublicOutput,
	);

	return {
		id: dwallet.id.id,
		centralizedDKGPublicOutput: centralizedPublicOutput,
		centralizedDKGPrivateOutput: centralizedPrivateOutput,
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
				mutable: true,
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
	let sessionData = result.events?.find(
		(event) =>
			event.type === startDKGFirstRoundEventMoveType && isStartDKGFirstRoundEvent(event.parsedJson),
	)?.parsedJson as StartDKGFirstRoundEvent;
	let completionEvent = await fetchCompletedEvent<DKGFirstRoundOutputEvent>(
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
	firstRound: DKGFirstRoundOutput,
	publicKeyShareAndProof: Uint8Array,
	encrypted_secret_share_and_proof: Uint8Array,
	encryption_key_id: string,
	signed_public_share: Uint8Array,
	encryptor_ed25519_pubkey: Uint8Array,
	centralizedPublicOutput: Uint8Array,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.object(firstRound.dwallet_cap_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(publicKeyShareAndProof)),
			tx.object(firstRound.output_object_id),
			tx.pure.id(firstRound.session_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(encrypted_secret_share_and_proof)),
			tx.object(encryption_key_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(signed_public_share)),
			tx.pure(bcs.vector(bcs.u8()).serialize(encryptor_ed25519_pubkey)),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedPublicOutput)),
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
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
	return await dWalletFromEvent(c, firstRound);
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
