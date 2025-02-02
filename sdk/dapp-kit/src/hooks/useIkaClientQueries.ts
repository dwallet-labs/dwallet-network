// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { UseQueryResult } from '@tanstack/react-query';
import { useQueries } from '@tanstack/react-query';

import { useIkaClientContext } from './useIkaClient.js';
import type { IkaRpcMethods, UseIkaClientQueryOptions } from './useIkaClientQuery.js';

type IkaClientQueryOptions = IkaRpcMethods[keyof IkaRpcMethods] extends infer Method
	? Method extends {
			name: infer M extends keyof IkaRpcMethods;
			params?: infer P;
		}
		? undefined extends P
			? {
					method: M;
					params?: P;
					options?: UseIkaClientQueryOptions<M, unknown>;
				}
			: {
					method: M;
					params: P;
					options?: UseIkaClientQueryOptions<M, unknown>;
				}
		: never
	: never;

export type UseIkaClientQueriesResults<Args extends readonly IkaClientQueryOptions[]> = {
	-readonly [K in keyof Args]: Args[K] extends {
		method: infer M extends keyof IkaRpcMethods;
		readonly options?:
			| {
					select?: (...args: any[]) => infer R;
			  }
			| object;
	}
		? UseQueryResult<unknown extends R ? IkaRpcMethods[M]['result'] : R>
		: never;
};

export function useIkaClientQueries<
	const Queries extends readonly IkaClientQueryOptions[],
	Results = UseIkaClientQueriesResults<Queries>,
>({
	queries,
	combine,
}: {
	queries: Queries;
	combine?: (results: UseIkaClientQueriesResults<Queries>) => Results;
}): Results {
	const ikaContext = useIkaClientContext();

	return useQueries({
		combine: combine as never,
		queries: queries.map((query) => {
			const { method, params, options: { queryKey = [], ...restOptions } = {} } = query;

			return {
				...restOptions,
				queryKey: [ikaContext.network, method, params, ...queryKey],
				queryFn: async () => {
					return await ikaContext.client[method](params as never);
				},
			};
		}) as [],
	});
}
