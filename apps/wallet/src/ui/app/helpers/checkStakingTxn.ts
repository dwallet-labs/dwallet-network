// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SuiTransactionBlockResponse } from '@mysten/sui.js/client';

// TODO: Support programmable transactions:
export function checkStakingTxn(_txn: SuiTransactionBlockResponse) {
	return false;
}
