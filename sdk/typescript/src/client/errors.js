"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.SuiHTTPStatusError = exports.JsonRpcError = exports.SuiHTTPTransportError = void 0;
const CODE_TO_ERROR_TYPE = {
    '-32700': 'ParseError',
    '-32600': 'InvalidRequest',
    '-32601': 'MethodNotFound',
    '-32602': 'InvalidParams',
    '-32603': 'InternalError',
};
class SuiHTTPTransportError extends Error {
}
exports.SuiHTTPTransportError = SuiHTTPTransportError;
class JsonRpcError extends SuiHTTPTransportError {
    code;
    type;
    constructor(message, code) {
        super(message);
        this.code = code;
        this.type = CODE_TO_ERROR_TYPE[code] ?? 'ServerError';
    }
}
exports.JsonRpcError = JsonRpcError;
class SuiHTTPStatusError extends SuiHTTPTransportError {
    status;
    statusText;
    constructor(message, status, statusText) {
        super(message);
        this.status = status;
        this.statusText = statusText;
    }
}
exports.SuiHTTPStatusError = SuiHTTPStatusError;
//# sourceMappingURL=errors.js.map