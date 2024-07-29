"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.Ed25519PublicKey = void 0;
const bcs_1 = require("@mysten/bcs");
const tweetnacl_1 = __importDefault(require("tweetnacl"));
const publickey_js_1 = require("../../cryptography/publickey.js");
const signature_scheme_js_1 = require("../../cryptography/signature-scheme.js");
const signature_js_1 = require("../../cryptography/signature.js");
const PUBLIC_KEY_SIZE = 32;
/**
 * An Ed25519 public key
 */
class Ed25519PublicKey extends publickey_js_1.PublicKey {
    static SIZE = PUBLIC_KEY_SIZE;
    data;
    /**
     * Create a new Ed25519PublicKey object
     * @param value ed25519 public key as buffer or base-64 encoded string
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
        if (this.data.length !== PUBLIC_KEY_SIZE) {
            throw new Error(`Invalid public key input. Expected ${PUBLIC_KEY_SIZE} bytes, got ${this.data.length}`);
        }
    }
    /**
     * Checks if two Ed25519 public keys are equal
     */
    equals(publicKey) {
        return super.equals(publicKey);
    }
    /**
     * Return the byte array representation of the Ed25519 public key
     */
    toRawBytes() {
        return this.data;
    }
    /**
     * Return the Sui address associated with this Ed25519 public key
     */
    flag() {
        return signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG['ED25519'];
    }
    /**
     * Verifies that the signature is valid for for the provided message
     */
    async verify(message, signature) {
        let bytes;
        if (typeof signature === 'string') {
            const parsed = (0, signature_js_1.parseSerializedSignature)(signature);
            if (parsed.signatureScheme !== 'ED25519') {
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
        return tweetnacl_1.default.sign.detached.verify(message, bytes, this.toRawBytes());
    }
}
exports.Ed25519PublicKey = Ed25519PublicKey;
//# sourceMappingURL=publickey.js.map