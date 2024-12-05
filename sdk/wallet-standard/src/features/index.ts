// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type {
	IdentifierRecord,
	StandardConnectFeature,
	StandardDisconnectFeature,
	StandardEventsFeature,
	WalletWithFeatures,
} from '@wallet-standard/core';

import type { IkaReportTransactionEffectsFeature } from './ikaReportTransactionEffects.js';
import type { IkaSignAndExecuteTransactionFeature } from './ikaSignAndExecuteTransaction.js';
import type { IkaSignAndExecuteTransactionBlockFeature } from './ikaSignAndExecuteTransactionBlock.js';
import type { IkaSignMessageFeature } from './ikaSignMessage.js';
import type { IkaSignPersonalMessageFeature } from './ikaSignPersonalMessage.js';
import type { IkaSignTransactionFeature } from './ikaSignTransaction.js';
import type { IkaSignTransactionBlockFeature } from './ikaSignTransactionBlock.js';

/**
 * Wallet Standard features that are unique to Ika, and that all Ika wallets are expected to implement.
 */
export type IkaFeatures = Partial<IkaSignTransactionBlockFeature> &
	Partial<IkaSignAndExecuteTransactionBlockFeature> &
	IkaSignPersonalMessageFeature &
	IkaSignAndExecuteTransactionFeature &
	IkaSignTransactionFeature &
	// This deprecated feature should be removed once wallets update to the new method:
	Partial<IkaSignMessageFeature> &
	Partial<IkaReportTransactionEffectsFeature>;

export type IkaWalletFeatures = StandardConnectFeature &
	StandardEventsFeature &
	IkaFeatures &
	// Disconnect is an optional feature:
	Partial<StandardDisconnectFeature>;

export type WalletWithIkaFeatures = WalletWithFeatures<IkaWalletFeatures>;

/**
 * Represents a wallet with the absolute minimum feature set required to function in the Ika ecosystem.
 */
export type WalletWithRequiredFeatures = WalletWithFeatures<
	MinimallyRequiredFeatures &
		Partial<IkaFeatures> &
		Partial<StandardDisconnectFeature> &
		IdentifierRecord<unknown>
>;

export type MinimallyRequiredFeatures = StandardConnectFeature & StandardEventsFeature;

export * from './ikaSignMessage.js';
export * from './ikaSignTransactionBlock.js';
export * from './ikaSignTransaction.js';
export * from './ikaSignAndExecuteTransactionBlock.js';
export * from './ikaSignAndExecuteTransaction.js';
export * from './ikaSignPersonalMessage.js';
export * from './ikaReportTransactionEffects.js';
