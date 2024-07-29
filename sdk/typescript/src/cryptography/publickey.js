"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.PublicKey = void 0;
exports.bytesEqual = bytesEqual;
const bcs_1 = require("@mysten/bcs");
const blake2b_1 = require("@noble/hashes/blake2b");
const utils_1 = require("@noble/hashes/utils");
const index_js_1 = require("../bcs/index.js");
const sui_types_js_1 = require("../utils/sui-types.js");
const intent_js_1 = require("./intent.js");
function bytesEqual(a, b) {
    if (a === b)
        return true;
    if (a.length !== b.length) {
        return false;
    }
    for (let i = 0; i < a.length; i++) {
        if (a[i] !== b[i]) {
            return false;
        }
    }
    return true;
}
/**
 * A public key
 */
class PublicKey {
    /**
     * Checks if two public keys are equal
     */
    equals(publicKey) {
        return bytesEqual(this.toRawBytes(), publicKey.toRawBytes());
    }
    /**
     * Return the base-64 representation of the public key
     */
    toBase64() {
        return (0, bcs_1.toB64)(this.toRawBytes());
    }
    toString() {
        throw new Error('`toString` is not implemented on public keys. Use `toBase64()` or `toRawBytes()` instead.');
    }
    /**
     * Return the Sui representation of the public key encoded in
     * base-64. A Sui public key is formed by the concatenation
     * of the scheme flag with the raw bytes of the public key
     */
    toSuiPublicKey() {
        const bytes = this.toSuiBytes();
        return (0, bcs_1.toB64)(bytes);
    }
    verifyWithIntent(bytes, signature, intent) {
        const intentMessage = (0, intent_js_1.messageWithIntent)(intent, bytes);
        const digest = (0, blake2b_1.blake2b)(intentMessage, { dkLen: 32 });
        return this.verify(digest, signature);
    }
    /**
     * Verifies that the signature is valid for for the provided PersonalMessage
     */
    verifyPersonalMessage(message, signature) {
        return this.verifyWithIntent(index_js_1.bcs.vector(index_js_1.bcs.u8()).serialize(message).toBytes(), signature, intent_js_1.IntentScope.PersonalMessage);
    }
    /**
     * Verifies that the signature is valid for for the provided TransactionBlock
     */
    verifyTransactionBlock(transactionBlock, signature) {
        return this.verifyWithIntent(transactionBlock, signature, intent_js_1.IntentScope.TransactionData);
    }
    /**
     * Returns the bytes representation of the public key
     * prefixed with the signature scheme flag
     */
    toSuiBytes() {
        const rawBytes = this.toRawBytes();
        const suiBytes = new Uint8Array(rawBytes.length + 1);
        suiBytes.set([this.flag()]);
        suiBytes.set(rawBytes, 1);
        return suiBytes;
    }
    /**
     * Return the Sui address associated with this Ed25519 public key
     */
    toSuiAddress() {
        // Each hex char represents half a byte, hence hex address doubles the length
        return (0, sui_types_js_1.normalizeSuiAddress)((0, utils_1.bytesToHex)((0, blake2b_1.blake2b)(this.toSuiBytes(), { dkLen: 32 })).slice(0, sui_types_js_1.SUI_ADDRESS_LENGTH * 2));
    }
}
exports.PublicKey = PublicKey;
//# sourceMappingURL=publickey.js.map