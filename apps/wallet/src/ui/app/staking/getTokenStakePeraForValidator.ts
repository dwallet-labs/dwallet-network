// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type DelegatedStake } from '@pera-io/pera/client';

// Get total Stake PERA for a specific validator address
export const getTokenStakePeraForValidator = (
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
