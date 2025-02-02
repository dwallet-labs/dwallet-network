// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useDeepBookConfigs } from '_app/hooks/deepbook/useDeepBookConfigs';
import { useDeepBookContext } from '_shared/deepBook/context';
import { IKA_TYPE_ARG } from '@ika-io/ika/utils';

export function useRecognizedCoins() {
	const coinsMap = useDeepBookContext().configs.coinsMap;
	return Object.values(coinsMap);
}

export function useAllowedSwapCoinsList() {
	const deepBookConfigs = useDeepBookConfigs();
	const coinsMap = deepBookConfigs.coinsMap;

	return [IKA_TYPE_ARG, coinsMap.IKA, coinsMap.USDC];
}
