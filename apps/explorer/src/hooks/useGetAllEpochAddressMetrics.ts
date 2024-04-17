// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useSuiClient } from '@mysten/dapp-kit';
import { type SuiClient } from '@dwallet/dwallet.js/client';
import { useQuery } from '@tanstack/react-query';

export function useGetAllEpochAddressMetrics(
	...input: Parameters<SuiClient['getAllEpochAddressMetrics']>
) {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['get', 'all', 'epoch', 'addresses', ...input],
		queryFn: () => client.getAllEpochAddressMetrics(...input),
	});
}
