// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type DelegatedStake } from '@pera-io/pera/client';

// Get staked Pera
export const getAllStakePera = (allDelegation: DelegatedStake[]) => {
	return (
		allDelegation.reduce(
			(acc, curr) => curr.stakes.reduce((total, { principal }) => total + BigInt(principal), acc),
			0n,
		) || 0n
	);
};
