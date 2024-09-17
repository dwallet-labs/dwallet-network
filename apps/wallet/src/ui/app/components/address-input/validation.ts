// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { usePeraNSEnabled } from '@mysten/core';
import { usePeraClient } from '@mysten/dapp-kit';
import { type PeraClient } from '@pera-io/pera/client';
import { isValidPeraAddress, isValidPeraNSName } from '@pera-io/pera/utils';
import { useMemo } from 'react';
import * as Yup from 'yup';

const CACHE_EXPIRY_TIME = 60 * 1000; // 1 minute in milliseconds

export function createPeraAddressValidation(client: PeraClient, peraNSEnabled: boolean) {
	const resolveCache = new Map<string, { valid: boolean; expiry: number }>();

	const currentTime = Date.now();
	return Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-pera-address', 'Invalid address. Please check again.', async (value) => {
			if (peraNSEnabled && isValidPeraNSName(value)) {
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

			return isValidPeraAddress(value);
		})
		.label("Recipient's address");
}

export function usePeraAddressValidation() {
	const client = usePeraClient();
	const peraNSEnabled = usePeraNSEnabled();

	return useMemo(() => {
		return createPeraAddressValidation(client, peraNSEnabled);
	}, [client, peraNSEnabled]);
}
