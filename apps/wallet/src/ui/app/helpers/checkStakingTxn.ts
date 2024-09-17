// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraTransactionBlockResponse } from '@pera-io/pera/client';

// TODO: Support programmable transactions:
export function checkStakingTxn(_txn: PeraTransactionBlockResponse) {
	return false;
}
