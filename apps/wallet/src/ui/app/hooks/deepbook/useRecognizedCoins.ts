// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { useDeepBookConfigs } from '_app/hooks/deepbook/useDeepBookConfigs';
import { useDeepBookContext } from '_shared/deepBook/context';
import { SUI_TYPE_ARG } from '@dwallet/dwallet.js/utils';

export function useRecognizedCoins() {
	const coinsMap = useDeepBookContext().configs.coinsMap;
	return Object.values(coinsMap);
}

export function useAllowedSwapCoinsList() {
	const deepBookConfigs = useDeepBookConfigs();
	const coinsMap = deepBookConfigs.coinsMap;

	return [SUI_TYPE_ARG, coinsMap.DWLT, coinsMap.USDC];
}
