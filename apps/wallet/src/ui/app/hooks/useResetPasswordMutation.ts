// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useMutation } from '@tanstack/react-query';

import { type BackgroundClient } from '../background-client';
import { useBackgroundClient } from './useBackgroundClient';

export function useResetPasswordMutation() {
	const backgroundClient = useBackgroundClient();
	return useMutation({
		mutationKey: ['reset wallet password'],
		mutationFn: async (...args: Parameters<BackgroundClient['resetPassword']>) => {
			return await backgroundClient.resetPassword(...args);
		},
	});
}
