// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { usePeraClient } from '@mysten/dapp-kit';
import { useQuery } from '@tanstack/react-query';

const PERA_NS_FEATURE_FLAG = 'perans';

// This should align with whatever names we want to be able to resolve.

export function usePeraNSEnabled() {
	return useFeatureIsOn(PERA_NS_FEATURE_FLAG);
}

export function useResolvePeraNSAddress(name?: string | null, enabled?: boolean) {
	const client = usePeraClient();
	const enabledPeraNs = usePeraNSEnabled();

	return useQuery({
		queryKey: ['resolve-perans-address', name],
		queryFn: async () => {
			return await client.resolveNameServiceAddress({
				name: name!,
			});
		},
		enabled: !!name && enabled && enabledPeraNs,
		refetchOnWindowFocus: false,
		retry: false,
	});
}

export function useResolvePeraNSName(address?: string | null) {
	const client = usePeraClient();
	const enabled = usePeraNSEnabled();

	return useQuery({
		queryKey: ['resolve-perans-name', address],
		queryFn: async () => {
			// NOTE: We only fetch 1 here because it's the default name.
			const { data } = await client.resolveNameServiceNames({
				address: address!,
				limit: 1,
			});

			return data[0] || null;
		},
		enabled: !!address && enabled,
		refetchOnWindowFocus: false,
		retry: false,
	});
}
