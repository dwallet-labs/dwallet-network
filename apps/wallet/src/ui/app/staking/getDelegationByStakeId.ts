// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { DelegatedStake } from '@ika-io/ika/client';

// Helper function to get the delegation by stakedIkaId
export const getDelegationDataByStakeId = (
	delegationsStake: DelegatedStake[],
	stakeIkaId: string,
) => {
	let stake = null;
	for (const { stakes } of delegationsStake) {
		stake = stakes.find(({ stakedIkaId }) => stakedIkaId === stakeIkaId) || null;
		if (stake) return stake;
	}

	return stake;
};
