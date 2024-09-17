// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraClient } from '@pera-io/pera/client';
import type { UseQueryOptions, UseQueryResult } from '@tanstack/react-query';
import { useQuery } from '@tanstack/react-query';

import type { PartialBy } from '../types/utilityTypes.js';
import { usePeraClientContext } from './usePeraClient.js';

export type PeraRpcMethodName = {
	[K in keyof PeraClient]: PeraClient[K] extends ((input: any) => Promise<any>) | (() => Promise<any>)
		? K
		: never;
}[keyof PeraClient];

export type PeraRpcMethods = {
	[K in PeraRpcMethodName]: PeraClient[K] extends (input: infer P) => Promise<infer R>
		? {
				name: K;
				result: R;
				params: P;
			}
		: PeraClient[K] extends () => Promise<infer R>
			? {
					name: K;
					result: R;
					params: undefined | object;
				}
			: never;
};

export type UsePeraClientQueryOptions<T extends keyof PeraRpcMethods, TData> = PartialBy<
	Omit<UseQueryOptions<PeraRpcMethods[T]['result'], Error, TData, unknown[]>, 'queryFn'>,
	'queryKey'
>;

export function usePeraClientQuery<
	T extends keyof PeraRpcMethods,
	TData = PeraRpcMethods[T]['result'],
>(
	...args: undefined extends PeraRpcMethods[T]['params']
		? [method: T, params?: PeraRpcMethods[T]['params'], options?: UsePeraClientQueryOptions<T, TData>]
		: [method: T, params: PeraRpcMethods[T]['params'], options?: UsePeraClientQueryOptions<T, TData>]
): UseQueryResult<TData, Error> {
	const [method, params, { queryKey = [], ...options } = {}] = args as [
		method: T,
		params?: PeraRpcMethods[T]['params'],
		options?: UsePeraClientQueryOptions<T, TData>,
	];

	const peraContext = usePeraClientContext();

	return useQuery({
		...options,
		queryKey: [peraContext.network, method, params, ...queryKey],
		queryFn: async () => {
			return await peraContext.client[method](params as never);
		},
	});
}
