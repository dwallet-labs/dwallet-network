// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useEffect } from 'react';
import { useNavigate } from 'react-router-dom';

import { useAccountSources } from '../../../hooks/useAccountSources';

export function ForgotPasswordIndexPage() {
	const allAccountSources = useAccountSources();
	const navigate = useNavigate();
	const totalRecoverable =
		allAccountSources.data?.filter(({ type }) => type === 'mnemonic').length || 0;
	useEffect(() => {
		if (allAccountSources.isPending) {
			return;
		}
		const url =
			totalRecoverable === 0 ? '/' : totalRecoverable === 1 ? './recover' : './recover-many';
		navigate(url, { replace: true });
	}, [allAccountSources.isPending, totalRecoverable, navigate]);
	return null;
}
