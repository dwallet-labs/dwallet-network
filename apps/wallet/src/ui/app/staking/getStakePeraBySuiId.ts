// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type DelegatedStake } from '@pera-io/pera/client';

// Get Stake PERA by stakePeraId
export const getStakePeraByPeraId = (allDelegation: DelegatedStake[], stakePeraId?: string | null) => {
	return (
		allDelegation.reduce((acc, curr) => {
			const total = BigInt(
				curr.stakes.find(({ stakedPeraId }) => stakedPeraId === stakePeraId)?.principal || 0,
			);
			return total + acc;
		}, 0n) || 0n
	);
};
