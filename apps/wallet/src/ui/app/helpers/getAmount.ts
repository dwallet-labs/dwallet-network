// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaEvent, IkaTransactionBlockKind, TransactionEffects } from '@ika-io/ika/client';

type FormattedBalance = {
	amount?: number | null;
	coinType?: string | null;
	recipientAddress: string;
}[];

export function getAmount(
	_txnData: IkaTransactionBlockKind,
	_txnEffect: TransactionEffects,
	_events: IkaEvent[],
): FormattedBalance | null {
	// TODO: Support programmable transactions:
	return null;
}
