// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useIkaClient } from '@mysten/dapp-kit';
import { PaginatedObjectsResponse, type IkaObjectDataFilter } from '@ika-io/ika/client';
import { useInfiniteQuery } from '@tanstack/react-query';

const MAX_OBJECTS_PER_REQ = 6;

export function useGetOwnedObjects(
	address?: string | null,
	filter?: IkaObjectDataFilter,
	maxObjectRequests = MAX_OBJECTS_PER_REQ,
) {
	const client = useIkaClient();
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
