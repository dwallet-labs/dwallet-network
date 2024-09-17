// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { WalletAccount } from '@wallet-standard/core';

/** The latest API version of the signPersonalMessage API. */
export type PeraSignPersonalMessageVersion = '1.0.0';

/**
 * A Wallet Standard feature for signing a personal message, and returning the
 * message bytes that were signed, and message signature.
 */
export type PeraSignPersonalMessageFeature = {
	/** Namespace for the feature. */
	'pera:signPersonalMessage': {
		/** Version of the feature API. */
		version: PeraSignPersonalMessageVersion;
		signPersonalMessage: PeraSignPersonalMessageMethod;
	};
};

export type PeraSignPersonalMessageMethod = (
	input: PeraSignPersonalMessageInput,
) => Promise<PeraSignPersonalMessageOutput>;

/** Input for signing personal messages. */
export interface PeraSignPersonalMessageInput {
	message: Uint8Array;
	account: WalletAccount;
}

/** Output of signing personal messages. */
export interface PeraSignPersonalMessageOutput extends SignedPersonalMessage {}

export interface SignedPersonalMessage {
	/** Base64 encoded message bytes */
	bytes: string;
	/** Base64 encoded signature */
	signature: string;
}
