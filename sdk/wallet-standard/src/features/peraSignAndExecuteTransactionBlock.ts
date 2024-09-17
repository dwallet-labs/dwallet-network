// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type {
	ExecuteTransactionRequestType,
	PeraTransactionBlockResponse,
	PeraTransactionBlockResponseOptions,
} from '@pera-io/pera/client';

import type { PeraSignTransactionBlockInput } from './peraSignTransactionBlock.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type PeraSignAndExecuteTransactionBlockVersion = '1.0.0';

/**
 * @deprecated Use `pera:signAndExecuteTransaction` instead.
 *
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type PeraSignAndExecuteTransactionBlockFeature = {
	/** Namespace for the feature. */
	'pera:signAndExecuteTransactionBlock': {
		/** Version of the feature API. */
		version: PeraSignAndExecuteTransactionBlockVersion;
		/** @deprecated Use `pera:signAndExecuteTransaction` instead. */
		signAndExecuteTransactionBlock: PeraSignAndExecuteTransactionBlockMethod;
	};
};

/** @deprecated Use `pera:signAndExecuteTransaction` instead. */
export type PeraSignAndExecuteTransactionBlockMethod = (
	input: PeraSignAndExecuteTransactionBlockInput,
) => Promise<PeraSignAndExecuteTransactionBlockOutput>;

/** Input for signing and sending transactions. */
export interface PeraSignAndExecuteTransactionBlockInput extends PeraSignTransactionBlockInput {
	/**
	 * @deprecated requestType will be ignored by JSON RPC in the future
	 */
	requestType?: ExecuteTransactionRequestType;
	/** specify which fields to return (e.g., transaction, effects, events, etc). By default, only the transaction digest will be returned. */
	options?: PeraTransactionBlockResponseOptions;
}

/** Output of signing and sending transactions. */
export interface PeraSignAndExecuteTransactionBlockOutput extends PeraTransactionBlockResponse {}
