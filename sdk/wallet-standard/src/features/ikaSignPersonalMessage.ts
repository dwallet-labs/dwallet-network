// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signPersonalMessage API. */
export type IkaSignPersonalMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 */
export type IkaSignPersonalMessageFeature = {
	/** Namespace for the feature. */
	'ika:signPersonalMessage': {
		/** Version of the feature API. */
		version: IkaSignPersonalMessageVersion;
		signPersonalMessage: IkaSignPersonalMessageMethod;
	};
};

export type IkaSignPersonalMessageMethod = (
	input: IkaSignPersonalMessageInput,
) => Promise<IkaSignPersonalMessageOutput>;

/** Input for signing personal messages. */
export interface IkaSignPersonalMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/** Output of signing personal messages. */
export interface IkaSignPersonalMessageOutput extends SignedPersonalMessage {}

export interface SignedPersonalMessage {
	/** Base64 encoded message bytes */
	bytes: string;
	/** Base64 encoded signature */
	signature: string;
}
