"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.FaucetRateLimitError = void 0;
exports.requestSuiFromFaucetV0 = requestSuiFromFaucetV0;
exports.requestSuiFromFaucetV1 = requestSuiFromFaucetV1;
exports.getFaucetRequestStatus = getFaucetRequestStatus;
exports.getFaucetHost = getFaucetHost;
class FaucetRateLimitError extends Error {
}
exports.FaucetRateLimitError = FaucetRateLimitError;
async function faucetRequest({ host, path, body, headers, method }) {
    const endpoint = new URL(path, host).toString();
    const res = await fetch(endpoint, {
        method,
        body: body ? JSON.stringify(body) : undefined,
        headers: {
            'Content-Type': 'application/json',
            ...(headers || {}),
        },
    });
    if (res.status === 429) {
        throw new FaucetRateLimitError(`Too many requests from this client have been sent to the faucet. Please retry later`);
    }
    try {
        const parsed = await res.json();
        if (parsed.error) {
            throw new Error(`Faucet returns error: ${parsed.error}`);
        }
        return parsed;
    }
    catch (e) {
        throw new Error(`Encountered error when parsing response from faucet, error: ${e}, status ${res.status}, response ${res}`);
    }
}
async function requestSuiFromFaucetV0(input) {
    return faucetRequest({
        host: input.host,
        path: '/gas',
        body: {
            FixedAmountRequest: {
                recipient: input.recipient,
            },
        },
        headers: input.headers,
        method: 'POST',
    });
}
async function requestSuiFromFaucetV1(input) {
    return faucetRequest({
        host: input.host,
        path: '/v1/gas',
        body: {
            FixedAmountRequest: {
                recipient: input.recipient,
            },
        },
        headers: input.headers,
        method: 'POST',
    });
}
async function getFaucetRequestStatus(input) {
    return faucetRequest({
        host: input.host,
        path: `/v1/status/${input.taskId}`,
        headers: input.headers,
        method: 'GET',
    });
}
function getFaucetHost(network) {
    switch (network) {
        case 'testnet':
            return 'http://faucet.alpha.testnet.dwallet.cloud';
        case 'devnet':
            return 'http://faucet.devnet.dwallet.cloud';
        case 'localnet':
            return 'http://127.0.0.1:9123';
        default:
            throw new Error(`Unknown network: ${network}`);
    }
}
//# sourceMappingURL=index.js.map