// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { isSuiNSName, useSuiNSEnabled } from '@mysten/core';
import { useSuiClient } from '@mysten/dapp-kit';
import { type SuiClient } from '@mysten/sui.js/client';
import { isValidSuiAddress } from '@mysten/sui.js/utils';
import { useMemo } from 'react';
import * as Yup from 'yup';

export function createSuiAddressValidation(client: SuiClient, suiNSEnabled: boolean) {
	const resolveCache = new Map<string, boolean>();

	return Yup.string()
		.ensure()
		.trim()
		.required()
		.test('is-sui-address', 'Invalid address. Please check again.', async (value) => {
			if (suiNSEnabled && isSuiNSName(value)) {
				if (resolveCache.has(value)) {
					return resolveCache.get(value)!;
				}

				const address = await client.resolveNameServiceAddress({
					name: value,
				});

				resolveCache.set(value, !!address);

				return !!address;
			}

			return isValidSuiAddress(value);
		})
		.label("Recipient's address");
}

export function useSuiAddressValidation() {
	const client = useSuiClient();
	const suiNSEnabled = useSuiNSEnabled();

	return useMemo(() => {
		return createSuiAddressValidation(client, suiNSEnabled);
	}, [client, suiNSEnabled]);
}
