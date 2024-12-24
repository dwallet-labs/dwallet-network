// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

export const packageId = '0x3';
export const dWalletModuleName = 'dwallet';
export const dWalletPackageID = '0x3';
export const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export interface Config {
	keypair: Keypair;
	client: PeraClient;
	timeout: number;
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
		await new Promise((resolve) => setTimeout(resolve, 5_000));

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
