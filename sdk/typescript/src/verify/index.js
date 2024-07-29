"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.verifySignature = verifySignature;
exports.verifyPersonalMessage = verifyPersonalMessage;
exports.verifyTransactionBlock = verifyTransactionBlock;
exports.publicKeyFromRawBytes = publicKeyFromRawBytes;
const index_js_1 = require("../cryptography/index.js");
const publickey_js_1 = require("../keypairs/ed25519/publickey.js");
const publickey_js_2 = require("../keypairs/secp256k1/publickey.js");
const publickey_js_3 = require("../keypairs/secp256r1/publickey.js");
// eslint-disable-next-line import/no-cycle
const publickey_js_4 = require("../multisig/publickey.js");
const publickey_js_5 = require("../zklogin/publickey.js");
async function verifySignature(bytes, signature) {
    const parsedSignature = parseSignature(signature);
    if (!(await parsedSignature.publicKey.verify(bytes, parsedSignature.serializedSignature))) {
        throw new Error(`Signature is not valid for the provided data`);
    }
    return parsedSignature.publicKey;
}
async function verifyPersonalMessage(message, signature) {
    const parsedSignature = parseSignature(signature);
    if (!(await parsedSignature.publicKey.verifyPersonalMessage(message, parsedSignature.serializedSignature))) {
        throw new Error(`Signature is not valid for the provided message`);
    }
    return parsedSignature.publicKey;
}
async function verifyTransactionBlock(transactionBlock, signature) {
    const parsedSignature = parseSignature(signature);
    if (!(await parsedSignature.publicKey.verifyTransactionBlock(transactionBlock, parsedSignature.serializedSignature))) {
        throw new Error(`Signature is not valid for the provided TransactionBlock`);
    }
    return parsedSignature.publicKey;
}
function parseSignature(signature) {
    const parsedSignature = (0, index_js_1.parseSerializedSignature)(signature);
    if (parsedSignature.signatureScheme === 'MultiSig') {
        return {
            ...parsedSignature,
            publicKey: new publickey_js_4.MultiSigPublicKey(parsedSignature.multisig.multisig_pk),
        };
    }
    if (parsedSignature.signatureScheme === 'ZkLogin') {
        throw new Error('ZkLogin is not supported yet');
    }
    const publicKey = publicKeyFromRawBytes(parsedSignature.signatureScheme, parsedSignature.publicKey);
    return {
        ...parsedSignature,
        publicKey,
    };
}
function publicKeyFromRawBytes(signatureScheme, bytes) {
    switch (signatureScheme) {
        case 'ED25519':
            return new publickey_js_1.Ed25519PublicKey(bytes);
        case 'Secp256k1':
            return new publickey_js_2.Secp256k1PublicKey(bytes);
        case 'Secp256r1':
            return new publickey_js_3.Secp256r1PublicKey(bytes);
        case 'MultiSig':
            return new publickey_js_4.MultiSigPublicKey(bytes);
        case 'ZkLogin':
            return new publickey_js_5.ZkLoginPublicIdentifier(bytes);
        default:
            throw new Error(`Unsupported signature scheme ${signatureScheme}`);
    }
}
//# sourceMappingURL=index.js.map