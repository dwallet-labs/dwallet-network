// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ValidatorLogo } from '_app/staking/validators/ValidatorLogo';
import { TxnAmount } from '_components/receipt-card/TxnAmount';
import { Text } from '_src/ui/app/shared/text';
import { useFormatCoin } from '@mysten/core';
import type { PeraEvent } from '@pera-io/pera/client';
import { PERA_TYPE_ARG } from '@pera-io/pera/utils';

import { Card } from '../../shared/transaction-summary/Card';

type UnStakeTxnCardProps = {
	event: PeraEvent;
};

export function UnStakeTxnCard({ event }: UnStakeTxnCardProps) {
	const json = event.parsedJson as {
		principal_amount?: number;
		reward_amount?: number;
		validator_address?: string;
	};
	const principalAmount = json?.principal_amount || 0;
	const rewardAmount = json?.reward_amount || 0;
	const validatorAddress = json?.validator_address;
	const totalAmount = Number(principalAmount) + Number(rewardAmount);
	const [formatPrinciple, symbol] = useFormatCoin(principalAmount, PERA_TYPE_ARG);
	const [formatRewards] = useFormatCoin(rewardAmount || 0, PERA_TYPE_ARG);

	return (
		<Card>
			<div className="flex flex-col divide-y divide-solid divide-gray-40 divide-x-0">
				{validatorAddress && (
					<div className="mb-3.5 w-full">
						<ValidatorLogo
							validatorAddress={validatorAddress}
							showAddress
							iconSize="md"
							size="body"
						/>
					</div>
				)}
				{totalAmount && <TxnAmount amount={totalAmount} coinType={PERA_TYPE_ARG} label="Total" />}

				<div className="flex justify-between w-full py-3.5">
					<div className="flex gap-1 items-baseline text-steel">
						<Text variant="body" weight="medium" color="steel-darker">
							Your PERA Stake
						</Text>
					</div>

					<div className="flex gap-1 items-baseline text-steel">
						<Text variant="body" weight="medium" color="steel-darker">
							{formatPrinciple} {symbol}
						</Text>
					</div>
				</div>

				<div className="flex justify-between w-full py-3.5">
					<div className="flex gap-1 items-baseline text-steel">
						<Text variant="body" weight="medium" color="steel-darker">
							Staking Rewards Earned
						</Text>
					</div>

					<div className="flex gap-1 items-baseline text-steel">
						<Text variant="body" weight="medium" color="steel-darker">
							{formatRewards} {symbol}
						</Text>
					</div>
				</div>
			</div>
		</Card>
	);
}
