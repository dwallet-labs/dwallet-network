// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	ExecuteTransactionRequestType,
	IkaTransactionBlockResponse,
	IkaTransactionBlockResponseOptions,
} from '@ika-io/ika/client';

import type { IkaSignTransactionBlockInput } from './ikaSignTransactionBlock.js';

/** The latest API version of the signAndExecuteTransactionBlock API. */
export type IkaSignAndExecuteTransactionBlockVersion = '1.0.0';

/**
 * @deprecated Use `ika:signAndExecuteTransaction` instead.
 *
 * A Wallet Standard feature for signing a transaction, and submitting it to the
 * network. The wallet is expected to submit the transaction to the network via RPC,
 * and return the transaction response.
 */
export type IkaSignAndExecuteTransactionBlockFeature = {
	/** Namespace for the feature. */
	'ika:signAndExecuteTransactionBlock': {
		/** Version of the feature API. */
		version: IkaSignAndExecuteTransactionBlockVersion;
		/** @deprecated Use `ika:signAndExecuteTransaction` instead. */
		signAndExecuteTransactionBlock: IkaSignAndExecuteTransactionBlockMethod;
	};
};

/** @deprecated Use `ika:signAndExecuteTransaction` instead. */
export type IkaSignAndExecuteTransactionBlockMethod = (
	input: IkaSignAndExecuteTransactionBlockInput,
) => Promise<IkaSignAndExecuteTransactionBlockOutput>;

/** Input for signing and sending transactions. */
export interface IkaSignAndExecuteTransactionBlockInput extends IkaSignTransactionBlockInput {
	/**
	 * @deprecated requestType will be ignored by JSON RPC in the future
	 */
	requestType?: ExecuteTransactionRequestType;
	/** specify which fields to return (e.g., transaction, effects, events, etc). By default, only the transaction digest will be returned. */
	options?: IkaTransactionBlockResponseOptions;
}

/** Output of signing and sending transactions. */
export interface IkaSignAndExecuteTransactionBlockOutput extends IkaTransactionBlockResponse {}
