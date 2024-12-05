// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useIkaClientContext, useIkaClientQuery, UseIkaClientQueryOptions } from '@mysten/dapp-kit';
import { GetObjectParams, IkaObjectResponse } from '@ika-io/ika/client';
import { useQueryClient, UseQueryResult } from '@tanstack/react-query';

export type UseObjectQueryOptions = UseIkaClientQueryOptions<'getObject', IkaObjectResponse>;
export type UseObjectQueryResponse = UseQueryResult<IkaObjectResponse, Error>;
export type InvalidateUseObjectQuery = () => void;

/**
 * Fetches an object, returning the response from RPC and a callback
 * to invalidate it.
 */
export function useObjectQuery(
	params: GetObjectParams,
	options?: UseObjectQueryOptions,
): [UseObjectQueryResponse, InvalidateUseObjectQuery] {
	const ctx = useIkaClientContext();
	const client = useQueryClient();
	const response = useIkaClientQuery('getObject', params, options);

	const invalidate = async () => {
		await client.invalidateQueries({
			queryKey: [ctx.network, 'getObject', params],
		});
	};

	return [response, invalidate];
}
