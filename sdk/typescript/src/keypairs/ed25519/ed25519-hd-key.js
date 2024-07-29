"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.derivePath = exports.isValidPath = exports.getPublicKey = exports.getMasterKeyFromSeed = exports.replaceDerive = exports.pathRegex = void 0;
// This is adapted from https://github.com/alepop/ed25519-hd-key replacing create-hmac
// with @noble/hashes to be browser compatible.
const bcs_1 = require("@mysten/bcs");
const hmac_1 = require("@noble/hashes/hmac");
const sha512_1 = require("@noble/hashes/sha512");
const tweetnacl_1 = __importDefault(require("tweetnacl"));
const ED25519_CURVE = 'ed25519 seed';
const HARDENED_OFFSET = 0x80000000;
exports.pathRegex = new RegExp("^m(\\/[0-9]+')+$");
const replaceDerive = (val) => val.replace("'", '');
exports.replaceDerive = replaceDerive;
const getMasterKeyFromSeed = (seed) => {
    const h = hmac_1.hmac.create(sha512_1.sha512, ED25519_CURVE);
    const I = h.update((0, bcs_1.fromHEX)(seed)).digest();
    const IL = I.slice(0, 32);
    const IR = I.slice(32);
    return {
        key: IL,
        chainCode: IR,
    };
};
exports.getMasterKeyFromSeed = getMasterKeyFromSeed;
const CKDPriv = ({ key, chainCode }, index) => {
    const indexBuffer = new ArrayBuffer(4);
    const cv = new DataView(indexBuffer);
    cv.setUint32(0, index);
    const data = new Uint8Array(1 + key.length + indexBuffer.byteLength);
    data.set(new Uint8Array(1).fill(0));
    data.set(key, 1);
    data.set(new Uint8Array(indexBuffer, 0, indexBuffer.byteLength), key.length + 1);
    const I = hmac_1.hmac.create(sha512_1.sha512, chainCode).update(data).digest();
    const IL = I.slice(0, 32);
    const IR = I.slice(32);
    return {
        key: IL,
        chainCode: IR,
    };
};
const getPublicKey = (privateKey, withZeroByte = true) => {
    const keyPair = tweetnacl_1.default.sign.keyPair.fromSeed(privateKey);
    const signPk = keyPair.secretKey.subarray(32);
    const newArr = new Uint8Array(signPk.length + 1);
    newArr.set([0]);
    newArr.set(signPk, 1);
    return withZeroByte ? newArr : signPk;
};
exports.getPublicKey = getPublicKey;
const isValidPath = (path) => {
    if (!exports.pathRegex.test(path)) {
        return false;
    }
    return !path
        .split('/')
        .slice(1)
        .map(exports.replaceDerive)
        .some(isNaN /* ts T_T*/);
};
exports.isValidPath = isValidPath;
const derivePath = (path, seed, offset = HARDENED_OFFSET) => {
    if (!(0, exports.isValidPath)(path)) {
        throw new Error('Invalid derivation path');
    }
    const { key, chainCode } = (0, exports.getMasterKeyFromSeed)(seed);
    const segments = path
        .split('/')
        .slice(1)
        .map(exports.replaceDerive)
        .map((el) => parseInt(el, 10));
    return segments.reduce((parentKeys, segment) => CKDPriv(parentKeys, segment + offset), {
        key,
        chainCode,
    });
};
exports.derivePath = derivePath;
//# sourceMappingURL=ed25519-hd-key.js.map