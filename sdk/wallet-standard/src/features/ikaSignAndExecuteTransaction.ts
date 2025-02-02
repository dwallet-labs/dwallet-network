// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SignedTransaction, IkaSignTransactionInput } from './ikaSignTransaction.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type IkaSignAndExecuteTransactionVersion = '2.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type IkaSignAndExecuteTransactionFeature = {
	/** Namespace for the feature. */
	'ika:signAndExecuteTransaction': {
		/** Version of the feature API. */
		version: IkaSignAndExecuteTransactionVersion;
		signAndExecuteTransaction: IkaSignAndExecuteTransactionMethod;
	};
};

export type IkaSignAndExecuteTransactionMethod = (
	input: IkaSignAndExecuteTransactionInput,
) => Promise<IkaSignAndExecuteTransactionOutput>;

/** Input for signing and sending transactions. */
export interface IkaSignAndExecuteTransactionInput extends IkaSignTransactionInput {}

/** Output of signing and sending transactions. */
export interface IkaSignAndExecuteTransactionOutput extends SignedTransaction {
	digest: string;
	/** Transaction effects as base64 encoded bcs. */
	effects: string;
}
