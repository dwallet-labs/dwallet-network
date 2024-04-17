// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type DelegatedStake } from '@dwallet/dwallet.js/client';

// Get total Stake SUI for a specific validator address
export const getTokenStakeSuiForValidator = (
	allDelegation: DelegatedStake[],
	validatorAddress?: string | null,
) => {
	return (
		allDelegation.reduce((acc, curr) => {
			if (validatorAddress === curr.validatorAddress) {
				return curr.stakes.reduce((total, { principal }) => total + BigInt(principal), acc);
			}
			return acc;
		}, 0n) || 0n
	);
};
