// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { FEATURES } from '_shared/experimentation/features';
import { useFeature } from '@growthbook/growthbook-react';

interface CoinMetadataOverride {
	[coinType: string]: {
		name?: string;
		iconUrl?: string;
	};
}

export function useCoinMetadataOverrides() {
	const coinMetadataOverrides = useFeature<CoinMetadataOverride>(
		FEATURES.TOKEN_METADATA_OVERRIDES,
	).value;

	return coinMetadataOverrides || {};
}
