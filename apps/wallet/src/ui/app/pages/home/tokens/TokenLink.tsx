// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { CoinItem } from '_components/active-coins-card/CoinItem';
import { ampli } from '_src/shared/analytics/ampli';
import { type CoinBalance } from '@pera-io/pera/client';
import { NPERA_PER_PERA } from '@pera-io/pera/utils';
import { type ReactNode } from 'react';
import { Link } from 'react-router-dom';

type Props = {
	coinBalance: CoinBalance;
	centerAction?: ReactNode;
	subtitle?: string;
};

export function TokenLink({ coinBalance, centerAction, subtitle }: Props) {
	return (
		<Link
			to={`/send?type=${encodeURIComponent(coinBalance.coinType)}`}
			onClick={() =>
				ampli.selectedCoin({
					coinType: coinBalance.coinType,
					totalBalance: Number(BigInt(coinBalance.totalBalance) / NPERA_PER_PERA),
					sourceFlow: 'TokenLink',
				})
			}
			key={coinBalance.coinType}
			className="no-underline w-full group/coin"
		>
			<CoinItem
				coinType={coinBalance.coinType}
				balance={BigInt(coinBalance.totalBalance)}
				centerAction={centerAction}
				subtitle={subtitle}
			/>
		</Link>
	);
}
