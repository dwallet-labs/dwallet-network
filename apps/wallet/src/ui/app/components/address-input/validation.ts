// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useIkaNSEnabled } from '@mysten/core';
import { useIkaClient } from '@mysten/dapp-kit';
import { type IkaClient } from '@ika-io/ika/client';
import { isValidIkaAddress, isValidIkaNSName } from '@ika-io/ika/utils';
import { useMemo } from 'react';
import * as Yup from 'yup';

const CACHE_EXPIRY_TIME = 60 * 1000; // 1 minute in milliseconds

export function createIkaAddressValidation(client: IkaClient, ikaNSEnabled: boolean) {
	const resolveCache = new Map<string, { valid: boolean; expiry: number }>();

	const currentTime = Date.now();
	return Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-ika-address', 'Invalid address. Please check again.', async (value) => {
			if (ikaNSEnabled && isValidIkaNSName(value)) {
				if (resolveCache.has(value)) {
					const cachedEntry = resolveCache.get(value)!;
					if (currentTime < cachedEntry.expiry) {
						return cachedEntry.valid;
					} else {
						resolveCache.delete(value); // Remove expired entry
					}
				}

				const address = await client.resolveNameServiceAddress({
					name: value,
				});

				resolveCache.set(value, {
					valid: !!address,
					expiry: currentTime + CACHE_EXPIRY_TIME,
				});

				return !!address;
			}

			return isValidIkaAddress(value);
		})
		.label("Recipient's address");
}

export function useIkaAddressValidation() {
	const client = useIkaClient();
	const ikaNSEnabled = useIkaNSEnabled();

	return useMemo(() => {
		return createIkaAddressValidation(client, ikaNSEnabled);
	}, [client, ikaNSEnabled]);
}
