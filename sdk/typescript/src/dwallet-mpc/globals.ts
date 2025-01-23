// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { Buffer } from 'buffer';

import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

export const packageId = '0x3';
export const dWalletModuleName = 'dwallet';
export const dWalletPackageID = '0x3';
export const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';
const dwalletSecp256K1MoveType = `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::Secp256K1`;
export const dWalletMoveType = `${dWalletPackageID}::${dWalletModuleName}::DWallet<${dwalletSecp256K1MoveType}>`;
export const checkpointCreationTime = 2000;

export const mockedProtocolPublicParameters = Uint8Array.from(
	Buffer.from(
		'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',
		'base64',
	),
);

export interface Config {
	keypair: Keypair;
	client: PeraClient;
	timeout: number;
}

export enum MPCKeyScheme {
	Secp256k1 = 1,
	Ristretto = 2,
}

interface FetchObjectFromEventParams<TEvent, TObject> {
	conf: Config;
	eventType: string;
	objectType: string;
	isEvent: (event: any) => event is TEvent;
	isObject: (obj: any) => obj is TObject;
	filterEvent: (event: TEvent) => boolean;
	getObjectId: (event: TEvent) => string;
}

// The Move type.
export interface DWallet {
	id: { id: string };
	session_id: string;
	dwallet_cap_id: string;
	// The Decentralized Public output of the second DKG round.
	decentralized_public_output: number[];
	// The Centralized Public output of the centralized (user) DKG round.
	centralized_public_output: number[];
	// The MPC network decryption key version that is used to decrypt this dWallet.
	dwallet_mpc_network_decryption_key_version: number;
}

// dWallet with the private key share.
export interface dWalletWithSecretKeyShare extends DWallet {
	// The Centralized (user) Secret Key Share.
	// Warning:
	// The secret (private) key share returned from this function should never be sent,
	// and should always be kept private.
	centralizedSecretKeyShare: number[];
}

// export interface CreatedDwallet {
// 	id: string;
// 	// todo(scaly): is this the public key or only Xa ?
// 	centralizedDKGPublicOutput: number[];
// 	// The Centralized Secret Key Share.
// 	centralizedDKGPrivateOutput: number[];
// 	// todo(zeev): check if we even need this one.
// 	decentralizedDKGOutput: number[];
// 	dWalletCapID: string;
// 	dWalletMPCNetworkDecryptionKeyVersion: number;
// }

export async function fetchObjectFromEvent<TEvent, TObject>({
	conf,
	eventType,
	objectType,
	isEvent,
	isObject,
	filterEvent,
	getObjectId,
}: FetchObjectFromEventParams<TEvent, TObject>): Promise<TObject> {
	let cursor = null;
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		// Wait for 5 seconds between queries
		await delay(5000);

		// Query events with the current cursor.
		const {
			data: events,
			nextCursor,
			hasNextPage,
		} = await conf.client.queryEvents({
			cursor,
			query: {
				MoveEventType: eventType,
			},
		});

		for (const eventData of events) {
			// Validate and parse the event.
			const event = isEvent(eventData.parsedJson) ? (eventData.parsedJson as TEvent) : null;

			if (!event) {
				throw new Error(
					`Invalid event of type ${eventType}, got: ${JSON.stringify(eventData.parsedJson)}`,
				);
			}

			if (!filterEvent(event)) {
				console.log({ event });
				continue;
			}

			return fetchObjectWithType(conf, objectType, isObject, getObjectId(event));
		}
		cursor = hasNextPage ? nextCursor : null;
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch an event of type ${eventType} within ${
			conf.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

export async function fetchCompletedEvent<TEvent extends { session_id: string }>(
	c: Config,
	sessionID: string,
	eventType: string,
	isEventFn: (parsedJson: any) => parsedJson is TEvent,
): Promise<TEvent> {
	const startTime = Date.now();
	let cursor = null;

	while (Date.now() - startTime <= c.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await delay(5_000);

		const { data, nextCursor, hasNextPage } = await c.client.queryEvents({
			query: {
				TimeRange: {
					startTime: (Date.now() - c.timeout).toString(),
					endTime: Date.now().toString(),
				},
			},
			cursor,
		});

		const match = data.find(
			(event) =>
				event.type === eventType &&
				isEventFn(event.parsedJson) &&
				event.parsedJson.session_id === sessionID,
		);

		if (match) return match.parsedJson as TEvent;
		if (hasNextPage) cursor = nextCursor;
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch an event of type ${eventType} within ${
			c.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

export async function fetchObjectWithType<TObject>(
	conf: Config,
	objectType: string,
	isObject: (obj: any) => obj is TObject,
	objectId: string,
) {
	const res = await conf.client.getObject({
		id: objectId,
		options: { showContent: true },
	});

	const objectData =
		res.data?.content?.dataType === 'moveObject' &&
		res.data?.content.type === objectType &&
		isObject(res.data.content.fields)
			? (res.data.content.fields as TObject)
			: null;

	if (!objectData) {
		throw new Error(
			`invalid object of type ${objectType}, got: ${JSON.stringify(res.data?.content)}`,
		);
	}

	return objectData;
}

export function isDWallet(obj: any): obj is DWallet {
	return (
		obj &&
		'id' in obj &&
		'session_id' in obj &&
		'dwallet_cap_id' in obj &&
		'decentralized_output' in obj
	);
}

/**
 * Utility function to create a delay.
 */
export function delay(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export function isEqual(arr1: Uint8Array, arr2: Uint8Array): boolean {
	return arr1.length === arr2.length && arr1.every((value, index) => value === arr2[index]);
}
