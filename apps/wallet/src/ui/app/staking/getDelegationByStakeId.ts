// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DelegatedStake } from '@dwallet-network/dwallet.js/client';

// Helper function to get the delegation by stakedSuiId
export const getDelegationDataByStakeId = (
	delegationsStake: DelegatedStake[],
	stakeSuiId: string,
) => {
	let stake = null;
	for (const { stakes } of delegationsStake) {
		stake = stakes.find(({ stakedSuiId }) => stakedSuiId === stakeSuiId) || null;
		if (stake) return stake;
	}

	return stake;
};
