"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.WebsocketClient = exports.DEFAULT_CLIENT_OPTIONS = void 0;
const errors_js_1 = require("./errors.js");
function getWebsocketUrl(httpUrl) {
    const url = new URL(httpUrl);
    url.protocol = url.protocol.replace('http', 'ws');
    return url.toString();
}
exports.DEFAULT_CLIENT_OPTIONS = {
    // We fudge the typing because we also check for undefined in the constructor:
    WebSocketConstructor: (typeof WebSocket !== 'undefined'
        ? WebSocket
        : undefined),
    callTimeout: 30000,
    reconnectTimeout: 3000,
    maxReconnects: 5,
};
class WebsocketClient {
    endpoint;
    options;
    #requestId = 0;
    #disconnects = 0;
    #webSocket = null;
    #connectionPromise = null;
    #subscriptions = new Set();
    #pendingRequests = new Map();
    constructor(endpoint, options = {}) {
        this.endpoint = endpoint;
        this.options = { ...exports.DEFAULT_CLIENT_OPTIONS, ...options };
        if (!this.options.WebSocketConstructor) {
            throw new Error('Missing WebSocket constructor');
        }
        if (this.endpoint.startsWith('http')) {
            this.endpoint = getWebsocketUrl(this.endpoint);
        }
    }
    async makeRequest(method, params) {
        const webSocket = await this.#setupWebSocket();
        return new Promise((resolve, reject) => {
            this.#requestId += 1;
            this.#pendingRequests.set(this.#requestId, {
                resolve: resolve,
                reject,
                timeout: setTimeout(() => {
                    this.#pendingRequests.delete(this.#requestId);
                    reject(new Error(`Request timeout: ${method}`));
                }, this.options.callTimeout),
            });
            webSocket.send(JSON.stringify({ jsonrpc: '2.0', id: this.#requestId, method, params }));
        }).then(({ error, result }) => {
            if (error) {
                throw new errors_js_1.JsonRpcError(error.message, error.code);
            }
            return result;
        });
    }
    #setupWebSocket() {
        if (this.#connectionPromise) {
            return this.#connectionPromise;
        }
        this.#connectionPromise = new Promise((resolve) => {
            this.#webSocket?.close();
            this.#webSocket = new this.options.WebSocketConstructor(this.endpoint);
            this.#webSocket.addEventListener('open', () => {
                this.#disconnects = 0;
                resolve(this.#webSocket);
            });
            this.#webSocket.addEventListener('close', () => {
                this.#disconnects++;
                if (this.#disconnects <= this.options.maxReconnects) {
                    setTimeout(() => {
                        this.#reconnect();
                    }, this.options.reconnectTimeout);
                }
            });
            this.#webSocket.addEventListener('message', ({ data }) => {
                let json;
                try {
                    json = JSON.parse(data);
                }
                catch (error) {
                    console.error(new Error(`Failed to parse RPC message: ${data}`, { cause: error }));
                    return;
                }
                if ('id' in json && json.id != null && this.#pendingRequests.has(json.id)) {
                    const { resolve, timeout } = this.#pendingRequests.get(json.id);
                    clearTimeout(timeout);
                    resolve(json);
                }
                else if ('params' in json) {
                    const { params } = json;
                    this.#subscriptions.forEach((subscription) => {
                        if (subscription.subscriptionId === params.subscription)
                            if (params.subscription === subscription.subscriptionId) {
                                subscription.onMessage(params.result);
                            }
                    });
                }
            });
        });
        return this.#connectionPromise;
    }
    async #reconnect() {
        this.#webSocket?.close();
        this.#connectionPromise = null;
        return Promise.allSettled([...this.#subscriptions].map((subscription) => subscription.subscribe(this)));
    }
    async subscribe(input) {
        const subscription = new RpcSubscription(input);
        this.#subscriptions.add(subscription);
        await subscription.subscribe(this);
        return () => subscription.unsubscribe(this);
    }
}
exports.WebsocketClient = WebsocketClient;
class RpcSubscription {
    subscriptionId = null;
    input;
    subscribed = false;
    constructor(input) {
        this.input = input;
    }
    onMessage(message) {
        if (this.subscribed) {
            this.input.onMessage(message);
        }
    }
    async unsubscribe(client) {
        const { subscriptionId } = this;
        this.subscribed = false;
        if (subscriptionId == null)
            return false;
        this.subscriptionId = null;
        return client.makeRequest(this.input.unsubscribe, [subscriptionId]);
    }
    async subscribe(client) {
        this.subscriptionId = null;
        this.subscribed = true;
        const newSubscriptionId = await client.makeRequest(this.input.method, this.input.params);
        if (this.subscribed) {
            this.subscriptionId = newSubscriptionId;
        }
    }
}
//# sourceMappingURL=rpc-websocket-client.js.map