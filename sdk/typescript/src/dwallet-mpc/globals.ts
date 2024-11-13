import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

export const packageId = '0x3';
export const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export async function fetchObjectBySessionId(
	sessionId: string,
	type: string,
	keypair: Keypair,
	client: PeraClient,
) {
	let cursor = null;
	const timeout = 5 * 60 * 1000; // 5 minutes in milliseconds
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
