// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DWalletClient } from '@dwallet-network/dwallet.js/client';
import type {
	InfiniteData,
	UseInfiniteQueryOptions,
	UseInfiniteQueryResult,
} from '@tanstack/react-query';
import { useInfiniteQuery } from '@tanstack/react-query';

import type { PartialBy } from '../types/utilityTypes.js';
import { useSuiClientContext } from './useSuiClient.js';

interface PaginatedResult {
	data?: unknown;
	nextCursor?: unknown;
	hasNextPage: boolean;
}

export type SuiRpcPaginatedMethodName = {
	[K in keyof DWalletClient]: DWalletClient[K] extends (input: any) => Promise<PaginatedResult> ? K : never;
}[keyof DWalletClient];

export type SuiRpcPaginatedMethods = {
	[K in SuiRpcPaginatedMethodName]: DWalletClient[K] extends (
		input: infer Params,
	) => Promise<infer Result extends { nextCursor?: infer Cursor | null }>
		? {
				name: K;
				result: Result;
				params: Params;
				cursor: Cursor;
		  }
		: never;
};

export type UseSuiClientInfiniteQueryOptions<
	T extends keyof SuiRpcPaginatedMethods,
	TData,
> = PartialBy<
	Omit<
		UseInfiniteQueryOptions<
			SuiRpcPaginatedMethods[T]['result'],
			Error,
			TData,
			SuiRpcPaginatedMethods[T]['result'],
			unknown[]
		>,
		'queryFn' | 'initialPageParam' | 'getNextPageParam'
	>,
	'queryKey'
>;

export function useSuiClientInfiniteQuery<
	T extends keyof SuiRpcPaginatedMethods,
	TData = InfiniteData<SuiRpcPaginatedMethods[T]['result']>,
>(
	method: T,
	params: SuiRpcPaginatedMethods[T]['params'],
	{
		queryKey = [],
		enabled = !!params,
		...options
	}: UseSuiClientInfiniteQueryOptions<T, TData> = {},
): UseInfiniteQueryResult<TData, Error> {
	const suiContext = useSuiClientContext();

	return useInfiniteQuery({
		...options,
		initialPageParam: null,
		queryKey: [suiContext.network, method, params, ...queryKey],
		enabled,
		queryFn: ({ pageParam }) =>
			suiContext.client[method]({
				...(params ?? {}),
				cursor: pageParam,
			} as never),
		getNextPageParam: ({ nextCursor }) => nextCursor ?? null,
	});
}
