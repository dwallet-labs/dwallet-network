// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SignedTransaction, PeraSignTransactionInput } from './peraSignTransaction.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type PeraSignAndExecuteTransactionVersion = '2.0.0';

/**
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type PeraSignAndExecuteTransactionFeature = {
	/** Namespace for the feature. */
	'pera:signAndExecuteTransaction': {
		/** Version of the feature API. */
		version: PeraSignAndExecuteTransactionVersion;
		signAndExecuteTransaction: PeraSignAndExecuteTransactionMethod;
	};
};

export type PeraSignAndExecuteTransactionMethod = (
	input: PeraSignAndExecuteTransactionInput,
) => Promise<PeraSignAndExecuteTransactionOutput>;

/** Input for signing and sending transactions. */
export interface PeraSignAndExecuteTransactionInput extends PeraSignTransactionInput {}

/** Output of signing and sending transactions. */
export interface PeraSignAndExecuteTransactionOutput extends SignedTransaction {
	digest: string;
	/** Transaction effects as base64 encoded bcs. */
	effects: string;
}
