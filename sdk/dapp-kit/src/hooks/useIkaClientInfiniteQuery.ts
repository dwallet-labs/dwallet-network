// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaClient } from '@ika-io/ika/client';
import type {
	InfiniteData,
	UseInfiniteQueryOptions,
	UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { PartialBy } from '../types/utilityTypes.js';
import { useIkaClientContext } from './useIkaClient.js';

interface PaginatedResult {
	data?: unknown;
	nextCursor?: unknown;
	hasNextPage: boolean;
}

export type IkaRpcPaginatedMethodName = {
	[K in keyof IkaClient]: IkaClient[K] extends (input: any) => Promise<PaginatedResult> ? K : never;
}[keyof IkaClient];

export type IkaRpcPaginatedMethods = {
	[K in IkaRpcPaginatedMethodName]: IkaClient[K] extends (
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

export type UseIkaClientInfiniteQueryOptions<
	T extends keyof IkaRpcPaginatedMethods,
	TData,
> = PartialBy<
	Omit<
		UseInfiniteQueryOptions<
			IkaRpcPaginatedMethods[T]['result'],
			Error,
			TData,
			IkaRpcPaginatedMethods[T]['result'],
			unknown[]
		>,
		'queryFn' | 'initialPageParam' | 'getNextPageParam'
	>,
	'queryKey'
>;

export function useIkaClientInfiniteQuery<
	T extends keyof IkaRpcPaginatedMethods,
	TData = InfiniteData<IkaRpcPaginatedMethods[T]['result']>,
>(
	method: T,
	params: IkaRpcPaginatedMethods[T]['params'],
	{
		queryKey = [],
		enabled = !!params,
		...options
	}: UseIkaClientInfiniteQueryOptions<T, TData> = {},
): UseInfiniteQueryResult<TData, Error> {
	const ikaContext = useIkaClientContext();

	return useInfiniteQuery({
		...options,
		initialPageParam: null,
		queryKey: [ikaContext.network, method, params, ...queryKey],
		enabled,
		queryFn: ({ pageParam }) =>
			ikaContext.client[method]({
				...(params ?? {}),
				cursor: pageParam,
			} as never),
		getNextPageParam: (lastPage) => (lastPage.hasNextPage ? lastPage.nextCursor ?? null : null),
	});
}
