// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type SignedTransaction } from '_src/ui/app/WalletSigner';
import type { IkaTransactionBlockResponse } from '@ika-io/ika/client';
import {
	type IkaSignAndExecuteTransactionBlockInput,
	type IkaSignMessageOutput,
} from '@mysten/wallet-standard';

export type TransactionDataType = {
	type: 'transaction';
	data: string;
	account: string;
	justSign?: boolean;
	requestType?: IkaSignAndExecuteTransactionBlockInput['requestType'];
	options?: IkaSignAndExecuteTransactionBlockInput['options'];
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
	txResult?: IkaTransactionBlockResponse | IkaSignMessageOutput;
	txResultError?: string;
	txSigned?: SignedTransaction;
	createdDate: string;
	tx: TransactionDataType | SignMessageDataType;
};

export interface SignMessageApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
	tx: SignMessageDataType;
	txResult?: IkaSignMessageOutput;
}

export interface TransactionApprovalRequest extends Omit<ApprovalRequest, 'txResult' | 'tx'> {
	tx: TransactionDataType;
	txResult?: IkaTransactionBlockResponse;
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
