// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useIkaClient } from '@mysten/dapp-kit';
import { IkaObjectDataOptions, IkaObjectResponse } from '@ika-io/ika/client';
import { useQuery, UseQueryOptions } from '@tanstack/react-query';

import { chunkArray } from '../utils/chunkArray';

export function useMultiGetObjects(
	ids: string[],
	options: IkaObjectDataOptions,
	queryOptions?: Omit<UseQueryOptions<IkaObjectResponse[]>, 'queryKey' | 'queryFn'>,
) {
	const client = useIkaClient();
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
