// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { usePeraClient } from '@mysten/dapp-kit';
import { PeraObjectDataOptions, PeraObjectResponse } from '@pera-io/pera/client';
import { useQuery, UseQueryOptions } from '@tanstack/react-query';

import { chunkArray } from '../utils/chunkArray';

export function useMultiGetObjects(
	ids: string[],
	options: PeraObjectDataOptions,
	queryOptions?: Omit<UseQueryOptions<PeraObjectResponse[]>, 'queryKey' | 'queryFn'>,
) {
	const client = usePeraClient();
	return useQuery({
		...queryOptions,
		queryKey: ['multiGetObjects', ids],
		queryFn: async () => {
			const responses = await Promise.all(
				chunkArray(ids, 50).map((chunk) =>
					client.multiGetObjects({
						ids: chunk,
						options,
					}),
				),
			);
			return responses.flat();
		},
		enabled: !!ids?.length,
	});
}
