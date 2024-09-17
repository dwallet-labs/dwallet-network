// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DelegatedStake } from '@pera-io/pera/client';

// Helper function to get the delegation by stakedPeraId
export const getDelegationDataByStakeId = (
	delegationsStake: DelegatedStake[],
	stakePeraId: string,
) => {
	let stake = null;
	for (const { stakes } of delegationsStake) {
		stake = stakes.find(({ stakedPeraId }) => stakedPeraId === stakePeraId) || null;
		if (stake) return stake;
	}

	return stake;
};
