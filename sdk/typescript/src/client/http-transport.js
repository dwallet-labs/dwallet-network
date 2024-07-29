"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.SuiHTTPTransport = void 0;
const version_js_1 = require("../version.js");
const errors_js_1 = require("./errors.js");
const rpc_websocket_client_js_1 = require("./rpc-websocket-client.js");
class SuiHTTPTransport {
    #requestId = 0;
    #options;
    #websocketClient;
    constructor(options) {
        this.#options = options;
    }
    fetch(input, init) {
        const fetch = this.#options.fetch ?? globalThis.fetch;
        if (!this.fetch) {
            throw new Error('The current environment does not support fetch, you can provide a fetch implementation in the options for SuiHTTPTransport.');
        }
        return fetch(input, init);
    }
    #getWebsocketClient() {
        if (!this.#websocketClient) {
            const WebSocketConstructor = this.#options.WebSocketConstructor ?? globalThis.WebSocket;
            if (!WebSocketConstructor) {
                throw new Error('The current environment does not support WebSocket, you can provide a WebSocketConstructor in the options for SuiHTTPTransport.');
            }
            this.#websocketClient = new rpc_websocket_client_js_1.WebsocketClient(this.#options.websocket?.url ?? this.#options.url, {
                WebSocketConstructor: this.#options.WebSocketConstructor,
                ...this.#options.websocket,
            });
        }
        return this.#websocketClient;
    }
    async request(input) {
        this.#requestId += 1;
        const res = await this.fetch(this.#options.rpc?.url ?? this.#options.url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Client-Sdk-Type': 'typescript',
                'Client-Sdk-Version': version_js_1.PACKAGE_VERSION,
                'Client-Target-Api-Version': version_js_1.TARGETED_RPC_VERSION,
                ...this.#options.rpc?.headers,
            },
            body: JSON.stringify({
                jsonrpc: '2.0',
                id: this.#requestId,
                method: input.method,
                params: input.params,
            }),
        });
        if (!res.ok) {
            throw new errors_js_1.SuiHTTPStatusError(`Unexpected status code: ${res.status}`, res.status, res.statusText);
        }
        const data = await res.json();
        if ('error' in data && data.error != null) {
            throw new errors_js_1.JsonRpcError(data.error.message, data.error.code);
        }
        return data.result;
    }
    async subscribe(input) {
        const unsubscribe = await this.#getWebsocketClient().subscribe(input);
        return async () => !!(await unsubscribe());
    }
}
exports.SuiHTTPTransport = SuiHTTPTransport;
//# sourceMappingURL=http-transport.js.map