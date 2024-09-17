// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { WalletAccount } from '@wallet-standard/core';

/**
 * The latest API version of the signMessage API.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `pera:signPersonalMessage` feature
 */
export type PeraSignMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 *
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `pera:signPersonalMessage` feature
 */
export type PeraSignMessageFeature = {
	/** Namespace for the feature. */
	'pera:signMessage': {
		/** Version of the feature API. */
		version: PeraSignMessageVersion;
		signMessage: PeraSignMessageMethod;
	};
};

/** @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `pera:signPersonalMessage` feature */
export type PeraSignMessageMethod = (input: PeraSignMessageInput) => Promise<PeraSignMessageOutput>;

/**
 * Input for signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `pera:signPersonalMessage` feature
 */
export interface PeraSignMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/**
 * Output of signing messages.
 * @deprecated Wallets can still implement this method for compatibility, but this has been replaced by the `pera:signPersonalMessage` feature
 */
export interface PeraSignMessageOutput {
	/** Base64 message bytes. */
	messageBytes: string;
	/** Base64 encoded signature */
	signature: string;
}
