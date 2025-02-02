// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { IkaTransactionBlockResponse } from '@ika-io/ika/client';

// todo: add more logic for deriving transaction label
export const getLabel = (transaction: IkaTransactionBlockResponse, currentAddress?: string) => {
	const isSender = transaction.transaction?.data.sender === currentAddress;
	// Rename to "Send" to Transaction
	return isSender ? 'Transaction' : 'Receive';
};
