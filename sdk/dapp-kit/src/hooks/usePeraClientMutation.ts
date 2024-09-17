// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { UseMutationOptions, UseMutationResult } from '@tanstack/react-query';
import { useMutation } from '@tanstack/react-query';

import { usePeraClientContext } from './usePeraClient.js';
import type { PeraRpcMethods } from './usePeraClientQuery.js';

export type UsePeraClientMutationOptions<T extends keyof PeraRpcMethods> = Omit<
	UseMutationOptions<PeraRpcMethods[T]['result'], Error, PeraRpcMethods[T]['params'], unknown[]>,
	'mutationFn'
>;

export function usePeraClientMutation<T extends keyof PeraRpcMethods>(
	method: T,
	options: UsePeraClientMutationOptions<T> = {},
): UseMutationResult<PeraRpcMethods[T]['result'], Error, PeraRpcMethods[T]['params'], unknown[]> {
	const peraContext = usePeraClientContext();

	return useMutation({
		...options,
		mutationFn: async (params) => {
			return await peraContext.client[method](params as never);
		},
	});
}
