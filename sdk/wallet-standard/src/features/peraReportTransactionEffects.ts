// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IdentifierString, WalletAccount } from '@wallet-standard/core';

/**
 * A Wallet Standard feature for reporting the effects of a transaction block executed by a dapp
 * The feature allows wallets to updated their caches using the effects of the transaction
 * executed outside of the wallet
 */
export type PeraReportTransactionEffectsFeature = {
	/** Namespace for the feature. */
	'pera:reportTransactionEffects': {
		/** Version of the feature API. */
		version: '1.0.0';
		reportTransactionEffects: PeraReportTransactionEffectsMethod;
	};
};

export type PeraReportTransactionEffectsMethod = (
	input: PeraReportTransactionEffectsInput,
) => Promise<void>;

/** Input for signing transactions. */
export interface PeraReportTransactionEffectsInput {
	account: WalletAccount;
	chain: IdentifierString;
	/** Transaction effects as base64 encoded bcs. */
	effects: string;
}
