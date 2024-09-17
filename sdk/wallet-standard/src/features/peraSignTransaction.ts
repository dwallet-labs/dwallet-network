// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signTransaction API. */
export type PeraSignTransactionVersion = '2.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type PeraSignTransactionFeature = {
	/** Namespace for the feature. */
	'pera:signTransaction': {
		/** Version of the feature API. */
		version: PeraSignTransactionVersion;
		signTransaction: PeraSignTransactionMethod;
	};
};

export type PeraSignTransactionMethod = (
	input: PeraSignTransactionInput,
) => Promise<SignedTransaction>;

/** Input for signing transactions. */
export interface PeraSignTransactionInput {
	transaction: { toJSON: () => Promise<string> };
	account: WalletAccount;
	chain: IdentifierString;
	signal?: AbortSignal;
}

/** Output of signing transactions. */

export interface SignedTransaction {
	/** Transaction as base64 encoded bcs. */
	bytes: string;
	/** Base64 encoded signature */
	signature: string;
}
