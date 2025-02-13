// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';

// This data changes every time the IKA contracts are being redeployed.
export const IKA_PACKAGE_ID = '0x66dca2cee84af8b507879dd7745672bdaa089fa98e5cb98165e657ec466b908e';
export const IKA_SYSTEM_PACKAGE_ID =
	'0x9b4ad924399f991023b9d053d4a81d880973d51c3e08bfa0c1ffb03e8f9d8436';
export const DWALLET_ECDSAK1_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1';
export const IKA_SYSTEM_OBJ_ID =
	'0x3eff62e4dfcbca5f92e5f7241041db2bfc0a0a64e15f047238805e3e9c15debe';
export const IKA_COIN_OBJECT_PATH = `${IKA_PACKAGE_ID}::ika::IKA`;
export const DWALLET_NETWORK_VERSION = 0;

export const SUI_PACKAGE_ID = '0x2';

export interface Config {
	keypair: Ed25519Keypair;
	client: SuiClient;
	timeout: number;
}

/**
 * Utility function to create a delay.
 */
export function delay(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

/**
 * TS representation of an event to start an MPC session.
 * Usually the only thing needed from this event is the `session_id`,
 * which is used to fetch the
 * completion event.
 */
export interface StartSessionEvent {
	session_id: string;
}
