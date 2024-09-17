// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { FEATURES } from '_src/shared/experimentation/features';
import { useFeatureValue } from '@growthbook/growthbook-react';

export function useCoinsReFetchingConfig() {
	const refetchInterval = useFeatureValue(FEATURES.WALLET_BALANCE_REFETCH_INTERVAL, 20_000);
	return {
		refetchInterval,
		staleTime: 5_000,
	};
}
