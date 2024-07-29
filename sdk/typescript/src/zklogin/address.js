"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.computeZkLoginAddressFromSeed = computeZkLoginAddressFromSeed;
const blake2b_1 = require("@noble/hashes/blake2b");
const utils_1 = require("@noble/hashes/utils");
const signature_scheme_js_1 = require("../cryptography/signature-scheme.js");
const index_js_1 = require("../utils/index.js");
const utils_js_1 = require("./utils.js");
function computeZkLoginAddressFromSeed(addressSeed, iss) {
    const addressSeedBytesBigEndian = (0, utils_js_1.toBigEndianBytes)(addressSeed, 32);
    if (iss === 'accounts.google.com') {
        iss = 'https://accounts.google.com';
    }
    const addressParamBytes = new TextEncoder().encode(iss);
    const tmp = new Uint8Array(2 + addressSeedBytesBigEndian.length + addressParamBytes.length);
    tmp.set([signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG.ZkLogin]);
    tmp.set([addressParamBytes.length], 1);
    tmp.set(addressParamBytes, 2);
    tmp.set(addressSeedBytesBigEndian, 2 + addressParamBytes.length);
    return (0, index_js_1.normalizeSuiAddress)((0, utils_1.bytesToHex)((0, blake2b_1.blake2b)(tmp, { dkLen: 32 })).slice(0, index_js_1.SUI_ADDRESS_LENGTH * 2));
}
//# sourceMappingURL=address.js.map