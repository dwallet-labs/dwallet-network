// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IkaClient } from '@ika-io/ika/client';
import type {
	UndefinedInitialDataOptions,
	UseQueryOptions,
	UseQueryResult,
} from '@tanstack/react-query';
import { queryOptions, useQuery, useSuspenseQuery } from '@tanstack/react-query';
import { useMemo } from 'react';

import type { PartialBy } from '../types/utilityTypes.js';
import { useIkaClientContext } from './useIkaClient.js';

export type IkaRpcMethodName = {
	[K in keyof IkaClient]: IkaClient[K] extends ((input: any) => Promise<any>) | (() => Promise<any>)
		? K
		: never;
}[keyof IkaClient];

export type IkaRpcMethods = {
	[K in IkaRpcMethodName]: IkaClient[K] extends (input: infer P) => Promise<infer R>
		? {
				name: K;
				result: R;
				params: P;
			}
		: IkaClient[K] extends () => Promise<infer R>
			? {
					name: K;
					result: R;
					params: undefined | object;
				}
			: never;
};

export type UseIkaClientQueryOptions<T extends keyof IkaRpcMethods, TData> = PartialBy<
	Omit<UseQueryOptions<IkaRpcMethods[T]['result'], Error, TData, unknown[]>, 'queryFn'>,
	'queryKey'
>;

export type GetIkaClientQueryOptions<T extends keyof IkaRpcMethods> = {
	client: IkaClient;
	network: string;
	method: T;
	options?: PartialBy<
		Omit<UndefinedInitialDataOptions<IkaRpcMethods[T]['result']>, 'queryFn'>,
		'queryKey'
	>;
} & (undefined extends IkaRpcMethods[T]['params']
	? { params?: IkaRpcMethods[T]['params'] }
	: { params: IkaRpcMethods[T]['params'] });

export function getIkaClientQuery<T extends keyof IkaRpcMethods>({
	client,
	network,
	method,
	params,
	options,
}: GetIkaClientQueryOptions<T>) {
	return queryOptions<IkaRpcMethods[T]['result']>({
		...options,
		queryKey: [network, method, params],
		queryFn: async () => {
			return await client[method](params as never);
		},
	});
}

export function useIkaClientQuery<
	T extends keyof IkaRpcMethods,
	TData = IkaRpcMethods[T]['result'],
>(
	...args: undefined extends IkaRpcMethods[T]['params']
		? [method: T, params?: IkaRpcMethods[T]['params'], options?: UseIkaClientQueryOptions<T, TData>]
		: [method: T, params: IkaRpcMethods[T]['params'], options?: UseIkaClientQueryOptions<T, TData>]
): UseQueryResult<TData, Error> {
	const [method, params, { queryKey = [], ...options } = {}] = args as [
		method: T,
		params?: IkaRpcMethods[T]['params'],
		options?: UseIkaClientQueryOptions<T, TData>,
	];

	const ikaContext = useIkaClientContext();

	return useQuery({
		...options,
		queryKey: [ikaContext.network, method, params, ...queryKey],
		queryFn: async () => {
			return await ikaContext.client[method](params as never);
		},
	});
}

export function useIkaClientSuspenseQuery<
	T extends keyof IkaRpcMethods,
	TData = IkaRpcMethods[T]['result'],
>(
	...args: undefined extends IkaRpcMethods[T]['params']
		? [method: T, params?: IkaRpcMethods[T]['params'], options?: UndefinedInitialDataOptions<TData>]
		: [method: T, params: IkaRpcMethods[T]['params'], options?: UndefinedInitialDataOptions<TData>]
) {
	const [method, params, options = {}] = args as [
		method: T,
		params?: IkaRpcMethods[T]['params'],
		options?: UndefinedInitialDataOptions<TData>,
	];

	const ikaContext = useIkaClientContext();

	const query = useMemo(() => {
		return getIkaClientQuery<T>({
			client: ikaContext.client,
			network: ikaContext.network,
			method,
			params,
			options,
		});
	}, [ikaContext.client, ikaContext.network, method, params, options]);

	return useSuspenseQuery(query);
}
