// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export function getFullnodeUrl(network: 'mainnet' | 'testnet' | 'devnet' | 'localnet') {
	switch (network) {
		case 'mainnet':
			return 'https://fullnode.mainnet.pera.io:443';
		case 'testnet':
			return 'https://fullnode.testnet.pera.io:443';
		case 'devnet':
			return 'https://fullnode.devnet.pera.io:443';
		case 'localnet':
			return 'http://127.0.0.1:9000';
		default:
			throw new Error(`Unknown network: ${network}`);
	}
}
