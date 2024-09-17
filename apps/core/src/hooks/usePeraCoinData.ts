// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useQuery } from '@tanstack/react-query';

import { useAppsBackend } from './useAppsBackend';

// TODO: We should consider using tRPC or something for apps-backend
type CoinData = {
	marketCap: string;
	fullyDilutedMarketCap: string;
	currentPrice: number;
	priceChangePercentageOver24H: number;
	circulatingSupply: number;
	totalSupply: number;
};

export const COIN_GECKO_PERA_URL = 'https://www.coingecko.com/en/coins/pera';

export function usePeraCoinData() {
	const { request } = useAppsBackend();
	return useQuery({
		queryKey: ['pera-coin-data'],
		queryFn: () => request<CoinData>('coins/pera', {}),
		gcTime: 24 * 60 * 60 * 1000,
		staleTime: Infinity,
	});
}
