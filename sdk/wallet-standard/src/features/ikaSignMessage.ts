// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { WalletAccount } from '@wallet-standard/core';

/**
 * The latest API version of the signMessage API.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `ika:signPersonalMessage` feature
 */
export type IkaSignMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 *
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `ika:signPersonalMessage` feature
 */
export type IkaSignMessageFeature = {
	/** Namespace for the feature. */
	'ika:signMessage': {
		/** Version of the feature API. */
		version: IkaSignMessageVersion;
		signMessage: IkaSignMessageMethod;
	};
};

/** @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `ika:signPersonalMessage` feature */
export type IkaSignMessageMethod = (input: IkaSignMessageInput) => Promise<IkaSignMessageOutput>;

/**
 * Input for signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `ika:signPersonalMessage` feature
 */
export interface IkaSignMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/**
 * Output of signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `ika:signPersonalMessage` feature
 */
export interface IkaSignMessageOutput {
	/** Base64 message bytes. */
	messageBytes: string;
	/** Base64 encoded signature */
	signature: string;
}
