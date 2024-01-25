// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useSuiClient } from '@mysten/dapp-kit';
import { useQuery } from '@tanstack/react-query';

export function useGetNetworkMetrics() {
	const client = useSuiClient();
	return useQuery({
		queryKey: ['home', 'metrics'],
		queryFn: () => client.getNetworkMetrics(),
		gcTime: 24 * 60 * 60 * 1000,
		staleTime: Infinity,
		retry: 5,
	});
}
