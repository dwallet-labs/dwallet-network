// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { bcs } from '../bcs/index.js';
import type { MoveValue, PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';

export const packageId = '0x3';
export const dWalletModuleName = 'dwallet';
export const dWalletPackageID = '0x3';
export const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export interface Config {
	keypair: Keypair;
	client: PeraClient;
	timeout: number;
}

interface MoveObjectWithFields {
	fields: { [key: string]: MoveValue };
}

export async function fetchObjectBySessionId(sessionId: string, type: string, c: Config) {
	let cursor = null;
	const startTime = Date.now();

	while (Date.now() - startTime < c.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await new Promise((r) => setTimeout(r, 5000));
		const {
			data: ownedObjects,
			hasNextPage,
			nextCursor,
		} = await c.client.getOwnedObjects({
			owner: c.keypair.toPeraAddress(),
			cursor,
		});
		const objectIds = ownedObjects.map((o) => o.data?.objectId).filter(Boolean) as string[];

		if (objectIds.length === 0) {
			continue;
		}

		const objectsContent = await c.client.multiGetObjects({
			ids: objectIds,
			options: { showContent: true },
		});

		const match = objectsContent
			.map((o) => o.data?.content)
			.find(
				(o) =>
					o?.dataType === 'moveObject' &&
					o?.type === type &&
					(o as MoveObjectWithFields)?.fields?.['session_id'] === sessionId,
			);
		if (match) return match;
		if (hasNextPage) {
			cursor = nextCursor;
		}
	}
	throw new Error(`Timeout: Unable to fetch an object of type ${type} within ${c.timeout}ms`);
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
		await new Promise((resolve) => setTimeout(resolve, 5000));

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

			// Fetch the object based on the event
			const res = await conf.client.getObject({
				id: getObjectId(event),
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
		cursor = hasNextPage ? nextCursor : null;
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch an event of type ${eventType} within ${
			conf.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

export const approveMessages = async (
	client: PeraClient,
	keypair: Keypair,
	dwalletCapId: string,
	messages: Uint8Array[],
) => {
	const tx = new Transaction();
	const [messageApprovals] = tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::approve_messages`,
		arguments: [
			tx.object(dwalletCapId),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
		],
	});
	tx.transferObjects([messageApprovals], keypair.toPeraAddress());
	return await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
};
