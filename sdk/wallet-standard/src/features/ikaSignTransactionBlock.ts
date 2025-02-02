// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { Transaction } from '@ika-io/ika/transactions';
import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signTransactionBlock API. */
export type IkaSignTransactionBlockVersion = '1.0.0';

/**
 * @deprecated Use `ika:signTransaction` instead.
 *
 * A Wallet Standard feature for signing a transaction, and returning the
 * serialized transaction and transaction signature.
 */
export type IkaSignTransactionBlockFeature = {
	/** Namespace for the feature. */
	'ika:signTransactionBlock': {
		/** Version of the feature API. */
		version: IkaSignTransactionBlockVersion;
		/** @deprecated Use `ika:signTransaction` instead. */
		signTransactionBlock: IkaSignTransactionBlockMethod;
	};
};

/** @deprecated Use `ika:signTransaction` instead. */
export type IkaSignTransactionBlockMethod = (
	input: IkaSignTransactionBlockInput,
) => Promise<IkaSignTransactionBlockOutput>;

/** Input for signing transactions. */
export interface IkaSignTransactionBlockInput {
	transactionBlock: Transaction;
	account: WalletAccount;
	chain: IdentifierString;
}

/** Output of signing transactions. */
export interface IkaSignTransactionBlockOutput extends SignedTransactionBlock {}

export interface SignedTransactionBlock {
	/** Transaction as base64 encoded bcs. */
	transactionBlockBytes: string;
	/** Base64 encoded signature */
	signature: string;
}
