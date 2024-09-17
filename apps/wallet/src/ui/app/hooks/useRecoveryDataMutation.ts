// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type PasswordRecoveryData } from '_src/shared/messaging/messages/payloads/MethodPayload';
import { useMutation } from '@tanstack/react-query';

import { useForgotPasswordContext } from '../pages/accounts/forgot-password/ForgotPasswordPage';
import { useBackgroundClient } from './useBackgroundClient';

export function useRecoveryDataMutation() {
	const backgroundClient = useBackgroundClient();
	const { add } = useForgotPasswordContext();
	return useMutation({
		mutationKey: ['add recovery data'],
		mutationFn: async (data: PasswordRecoveryData) => {
			await backgroundClient.verifyPasswordRecoveryData({ data });
			add(data);
		},
	});
}
