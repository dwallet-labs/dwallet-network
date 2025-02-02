// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Ed25519Keypair } from '@ika-io/ika/keypairs/ed25519';
import type { WalletAccount } from '@mysten/wallet-standard';
import { ReadonlyWalletAccount } from '@mysten/wallet-standard';

export function createMockAccount(accountOverrides: Partial<WalletAccount> = {}) {
	const keypair = new Ed25519Keypair();
	return new ReadonlyWalletAccount({
		address: keypair.getPublicKey().toIkaAddress(),
		publicKey: keypair.getPublicKey().toIkaBytes(),
		chains: ['ika:unknown'],
		features: [
			'ika:signAndExecuteTransactionBlock',
			'ika:signTransactionBlock',
			'ika:signAndExecuteTransaction',
			'ika:signTransaction',
		],
		...accountOverrides,
	});
}
