// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export function getFullnodeUrl(network: 'mainnet' | 'testnet' | 'devnet' | 'localnet') {
	switch (network) {
		// case 'mainnet':
		// 	return 'https://fullnode.mainnet.sui.io:443';
		case 'testnet':
			return 'http://fullnode.alpha.testnet.dwallet.cloud:9000';
		case 'devnet':
			return 'http://fullnode.devnet.dwallet.cloud:9000';
		case 'localnet':
			return 'http://127.0.0.1:9000';
		default:
			throw new Error(`Unknown network: ${network}`);
	}
}
