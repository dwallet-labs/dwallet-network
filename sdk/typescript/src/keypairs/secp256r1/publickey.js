"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.Secp256r1PublicKey = void 0;
const bcs_1 = require("@mysten/bcs");
const p256_1 = require("@noble/curves/p256");
const sha256_1 = require("@noble/hashes/sha256");
const publickey_js_1 = require("../../cryptography/publickey.js");
const signature_scheme_js_1 = require("../../cryptography/signature-scheme.js");
const signature_js_1 = require("../../cryptography/signature.js");
const SECP256R1_PUBLIC_KEY_SIZE = 33;
/**
 * A Secp256r1 public key
 */
class Secp256r1PublicKey extends publickey_js_1.PublicKey {
    static SIZE = SECP256R1_PUBLIC_KEY_SIZE;
    data;
    /**
     * Create a new Secp256r1PublicKey object
     * @param value secp256r1 public key as buffer or base-64 encoded string
     */
    constructor(value) {
        super();
        if (typeof value === 'string') {
            this.data = (0, bcs_1.fromB64)(value);
        }
        else if (value instanceof Uint8Array) {
            this.data = value;
        }
        else {
            this.data = Uint8Array.from(value);
        }
        if (this.data.length !== SECP256R1_PUBLIC_KEY_SIZE) {
            throw new Error(`Invalid public key input. Expected ${SECP256R1_PUBLIC_KEY_SIZE} bytes, got ${this.data.length}`);
        }
    }
    /**
     * Checks if two Secp256r1 public keys are equal
     */
    equals(publicKey) {
        return super.equals(publicKey);
    }
    /**
     * Return the byte array representation of the Secp256r1 public key
     */
    toRawBytes() {
        return this.data;
    }
    /**
     * Return the Sui address associated with this Secp256r1 public key
     */
    flag() {
        return signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG['Secp256r1'];
    }
    /**
     * Verifies that the signature is valid for for the provided message
     */
    async verify(message, signature) {
        let bytes;
        if (typeof signature === 'string') {
            const parsed = (0, signature_js_1.parseSerializedSignature)(signature);
            if (parsed.signatureScheme !== 'Secp256r1') {
                throw new Error('Invalid signature scheme');
            }
            if (!(0, publickey_js_1.bytesEqual)(this.toRawBytes(), parsed.publicKey)) {
                throw new Error('Signature does not match public key');
            }
            bytes = parsed.signature;
        }
        else {
            bytes = signature;
        }
        return p256_1.secp256r1.verify(p256_1.secp256r1.Signature.fromCompact(bytes), (0, sha256_1.sha256)(message), this.toRawBytes());
    }
}
exports.Secp256r1PublicKey = Secp256r1PublicKey;
//# sourceMappingURL=publickey.js.map