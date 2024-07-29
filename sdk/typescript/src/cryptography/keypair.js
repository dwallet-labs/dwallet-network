"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.Keypair = exports.BaseSigner = exports.LEGACY_PRIVATE_KEY_SIZE = exports.PRIVATE_KEY_SIZE = void 0;
const bcs_1 = require("@mysten/bcs");
const blake2b_1 = require("@noble/hashes/blake2b");
const index_js_1 = require("../bcs/index.js");
const intent_js_1 = require("./intent.js");
const signature_js_1 = require("./signature.js");
exports.PRIVATE_KEY_SIZE = 32;
exports.LEGACY_PRIVATE_KEY_SIZE = 64;
/**
 * TODO: Document
 */
class BaseSigner {
    /**
     * Sign messages with a specific intent. By combining the message bytes with the intent before hashing and signing,
     * it ensures that a signed message is tied to a specific purpose and domain separator is provided
     */
    async signWithIntent(bytes, intent) {
        const intentMessage = (0, intent_js_1.messageWithIntent)(intent, bytes);
        const digest = (0, blake2b_1.blake2b)(intentMessage, { dkLen: 32 });
        const signature = (0, signature_js_1.toSerializedSignature)({
            signature: await this.sign(digest),
            signatureScheme: this.getKeyScheme(),
            publicKey: this.getPublicKey(),
        });
        return {
            signature,
            bytes: (0, bcs_1.toB64)(bytes),
        };
    }
    /**
     * Signs provided transaction block by calling `signWithIntent()` with a `TransactionData` provided as intent scope
     */
    async signTransactionBlock(bytes) {
        return this.signWithIntent(bytes, intent_js_1.IntentScope.TransactionData);
    }
    /**
     * Signs provided personal message by calling `signWithIntent()` with a `PersonalMessage` provided as intent scope
     */
    async signPersonalMessage(bytes) {
        return this.signWithIntent(index_js_1.bcs.vector(index_js_1.bcs.u8()).serialize(bytes).toBytes(), intent_js_1.IntentScope.PersonalMessage);
    }
    toSuiAddress() {
        return this.getPublicKey().toSuiAddress();
    }
}
exports.BaseSigner = BaseSigner;
/**
 * TODO: Document
 */
class Keypair extends BaseSigner {
}
exports.Keypair = Keypair;
//# sourceMappingURL=keypair.js.map