// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { PeraTransactionBlockResponse } from '@pera-io/pera/client';
import {
	type PeraSignAndExecuteTransactionBlockInput,
	type PeraSignMessageOutput,
} from '@mysten/wallet-standard';

export type TransactionDataType = {
	type: 'transaction';
	data: string;
	account: string;
	justSign?: boolean;
	requestType?: PeraSignAndExecuteTransactionBlockInput['requestType'];
	options?: PeraSignAndExecuteTransactionBlockInput['options'];
};

export type SignMessageDataType = {
	type: 'sign-message';
	message: string;
	accountAddress: string;
};

export type ApprovalRequest = {
	id: string;
	approved: boolean | null;
	origin: string;
	originFavIcon?: string;
	txResult?: PeraTransactionBlockResponse | PeraSignMessageOutput;
	txResultError?: string;
	txSigned?: SignedTransaction;
	createdDate: string;
	tx: TransactionDataType | SignMessageDataType;
};

export interface SignMessageApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
	tx: SignMessageDataType;
	txResult?: PeraSignMessageOutput;
}

export interface TransactionApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
	tx: TransactionDataType;
	txResult?: PeraTransactionBlockResponse;
}

export function isSignMessageApprovalRequest(
	request: ApprovalRequest,
): request is SignMessageApprovalRequest {
	return request.tx.type === 'sign-message';
}

export function isTransactionApprovalRequest(
	request: ApprovalRequest,
): request is TransactionApprovalRequest {
	return request.tx.type !== 'sign-message';
}
