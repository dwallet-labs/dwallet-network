// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SerializedSignature } from '../cryptography/signature.js';

export type SignedTransaction = {
	transactionBlockBytes: string;
	signature: SerializedSignature;
};

export type SignedMessage = {
	messageBytes: string;
	signature: SerializedSignature;
};
