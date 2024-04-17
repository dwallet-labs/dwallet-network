// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SuiEvent, SuiTransactionBlockKind, TransactionEffects } from '@dwallet/dwallet.js/client';

type FormattedBalance = {
	amount?: number | null;
	coinType?: string | null;
	recipientAddress: string;
}[];

export function getAmount(
	_txnData: SuiTransactionBlockKind,
	_txnEffect: TransactionEffects,
	_events: SuiEvent[],
): FormattedBalance | null {
	// TODO: Support programmable transactions:
	return null;
}
