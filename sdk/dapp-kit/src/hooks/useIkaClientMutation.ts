// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { UseMutationOptions, UseMutationResult } from '@tanstack/react-query';
import { useMutation } from '@tanstack/react-query';

import { useIkaClientContext } from './useIkaClient.js';
import type { IkaRpcMethods } from './useIkaClientQuery.js';

export type UseIkaClientMutationOptions<T extends keyof IkaRpcMethods> = Omit<
	UseMutationOptions<IkaRpcMethods[T]['result'], Error, IkaRpcMethods[T]['params'], unknown[]>,
	'mutationFn'
>;

export function useIkaClientMutation<T extends keyof IkaRpcMethods>(
	method: T,
	options: UseIkaClientMutationOptions<T> = {},
): UseMutationResult<IkaRpcMethods[T]['result'], Error, IkaRpcMethods[T]['params'], unknown[]> {
	const ikaContext = useIkaClientContext();

	return useMutation({
		...options,
		mutationFn: async (params) => {
			return await ikaContext.client[method](params as never);
		},
	});
}
