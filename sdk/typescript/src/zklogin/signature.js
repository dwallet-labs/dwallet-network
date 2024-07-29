"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.getZkLoginSignature = getZkLoginSignature;
exports.parseZkLoginSignature = parseZkLoginSignature;
const bcs_1 = require("@mysten/bcs");
const signature_scheme_js_1 = require("../cryptography/signature-scheme.js");
const bcs_js_1 = require("./bcs.js");
function getZkLoginSignatureBytes({ inputs, maxEpoch, userSignature }) {
    return bcs_js_1.zkLoginSignature
        .serialize({
        inputs,
        maxEpoch,
        userSignature: typeof userSignature === 'string' ? (0, bcs_1.fromB64)(userSignature) : userSignature,
    }, { maxSize: 2048 })
        .toBytes();
}
function getZkLoginSignature({ inputs, maxEpoch, userSignature }) {
    const bytes = getZkLoginSignatureBytes({ inputs, maxEpoch, userSignature });
    const signatureBytes = new Uint8Array(bytes.length + 1);
    signatureBytes.set([signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG.ZkLogin]);
    signatureBytes.set(bytes, 1);
    return (0, bcs_1.toB64)(signatureBytes);
}
function parseZkLoginSignature(signature) {
    return bcs_js_1.zkLoginSignature.parse(typeof signature === 'string' ? (0, bcs_1.fromB64)(signature) : signature);
}
//# sourceMappingURL=signature.js.map