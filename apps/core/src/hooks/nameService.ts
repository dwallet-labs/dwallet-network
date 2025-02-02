// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { useIkaClient } from '@mysten/dapp-kit';
import { useQuery } from '@tanstack/react-query';

const IKA_NS_FEATURE_FLAG = 'ikans';

// This should align with whatever names we want to be able to resolve.

export function useIkaNSEnabled() {
	return useFeatureIsOn(IKA_NS_FEATURE_FLAG);
}

export function useResolveIkaNSAddress(name?: string | null, enabled?: boolean) {
	const client = useIkaClient();
	const enabledIkaNs = useIkaNSEnabled();

	return useQuery({
		queryKey: ['resolve-ikans-address', name],
		queryFn: async () => {
			return await client.resolveNameServiceAddress({
				name: name!,
			});
		},
		enabled: !!name && enabled && enabledIkaNs,
		refetchOnWindowFocus: false,
		retry: false,
	});
}

export function useResolveIkaNSName(address?: string | null) {
	const client = useIkaClient();
	const enabled = useIkaNSEnabled();

	return useQuery({
		queryKey: ['resolve-ikans-name', address],
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
