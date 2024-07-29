"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.hashTypedData = hashTypedData;
const blake2b_1 = require("@noble/hashes/blake2b");
/**
 * Generates a Blake2b hash of typed data as a base64 string.
 *
 * @param typeTag type tag (e.g. TransactionData, SenderSignedData)
 * @param data data to hash
 */
function hashTypedData(typeTag, data) {
    const typeTagBytes = Array.from(`${typeTag}::`).map((e) => e.charCodeAt(0));
    const dataWithTag = new Uint8Array(typeTagBytes.length + data.length);
    dataWithTag.set(typeTagBytes);
    dataWithTag.set(data, typeTagBytes.length);
    return (0, blake2b_1.blake2b)(dataWithTag, { dkLen: 32 });
}
//# sourceMappingURL=hash.js.map