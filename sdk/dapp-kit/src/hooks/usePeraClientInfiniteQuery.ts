// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraClient } from '@pera-io/pera/client';
import type {
	InfiniteData,
	UseInfiniteQueryOptions,
	UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { PartialBy } from '../types/utilityTypes.js';
import { usePeraClientContext } from './usePeraClient.js';

interface PaginatedResult {
	data?: unknown;
	nextCursor?: unknown;
	hasNextPage: boolean;
}

export type PeraRpcPaginatedMethodName = {
	[K in keyof PeraClient]: PeraClient[K] extends (input: any) => Promise<PaginatedResult> ? K : never;
}[keyof PeraClient];

export type PeraRpcPaginatedMethods = {
	[K in PeraRpcPaginatedMethodName]: PeraClient[K] extends (
		input: infer Params,
	) => Promise<
		infer Result extends { hasNextPage?: boolean | null; nextCursor?: infer Cursor | null }
	>
		? {
				name: K;
				result: Result;
				params: Params;
				cursor: Cursor;
			}
		: never;
};

export type UsePeraClientInfiniteQueryOptions<
	T extends keyof PeraRpcPaginatedMethods,
	TData,
> = PartialBy<
	Omit<
		UseInfiniteQueryOptions<
			PeraRpcPaginatedMethods[T]['result'],
			Error,
			TData,
			PeraRpcPaginatedMethods[T]['result'],
			unknown[]
		>,
		'queryFn' | 'initialPageParam' | 'getNextPageParam'
	>,
	'queryKey'
>;

export function usePeraClientInfiniteQuery<
	T extends keyof PeraRpcPaginatedMethods,
	TData = InfiniteData<PeraRpcPaginatedMethods[T]['result']>,
>(
	method: T,
	params: PeraRpcPaginatedMethods[T]['params'],
	{
		queryKey = [],
		enabled = !!params,
		...options
	}: UsePeraClientInfiniteQueryOptions<T, TData> = {},
): UseInfiniteQueryResult<TData, Error> {
	const peraContext = usePeraClientContext();

	return useInfiniteQuery({
		...options,
		initialPageParam: null,
		queryKey: [peraContext.network, method, params, ...queryKey],
		enabled,
		queryFn: ({ pageParam }) =>
			peraContext.client[method]({
				...(params ?? {}),
				cursor: pageParam,
			} as never),
		getNextPageParam: (lastPage) => (lastPage.hasNextPage ? lastPage.nextCursor ?? null : null),
	});
}
