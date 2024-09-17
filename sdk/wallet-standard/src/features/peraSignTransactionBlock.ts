// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { Transaction } from '@pera-io/pera/transactions';
import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signTransactionBlock API. */
export type PeraSignTransactionBlockVersion = '1.0.0';

/**
 * @deprecated Use `pera:signTransaction` instead.
 *
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type PeraSignTransactionBlockFeature = {
	/** Namespace for the feature. */
	'pera:signTransactionBlock': {
		/** Version of the feature API. */
		version: PeraSignTransactionBlockVersion;
		/** @deprecated Use `pera:signTransaction` instead. */
		signTransactionBlock: PeraSignTransactionBlockMethod;
	};
};

/** @deprecated Use `pera:signTransaction` instead. */
export type PeraSignTransactionBlockMethod = (
	input: PeraSignTransactionBlockInput,
) => Promise<PeraSignTransactionBlockOutput>;

/** Input for signing transactions. */
export interface PeraSignTransactionBlockInput {
	transactionBlock: Transaction;
	account: WalletAccount;
	chain: IdentifierString;
}

/** Output of signing transactions. */
export interface PeraSignTransactionBlockOutput extends SignedTransactionBlock {}

export interface SignedTransactionBlock {
	/** Transaction as base64 encoded bcs. */
	transactionBlockBytes: string;
	/** Base64 encoded signature */
	signature: string;
}
