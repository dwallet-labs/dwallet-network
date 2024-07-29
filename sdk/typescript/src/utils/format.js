"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.formatAddress = formatAddress;
exports.formatDigest = formatDigest;
const ELLIPSIS = '\u{2026}';
function formatAddress(address) {
    if (address.length <= 6) {
        return address;
    }
    const offset = address.startsWith('0x') ? 2 : 0;
    return `0x${address.slice(offset, offset + 4)}${ELLIPSIS}${address.slice(-4)}`;
}
function formatDigest(digest) {
    // Use 10 first characters
    return `${digest.slice(0, 10)}${ELLIPSIS}`;
}
//# sourceMappingURL=format.js.map