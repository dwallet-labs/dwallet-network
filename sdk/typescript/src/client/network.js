"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.getFullnodeUrl = getFullnodeUrl;
function getFullnodeUrl(network) {
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
//# sourceMappingURL=network.js.map