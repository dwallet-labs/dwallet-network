"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.SIGNATURE_FLAG_TO_SCHEME = exports.SIGNATURE_SCHEME_TO_SIZE = exports.SIGNATURE_SCHEME_TO_FLAG = void 0;
exports.SIGNATURE_SCHEME_TO_FLAG = {
    ED25519: 0x00,
    Secp256k1: 0x01,
    Secp256r1: 0x02,
    MultiSig: 0x03,
    ZkLogin: 0x05,
};
exports.SIGNATURE_SCHEME_TO_SIZE = {
    ED25519: 32,
    Secp256k1: 33,
    Secp256r1: 33,
};
exports.SIGNATURE_FLAG_TO_SCHEME = {
    0x00: 'ED25519',
    0x01: 'Secp256k1',
    0x02: 'Secp256r1',
    0x03: 'MultiSig',
    0x05: 'ZkLogin',
};
//# sourceMappingURL=signature-scheme.js.map