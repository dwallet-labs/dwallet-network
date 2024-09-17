// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { usePeraClient } from '@mysten/dapp-kit';
import { PaginatedObjectsResponse, type PeraObjectDataFilter } from '@pera-io/pera/client';
import { useInfiniteQuery } from '@tanstack/react-query';

const MAX_OBJECTS_PER_REQ = 6;

export function useGetOwnedObjects(
	address?: string | null,
	filter?: PeraObjectDataFilter,
	maxObjectRequests = MAX_OBJECTS_PER_REQ,
) {
	const client = usePeraClient();
	return useInfiniteQuery<PaginatedObjectsResponse>({
		initialPageParam: null,
		queryKey: ['get-owned-objects', address, filter, maxObjectRequests],
		queryFn: ({ pageParam }) =>
			client.getOwnedObjects({
				owner: address!,
				filter,
				options: {
					showType: true,
					showContent: true,
					showDisplay: true,
				},
				limit: maxObjectRequests,
				cursor: pageParam as string | null,
			}),

		staleTime: 10 * 1000,
		enabled: !!address,
		getNextPageParam: ({ hasNextPage, nextCursor }) => (hasNextPage ? nextCursor : null),
	});
}
