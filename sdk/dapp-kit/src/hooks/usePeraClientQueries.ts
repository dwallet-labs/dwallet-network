// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { UseQueryResult } from '@tanstack/react-query';
import { useQueries } from '@tanstack/react-query';

import { usePeraClientContext } from './usePeraClient.js';
import type { PeraRpcMethods, UsePeraClientQueryOptions } from './usePeraClientQuery.js';

type PeraClientQueryOptions = PeraRpcMethods[keyof PeraRpcMethods] extends infer Method
	? Method extends {
			name: infer M extends keyof PeraRpcMethods;
			params?: infer P;
		}
		? undefined extends P
			? {
					method: M;
					params?: P;
					options?: UsePeraClientQueryOptions<M, unknown>;
				}
			: {
					method: M;
					params: P;
					options?: UsePeraClientQueryOptions<M, unknown>;
				}
		: never
	: never;

export type UsePeraClientQueriesResults<Args extends readonly PeraClientQueryOptions[]> = {
	-readonly [K in keyof Args]: Args[K] extends {
		method: infer M extends keyof PeraRpcMethods;
		readonly options?:
			| {
					select?: (...args: any[]) => infer R;
			  }
			| object;
	}
		? UseQueryResult<unknown extends R ? PeraRpcMethods[M]['result'] : R>
		: never;
};

export function usePeraClientQueries<
	const Queries extends readonly PeraClientQueryOptions[],
	Results = UsePeraClientQueriesResults<Queries>,
>({
	queries,
	combine,
}: {
	queries: Queries;
	combine?: (results: UsePeraClientQueriesResults<Queries>) => Results;
}): Results {
	const peraContext = usePeraClientContext();

	return useQueries({
		combine: combine as never,
		queries: queries.map((query) => {
			const { method, params, options: { queryKey = [], ...restOptions } = {} } = query;

			return {
				...restOptions,
				queryKey: [peraContext.network, method, params, ...queryKey],
				queryFn: async () => {
					return await peraContext.client[method](params as never);
				},
			};
		}) as [],
	});
}
