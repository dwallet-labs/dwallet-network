// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useFormatCoin, CoinFormat, formatBalance } from '@mysten/core';
import { SUI_TYPE_ARG } from '@dwallet/dwallet.js/utils';
import { Text } from '@mysten/ui';

type StakeColumnProps = {
	stake: bigint | number | string;
	hideCoinSymbol?: boolean;
	inMIST?: boolean;
};

export function StakeColumn({ stake, hideCoinSymbol, inMIST = false }: StakeColumnProps) {
	const coinFormat = hideCoinSymbol ? CoinFormat.FULL : CoinFormat.ROUNDED;
	const [amount, symbol] = useFormatCoin(stake, SUI_TYPE_ARG, coinFormat);
	return (
		<div className="flex items-end gap-0.5">
			<Text variant="bodySmall/medium" color="steel-dark">
				{inMIST ? formatBalance(stake, 0, coinFormat) : amount}
			</Text>
			{!hideCoinSymbol && (
				<Text variant="captionSmall/medium" color="steel-dark">
					{inMIST ? 'nDWLT' : symbol}
				</Text>
			)}
		</div>
	);
}
