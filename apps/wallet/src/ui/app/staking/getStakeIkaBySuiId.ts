// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type DelegatedStake } from '@ika-io/ika/client';

// Get Stake IKA by stakeIkaId
export const getStakeIkaByIkaId = (allDelegation: DelegatedStake[], stakeIkaId?: string | null) => {
	return (
		allDelegation.reduce((acc, curr) => {
			const total = BigInt(
				curr.stakes.find(({ stakedIkaId }) => stakedIkaId === stakeIkaId)?.principal || 0,
			);
			return total + acc;
		}, 0n) || 0n
	);
};
