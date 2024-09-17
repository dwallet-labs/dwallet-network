// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type {
	IdentifierRecord,
	StandardConnectFeature,
	StandardDisconnectFeature,
	StandardEventsFeature,
	WalletWithFeatures,
} from '@wallet-standard/core';

import type { PeraReportTransactionEffectsFeature } from './peraReportTransactionEffects.js';
import type { PeraSignAndExecuteTransactionFeature } from './peraSignAndExecuteTransaction.js';
import type { PeraSignAndExecuteTransactionBlockFeature } from './peraSignAndExecuteTransactionBlock.js';
import type { PeraSignMessageFeature } from './peraSignMessage.js';
import type { PeraSignPersonalMessageFeature } from './peraSignPersonalMessage.js';
import type { PeraSignTransactionFeature } from './peraSignTransaction.js';
import type { PeraSignTransactionBlockFeature } from './peraSignTransactionBlock.js';

/**
 * Wallet Standard features that are unique to Pera, and that all Pera wallets are expected to implement.
 */
export type PeraFeatures = Partial<PeraSignTransactionBlockFeature> &
	Partial<PeraSignAndExecuteTransactionBlockFeature> &
	PeraSignPersonalMessageFeature &
	PeraSignAndExecuteTransactionFeature &
	PeraSignTransactionFeature &
	// This deprecated feature should be removed once wallets update to the new method:
	Partial<PeraSignMessageFeature> &
	Partial<PeraReportTransactionEffectsFeature>;

export type PeraWalletFeatures = StandardConnectFeature &
	StandardEventsFeature &
	PeraFeatures &
	// Disconnect is an optional feature:
	Partial<StandardDisconnectFeature>;

export type WalletWithPeraFeatures = WalletWithFeatures<PeraWalletFeatures>;

/**
 * Represents a wallet with the absolute minimum feature set required to function in the Pera ecosystem.
 */
export type WalletWithRequiredFeatures = WalletWithFeatures<
	MinimallyRequiredFeatures &
		Partial<PeraFeatures> &
		Partial<StandardDisconnectFeature> &
		IdentifierRecord<unknown>
>;

export type MinimallyRequiredFeatures = StandardConnectFeature & StandardEventsFeature;

export * from './peraSignMessage.js';
export * from './peraSignTransactionBlock.js';
export * from './peraSignTransaction.js';
export * from './peraSignAndExecuteTransactionBlock.js';
export * from './peraSignAndExecuteTransaction.js';
export * from './peraSignPersonalMessage.js';
export * from './peraReportTransactionEffects.js';
