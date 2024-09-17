// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { PeraTransactionBlockResponse } from '@pera-io/pera/client';

// todo: add more logic for deriving transaction label
export const getLabel = (transaction: PeraTransactionBlockResponse, currentAddress?: string) => {
	const isSender = transaction.transaction?.data.sender === currentAddress;
	// Rename to "Send" to Transaction
	return isSender ? 'Transaction' : 'Receive';
};
