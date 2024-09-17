// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
export const onChainAmountToFloat = (amount: string, decimals: number) => {
	const total = parseFloat(amount);

	return total / Math.pow(10, decimals);
};

export const formatAddress = (address: string) => {
	return `${address.substring(0, 4)}...${address.slice(-10)}`;
};
