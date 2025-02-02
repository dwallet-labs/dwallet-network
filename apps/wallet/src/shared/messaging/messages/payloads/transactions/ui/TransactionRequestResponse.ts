// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isBasePayload } from '_payloads';
import type { BasePayload, Payload } from '_payloads';
import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { IkaTransactionBlockResponse } from '@ika-io/ika/client';
import { type IkaSignMessageOutput } from '@mysten/wallet-standard';

export interface TransactionRequestResponse extends BasePayload {
	type: 'transaction-request-response';
	txID: string;
	approved: boolean;
	txResult?: IkaTransactionBlockResponse | IkaSignMessageOutput;
	txResultError?: string;
	txSigned?: SignedTransaction;
}

export function isTransactionRequestResponse(
	payload: Payload,
): payload is TransactionRequestResponse {
	return isBasePayload(payload) && payload.type === 'transaction-request-response';
}
