// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { isBasePayload } from '_payloads';
import type { BasePayload, Payload } from '_payloads';
import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { PeraTransactionBlockResponse } from '@pera-io/pera/client';
import { type PeraSignMessageOutput } from '@mysten/wallet-standard';

export interface TransactionRequestResponse extends BasePayload {
	type: 'transaction-request-response';
	txID: string;
	approved: boolean;
	txResult?: PeraTransactionBlockResponse | PeraSignMessageOutput;
	txResultError?: string;
	txSigned?: SignedTransaction;
}

export function isTransactionRequestResponse(
	payload: Payload,
): payload is TransactionRequestResponse {
	return isBasePayload(payload) && payload.type === 'transaction-request-response';
}
