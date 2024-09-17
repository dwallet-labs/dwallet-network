// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { usePeraClient } from '@mysten/dapp-kit';
import { DynamicFieldPage } from '@pera-io/pera/client';
import { normalizePeraAddress } from '@pera-io/pera/utils';
import { useInfiniteQuery } from '@tanstack/react-query';

const MAX_PAGE_SIZE = 10;

export function useGetDynamicFields(parentId: string, maxPageSize = MAX_PAGE_SIZE) {
	const client = usePeraClient();
	return useInfiniteQuery<DynamicFieldPage>({
		queryKey: ['dynamic-fields', { maxPageSize, parentId }],
		queryFn: ({ pageParam = null }) =>
			client.getDynamicFields({
				parentId: normalizePeraAddress(parentId),
				cursor: pageParam as string | null,
				limit: maxPageSize,
			}),
		enabled: !!parentId,
		initialPageParam: null,
		getNextPageParam: ({ nextCursor, hasNextPage }) => (hasNextPage ? nextCursor : null),
	});
}
