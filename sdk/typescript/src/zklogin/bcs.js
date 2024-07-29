"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.zkLoginSignature = void 0;
const bcs_1 = require("@mysten/bcs");
exports.zkLoginSignature = bcs_1.bcs.struct('ZkLoginSignature', {
    inputs: bcs_1.bcs.struct('ZkLoginSignatureInputs', {
        proofPoints: bcs_1.bcs.struct('ZkLoginSignatureInputsProofPoints', {
            a: bcs_1.bcs.vector(bcs_1.bcs.string()),
            b: bcs_1.bcs.vector(bcs_1.bcs.vector(bcs_1.bcs.string())),
            c: bcs_1.bcs.vector(bcs_1.bcs.string()),
        }),
        issBase64Details: bcs_1.bcs.struct('ZkLoginSignatureInputsClaim', {
            value: bcs_1.bcs.string(),
            indexMod4: bcs_1.bcs.u8(),
        }),
        headerBase64: bcs_1.bcs.string(),
        addressSeed: bcs_1.bcs.string(),
    }),
    maxEpoch: bcs_1.bcs.u64(),
    userSignature: bcs_1.bcs.vector(bcs_1.bcs.u8()),
});
//# sourceMappingURL=bcs.js.map