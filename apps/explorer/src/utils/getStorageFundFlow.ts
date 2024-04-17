// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type EndOfEpochInfo } from '@dwallet/dwallet.js/client';

export function getEpochStorageFundFlow(endOfEpochInfo: EndOfEpochInfo | null) {
	const fundInflow = endOfEpochInfo
		? BigInt(endOfEpochInfo.storageFundReinvestment) +
		  BigInt(endOfEpochInfo.storageCharge) +
		  BigInt(endOfEpochInfo.leftoverStorageFundInflow)
		: null;

	const fundOutflow = endOfEpochInfo ? BigInt(endOfEpochInfo.storageRebate) : null;

	const netInflow = fundInflow !== null && fundOutflow !== null ? fundInflow - fundOutflow : null;

	return { netInflow, fundInflow, fundOutflow };
}
