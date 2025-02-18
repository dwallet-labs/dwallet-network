// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';

export const DWALLET_ECDSAK1_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1';
export const DWALLET_NETWORK_VERSION = 0;

export const SUI_PACKAGE_ID = '0x2';

interface IkaConfig {
	ika_package_id: string;
	ika_system_package_id: string;
	ika_system_obj_id: string;
}

export interface Config {
	keypair: Ed25519Keypair;
	client: SuiClient;
	timeout: number;
	ikaConfig: IkaConfig;
}

/**
 * Utility function to create a delay.
 */
export function delay(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}