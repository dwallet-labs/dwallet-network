// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useResolvePeraNSName } from '_app/hooks/useAppResolvePeransName';
import { Text } from '_src/ui/app/shared/text';
import { useFormatCoin } from '@mysten/core';
import { usePeraClientQuery } from '@mysten/dapp-kit';
import { CheckFill16 } from '@mysten/icons';
import { formatAddress, PERA_TYPE_ARG } from '@pera-io/pera/utils';
import cl from 'clsx';

import { useCoinsReFetchingConfig } from '../../hooks';

type LedgerAccountRowProps = {
	isSelected: boolean;
	address: string;
};

export function LedgerAccountRow({ isSelected, address }: LedgerAccountRowProps) {
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();

	const { data: coinBalance } = usePeraClientQuery(
		'getBalance',
		{
			coinType: PERA_TYPE_ARG,
			owner: address,
		},
		{
			refetchInterval,
			staleTime,
		},
	);
	const domainName = useResolvePeraNSName(address);

	const [totalAmount, totalAmountSymbol] = useFormatCoin(
		coinBalance?.totalBalance ?? 0,
		PERA_TYPE_ARG,
	);

	return (
		<div className="flex items-center gap-3">
			<CheckFill16
				className={cl('w-4 h-4', {
					'text-gray-50': !isSelected,
					'text-success': isSelected,
				})}
			/>
			<Text
				mono
				variant="bodySmall"
				weight="semibold"
				color={isSelected ? 'steel-darker' : 'steel-dark'}
			>
				{domainName ?? formatAddress(address)}
			</Text>
			<div className="ml-auto">
				<Text variant="bodySmall" color="steel" weight="semibold" mono>
					{totalAmount} {totalAmountSymbol}
				</Text>
			</div>
		</div>
	);
}
