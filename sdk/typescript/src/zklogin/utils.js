"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.toPaddedBigEndianBytes = toPaddedBigEndianBytes;
exports.toBigEndianBytes = toBigEndianBytes;
const utils_1 = require("@noble/hashes/utils");
function findFirstNonZeroIndex(bytes) {
    for (let i = 0; i < bytes.length; i++) {
        if (bytes[i] !== 0) {
            return i;
        }
    }
    return -1;
}
// Derive bytearray from num where the bytearray is padded to the left with 0s to the specified width.
function toPaddedBigEndianBytes(num, width) {
    const hex = num.toString(16);
    return (0, utils_1.hexToBytes)(hex.padStart(width * 2, '0').slice(-width * 2));
}
// Derive bytearray from num where the bytearray is not padded with 0.
function toBigEndianBytes(num, width) {
    const bytes = toPaddedBigEndianBytes(num, width);
    const firstNonZeroIndex = findFirstNonZeroIndex(bytes);
    if (firstNonZeroIndex === -1) {
        return new Uint8Array([0]);
    }
    return bytes.slice(firstNonZeroIndex);
}
//# sourceMappingURL=utils.js.map