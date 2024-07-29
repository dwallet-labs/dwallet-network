"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.ZkLoginPublicIdentifier = void 0;
exports.toZkLoginPublicIdentifier = toZkLoginPublicIdentifier;
const bcs_1 = require("@mysten/bcs");
const publickey_js_1 = require("../cryptography/publickey.js");
const signature_scheme_js_1 = require("../cryptography/signature-scheme.js");
const utils_js_1 = require("./utils.js");
/**
 * A zkLogin public identifier
 */
class ZkLoginPublicIdentifier extends publickey_js_1.PublicKey {
    data;
    /**
     * Create a new ZkLoginPublicIdentifier object
     * @param value zkLogin public identifier as buffer or base-64 encoded string
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
    }
    /**
     * Checks if two zkLogin public identifiers are equal
     */
    equals(publicKey) {
        return super.equals(publicKey);
    }
    /**
     * Return the byte array representation of the zkLogin public identifier
     */
    toRawBytes() {
        return this.data;
    }
    /**
     * Return the Sui address associated with this ZkLogin public identifier
     */
    flag() {
        return signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG['ZkLogin'];
    }
    /**
     * Verifies that the signature is valid for for the provided message
     */
    async verify(_message, _signature) {
        throw Error('does not support');
    }
}
exports.ZkLoginPublicIdentifier = ZkLoginPublicIdentifier;
// Derive the public identifier for zklogin based on address seed and iss.
function toZkLoginPublicIdentifier(addressSeed, iss) {
    // Consists of iss_bytes_len || iss_bytes || padded_32_byte_address_seed.
    const addressSeedBytesBigEndian = (0, utils_js_1.toPaddedBigEndianBytes)(addressSeed, 32);
    const issBytes = new TextEncoder().encode(iss);
    const tmp = new Uint8Array(1 + issBytes.length + addressSeedBytesBigEndian.length);
    tmp.set([issBytes.length], 0);
    tmp.set(issBytes, 1);
    tmp.set(addressSeedBytesBigEndian, 1 + issBytes.length);
    return new ZkLoginPublicIdentifier(tmp);
}
//# sourceMappingURL=publickey.js.map