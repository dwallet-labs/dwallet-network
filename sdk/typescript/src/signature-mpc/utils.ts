// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { setTimeout } from 'timers/promises';

import type { DWalletClient } from '../client';
import type { Keypair } from '../cryptography';

export async function fetchObjectBySessionId(
	sessionId: string,
	type: string,
	keypair: Keypair,
	client: DWalletClient,
) {
	let cursor = null;
	while (true) {
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
		await setTimeout(500);
	}
}
