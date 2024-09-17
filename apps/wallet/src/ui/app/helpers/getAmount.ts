// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraEvent, PeraTransactionBlockKind, TransactionEffects } from '@pera-io/pera/client';

type FormattedBalance = {
	amount?: number | null;
	coinType?: string | null;
	recipientAddress: string;
}[];

export function getAmount(
	_txnData: PeraTransactionBlockKind,
	_txnEffect: TransactionEffects,
	_events: PeraEvent[],
): FormattedBalance | null {
	// TODO: Support programmable transactions:
	return null;
}
