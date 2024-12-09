import type { MoveValue, PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

export const dWalletPackageID = '0x3';
export const dWalletModuleName = 'dwallet';
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
	const timeout = 15 * 60 * 1000; // 15 minutes in milliseconds
	const startTime = Date.now();

	for (;;) {
		if (Date.now() - startTime > timeout) {
			throw new Error('Timeout: Unable to fetch object within 5 minutes');
		}

		const objects = await client.getOwnedObjects({
			owner: keypair.toPeraAddress(),
			cursor: cursor,
		});
		const objectsContent = await client.multiGetObjects({
			ids: objects.data.map((o) => o.data?.objectId!),
			options: { showContent: true },
		});

		const objectsFiltered = objectsContent
			.map((o) => o.data?.content)
			.filter((o) => {
				return (
					o?.dataType === 'moveObject' &&
					o?.type === type &&
					// @ts-ignore
					o.fields['session_id'] === sessionId
				);
			});
		if (objectsFiltered.length > 0) {
			return objectsFiltered[0];
		} else if (objects.hasNextPage) {
			cursor = objects.nextCursor;
		} else {
			cursor = null;
		}
		await new Promise((r) => setTimeout(r, 500));
	}
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


export const getEventByTypeAndSessionId = async (
	client: PeraClient,
	eventType: string,
	session_id: string,
) => {
	const tenMinutesInMillis = 10 * 60 * 1000;
	const startTime = Date.now();

	for (;;) {
		if (Date.now() - startTime > tenMinutesInMillis) {
			throw new Error('Timeout: Unable to fetch object within 10 minutes');
		}
		await new Promise((resolve) => setTimeout(resolve, 5_000));
		let newEvents = await client.queryEvents({
			query: {
				TimeRange: {
					startTime: (Date.now() - tenMinutesInMillis).toString(),
					endTime: Date.now().toString(),
				},
			},
		});
		let matchingEvent = newEvents.data.find(
			(event) =>
				(
					event.parsedJson as {
						session_id: string;
					}
				).session_id === session_id && event.type === eventType,
		);
		if (matchingEvent) {
			return matchingEvent.parsedJson;
		}
	}
};

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
