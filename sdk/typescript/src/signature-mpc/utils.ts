// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

export async function fetchObjectBySessionId(
	sessionId: string,
	type: string,
	keypair: Keypair,
	client: DWalletClient,
) {
	let cursor = null;
	for (;;) {
		const objects = await client.getOwnedObjects({ owner: keypair.toSuiAddress(), cursor: cursor });
		const objectsContent = await client.multiGetObjects({
			ids: objects.data.map((o) => o.data?.objectId!),
			options: { showContent: true },
		});

		const objectsFiltered = objectsContent
			.map((o) => o.data?.content)
			.filter((o) => {
				return (
					// @ts-ignore
					o?.dataType == 'moveObject' && o?.type == type && o.fields['session_id'] == sessionId
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
