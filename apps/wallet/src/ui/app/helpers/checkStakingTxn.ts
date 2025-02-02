// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaTransactionBlockResponse } from '@ika-io/ika/client';

// TODO: Support programmable transactions:
export function checkStakingTxn(_txn: IkaTransactionBlockResponse) {
	return false;
}
