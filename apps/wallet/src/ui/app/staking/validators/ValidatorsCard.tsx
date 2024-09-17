// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import BottomMenuLayout, { Content, Menu } from '_app/shared/bottom-menu-layout';
import { Button } from '_app/shared/ButtonUI';
import { Card, CardItem } from '_app/shared/card';
import { Text } from '_app/shared/text';
import Alert from '_components/alert';
import LoadingIndicator from '_components/loading/LoadingIndicator';
import { ampli } from '_src/shared/analytics/ampli';
import {
	DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	DELEGATED_STAKES_QUERY_STALE_TIME,
} from '_src/shared/constants';
import { useGetDelegatedStake } from '@mysten/core';
import { usePeraClientQuery } from '@mysten/dapp-kit';
import { Plus12 } from '@mysten/icons';
import type { StakeObject } from '@pera-io/pera/client';
import { useMemo } from 'react';

import { useActiveAddress } from '../../hooks/useActiveAddress';
import { getAllStakePera } from '../getAllStakePera';
import { StakeAmount } from '../home/StakeAmount';
import { StakeCard, type DelegationObjectWithValidator } from '../home/StakedCard';

export function ValidatorsCard() {
	const accountAddress = useActiveAddress();
	const {
		data: delegatedStake,
		isPending,
		isError,
		error,
	} = useGetDelegatedStake({
		address: accountAddress || '',
		staleTime: DELEGATED_STAKES_QUERY_STALE_TIME,
		refetchInterval: DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	});

	const { data: system } = usePeraClientQuery('getLatestPeraSystemState');
	const activeValidators = system?.activeValidators;

	// Total active stake for all Staked validators
	const totalStake = useMemo(() => {
		if (!delegatedStake) return 0n;
		return getAllStakePera(delegatedStake);
	}, [delegatedStake]);

	const delegations = useMemo(() => {
		return delegatedStake?.flatMap((delegation) => {
			return delegation.stakes.map((d) => ({
				...d,
				// flag any inactive validator for the stakePera object
				// if the stakingPoolId is not found in the activeValidators list flag as inactive
				inactiveValidator: !activeValidators?.find(
					({ stakingPoolId }) => stakingPoolId === delegation.stakingPool,
				),
				validatorAddress: delegation.validatorAddress,
			}));
		});
	}, [activeValidators, delegatedStake]);

	// Check if there are any inactive validators
	const hasInactiveValidatorDelegation = delegations?.some(
		({ inactiveValidator }) => inactiveValidator,
	);

	// Get total rewards for all delegations
	const totalEarnTokenReward = useMemo(() => {
		if (!delegatedStake || !activeValidators) return 0n;
		return (
			delegatedStake.reduce(
				(acc, curr) =>
					curr.stakes.reduce(
						(total, { estimatedReward }: StakeObject & { estimatedReward?: string }) =>
							total + BigInt(estimatedReward || 0),
						acc,
					),
				0n,
			) || 0n
		);
	}, [delegatedStake, activeValidators]);

	const numberOfValidators = delegatedStake?.length || 0;

	if (isPending) {
		return (
			<div className="p-2 w-full flex justify-center items-center h-full">
				<LoadingIndicator />
			</div>
		);
	}

	if (isError) {
		return (
			<div className="p-2 w-full flex justify-center items-center h-full mb-2">
				<Alert>
					<strong>{error?.message}</strong>
				</Alert>
			</div>
		);
	}

	return (
		<div className="flex flex-col flex-nowrap h-full w-full">
			<BottomMenuLayout>
				<Content>
					<div className="mb-4">
						{hasInactiveValidatorDelegation ? (
							<div className="mb-3">
								<Alert>
									Unstake PERA from the inactive validators and stake on an active validator to start
									earning rewards again.
								</Alert>
							</div>
						) : null}
						<div className="grid grid-cols-2 gap-2.5 mb-4">
							{system &&
								delegations
									?.filter(({ inactiveValidator }) => inactiveValidator)
									.map((delegation) => (
										<StakeCard
											delegationObject={delegation as DelegationObjectWithValidator}
											currentEpoch={Number(system.epoch)}
											key={delegation.stakedPeraId}
											inactiveValidator
										/>
									))}
						</div>
						<Card
							padding="none"
							header={
								<div className="py-2.5 flex px-3.75 justify-center w-full">
									<Text variant="captionSmall" weight="semibold" color="steel-darker">
										Staking on {numberOfValidators}
										{numberOfValidators > 1 ? ' Validators' : ' Validator'}
									</Text>
								</div>
							}
						>
							<div className="flex divide-x divide-solid divide-gray-45 divide-y-0">
								<CardItem title="Your Stake">
									<StakeAmount balance={totalStake} variant="heading5" />
								</CardItem>
								<CardItem title="Earned">
									<StakeAmount balance={totalEarnTokenReward} variant="heading5" isEarnedRewards />
								</CardItem>
							</div>
						</Card>

						<div className="grid grid-cols-2 gap-2.5 mt-4">
							{system &&
								delegations
									?.filter(({ inactiveValidator }) => !inactiveValidator)
									.map((delegation) => (
										<StakeCard
											delegationObject={delegation as DelegationObjectWithValidator}
											currentEpoch={Number(system.epoch)}
											key={delegation.stakedPeraId}
										/>
									))}
						</div>
					</div>
				</Content>
				<Menu stuckClass="staked-cta" className="w-full px-0 pb-0 mx-0">
					<Button
						size="tall"
						variant="secondary"
						to="new"
						onClick={() =>
							ampli.clickedStakePera({
								isCurrentlyStaking: true,
								sourceFlow: 'Validator card',
							})
						}
						before={<Plus12 />}
						text="Stake PERA"
					/>
				</Menu>
			</BottomMenuLayout>
		</div>
	);
}
