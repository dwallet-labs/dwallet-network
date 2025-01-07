// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { create_dkg_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletModuleName,
	dWalletPackageID,
	fetchCompletedEvent,
	fetchObjectFromEvent,
	MPCKeyScheme,
	packageId,
} from './globals.js';

const dwalletSecp256K1MoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::Secp256K1`;
export const dWalletMoveType = `${dWalletPackageID}::${dWalletModuleName}::DWallet<${dwalletSecp256K1MoveType}>`;
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

// The Move type.
export interface DWallet {
	id: { id: string };
	session_id: string;
	dwallet_cap_id: string;
	output: number[];
	dwallet_mpc_network_key_version: number;
}

export interface CreatedDwallet {
	id: string;
	centralizedDKGPublicOutput: number[];
	centralizedDKGPrivateOutput: number[];
	decentralizedDKGOutput: number[];
	dwalletCapID: string;
	dwalletMPCNetworkKeyVersion: number;
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
): Promise<CreatedDwallet> {
	const dkgFirstRoundOutput: DKGFirstRoundOutput = await launchDKGFirstRound(conf);
	console.log('DKG First Round Output:', dkgFirstRoundOutput);
	let [publicKeyShareAndProof, centralizedPublicOutput, centralizedPrivateOutput] =
		create_dkg_centralized_output(
			// Todo (#382): Pass the actual chain's public parameters.
			// Right now we pass an empty array, and the wasm
			// function behind the scenes uses the default, mock public parameters.
			// Can't be an empty array as it makes the wasm crash for some reason
			protocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Uint8Array.from(dkgFirstRoundOutput.output),
			// Remove the 0x prefix.
			dkgFirstRoundOutput.session_id.slice(2),
		);
	let dwallet = await launchDKGSecondRound(conf, dkgFirstRoundOutput, publicKeyShareAndProof);

	return {
		id: dwallet.id.id,
		centralizedDKGPublicOutput: centralizedPublicOutput,
		centralizedDKGPrivateOutput: centralizedPrivateOutput,
		decentralizedDKGOutput: dwallet.output,
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
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::launch_dkg_second_round`,
		arguments: [
			tx.object(firstRound.dwallet_cap_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(publicKeyShareAndProof)),
			tx.object(firstRound.output_object_id),
			tx.pure.id(firstRound.session_id),
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

export function isDWallet(obj: any): obj is DWallet {
	return obj && 'id' in obj && 'session_id' in obj && 'dwallet_cap_id' in obj && 'output' in obj;
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
