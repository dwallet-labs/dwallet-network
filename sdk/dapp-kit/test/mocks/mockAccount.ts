// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Ed25519Keypair } from '@pera-io/pera/keypairs/ed25519';
import type { WalletAccount } from '@mysten/wallet-standard';
import { ReadonlyWalletAccount } from '@mysten/wallet-standard';

export function createMockAccount(accountOverrides: Partial<WalletAccount> = {}) {
	const keypair = new Ed25519Keypair();
	return new ReadonlyWalletAccount({
		address: keypair.getPublicKey().toPeraAddress(),
		publicKey: keypair.getPublicKey().toPeraBytes(),
		chains: ['pera:unknown'],
		features: [
			'pera:signAndExecuteTransactionBlock',
			'pera:signTransactionBlock',
			'pera:signAndExecuteTransaction',
			'pera:signTransaction',
		],
		...accountOverrides,
	});
}
