// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type DelegatedStake } from '@dwallet-network/dwallet.js/client';

// Get staked Sui
export const getAllStakeSui = (allDelegation: DelegatedStake[]) => {
	return (
		allDelegation.reduce(
			(acc, curr) => curr.stakes.reduce((total, { principal }) => total + BigInt(principal), acc),
			0n,
		) || 0n
	);
};
