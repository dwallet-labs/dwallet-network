// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import BigNumber from 'bignumber.js';

export function parseAmount(amount: string, coinDecimals: number) {
	try {
		return BigInt(new BigNumber(amount).shiftedBy(coinDecimals).integerValue().toString());
	} catch (e) {
		return BigInt(0);
	}
}
