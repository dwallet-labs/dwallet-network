// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { ResolvedNameServiceNames } from '@pera-io/pera/client';
import type { UseQueryOptions, UseQueryResult } from '@tanstack/react-query';

import { usePeraClientQuery } from './usePeraClientQuery.js';

export function useResolvePeraNSName(
	address?: string | null,
	options?: Omit<
		UseQueryOptions<ResolvedNameServiceNames, Error, string | null, unknown[]>,
		'queryFn' | 'queryKey' | 'select'
	>,
): UseQueryResult<string | null, Error> {
	return usePeraClientQuery(
		'resolveNameServiceNames',
		{
			address: address!,
			limit: 1,
		},
		{
			...options,
			refetchOnWindowFocus: false,
			retry: false,
			select: (data) => (data.data.length > 0 ? data.data[0] : null),
			enabled: !!address && options?.enabled !== false,
		},
	);
}
