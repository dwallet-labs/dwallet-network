// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useIkaClient } from '@mysten/dapp-kit';
import { PaginatedCoins } from '@ika-io/ika/client';
import { useInfiniteQuery } from '@tanstack/react-query';

const MAX_COINS_PER_REQUEST = 10;

export function useGetCoins(
	coinType: string,
	address?: string | null,
	maxCoinsPerRequest = MAX_COINS_PER_REQUEST,
) {
	const client = useIkaClient();
	return useInfiniteQuery<PaginatedCoins>({
		queryKey: ['get-coins', address, coinType, maxCoinsPerRequest],
		initialPageParam: null,
		getNextPageParam: ({ hasNextPage, nextCursor }) => (hasNextPage ? nextCursor : null),
		queryFn: ({ pageParam }) =>
			client.getCoins({
				owner: address!,
				coinType,
				cursor: pageParam as string | null,
				limit: maxCoinsPerRequest,
			}),
		enabled: !!address,
	});
}
