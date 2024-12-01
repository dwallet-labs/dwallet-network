import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

export const packageId = '0x3';
export const dWalletModuleName = 'dwallet';
export const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export async function fetchObjectBySessionId(
	sessionId: string,
	type: string,
	keypair: Keypair,
	client: PeraClient,
) {
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
