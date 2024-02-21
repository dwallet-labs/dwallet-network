// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { COIN_GECKO_SUI_URL, useSuiCoinData } from '@mysten/core';
import { Sui } from '@mysten/icons';
import { Text } from '@mysten/ui';

import { Card } from '~/ui/Card';
import { ButtonOrLink } from '~/ui/utils/ButtonOrLink';

export function SuiTokenCard() {
	const { data } = useSuiCoinData();
	const { currentPrice } = data || {};

	const formattedPrice = currentPrice
		? currentPrice.toLocaleString('en', {
				style: 'currency',
				currency: 'USD',
		  })
		: '--';

	return (
		<ButtonOrLink href={COIN_GECKO_SUI_URL}>
			<Card growOnHover bg="white/80" spacing="lg" height="full">
				<div className="flex gap-2">
					<div className="mr-1 w-9 flex-shrink-0">
						<Sui className="h-full w-full" />
					</div>
					<div className="mt-1 flex w-full flex-col gap-0.5">
						<Text variant="body/semibold" color="steel-dark">
							1 DWLT = {formattedPrice}
						</Text>
						<Text variant="subtitleSmallExtra/medium" color="steel">
							via CoinGecko
						</Text>
					</div>
				</div>
			</Card>
		</ButtonOrLink>
	);
}
