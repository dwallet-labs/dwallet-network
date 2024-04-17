// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useSuiClient } from '@mysten/dapp-kit';
import { DynamicFieldPage } from '@dwallet/dwallet.js/client';
import { normalizeSuiAddress } from '@dwallet/dwallet.js/utils';
import { useInfiniteQuery } from '@tanstack/react-query';

const MAX_PAGE_SIZE = 10;

export function useGetDynamicFields(parentId: string, maxPageSize = MAX_PAGE_SIZE) {
	const client = useSuiClient();
	return useInfiniteQuery<DynamicFieldPage>({
		queryKey: ['dynamic-fields', { maxPageSize, parentId }],
		queryFn: ({ pageParam = null }) =>
			client.getDynamicFields({
				parentId: normalizeSuiAddress(parentId),
				cursor: pageParam as string | null,
				limit: maxPageSize,
			}),
		enabled: !!parentId,
		initialPageParam: null,
		getNextPageParam: ({ nextCursor, hasNextPage }) => (hasNextPage ? nextCursor : null),
	});
}
