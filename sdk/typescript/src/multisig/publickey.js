"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.MultiSigPublicKey = exports.MIN_SIGNER_IN_MULTISIG = exports.MAX_SIGNER_IN_MULTISIG = void 0;
exports.parsePartialSignatures = parsePartialSignatures;
const bcs_1 = require("@mysten/bcs");
const blake2b_1 = require("@noble/hashes/blake2b");
const utils_1 = require("@noble/hashes/utils");
const index_js_1 = require("../bcs/index.js");
const publickey_js_1 = require("../cryptography/publickey.js");
const signature_scheme_js_1 = require("../cryptography/signature-scheme.js");
const signature_js_1 = require("../cryptography/signature.js");
const sui_types_js_1 = require("../utils/sui-types.js");
// eslint-disable-next-line import/no-cycle
const index_js_2 = require("../verify/index.js");
const publickey_js_2 = require("../zklogin/publickey.js");
exports.MAX_SIGNER_IN_MULTISIG = 10;
exports.MIN_SIGNER_IN_MULTISIG = 1;
/**
 * A MultiSig public key
 */
class MultiSigPublicKey extends publickey_js_1.PublicKey {
    rawBytes;
    multisigPublicKey;
    publicKeys;
    /**
     * Create a new MultiSigPublicKey object
     */
    constructor(
    /**
     *  MultiSig public key as buffer or base-64 encoded string
     */
    value) {
        super();
        if (typeof value === 'string') {
            this.rawBytes = (0, bcs_1.fromB64)(value);
            this.multisigPublicKey = index_js_1.bcs.MultiSigPublicKey.parse(this.rawBytes);
        }
        else if (value instanceof Uint8Array) {
            this.rawBytes = value;
            this.multisigPublicKey = index_js_1.bcs.MultiSigPublicKey.parse(this.rawBytes);
        }
        else {
            this.multisigPublicKey = value;
            this.rawBytes = index_js_1.bcs.MultiSigPublicKey.serialize(value).toBytes();
        }
        if (this.multisigPublicKey.threshold < 1) {
            throw new Error('Invalid threshold');
        }
        const seenPublicKeys = new Set();
        this.publicKeys = this.multisigPublicKey.pk_map.map(({ pubKey, weight }) => {
            const [scheme, bytes] = Object.entries(pubKey)[0];
            const publicKeyStr = Uint8Array.from(bytes).toString();
            if (seenPublicKeys.has(publicKeyStr)) {
                throw new Error(`Multisig does not support duplicate public keys`);
            }
            seenPublicKeys.add(publicKeyStr);
            if (weight < 1) {
                throw new Error(`Invalid weight`);
            }
            return {
                publicKey: (0, index_js_2.publicKeyFromRawBytes)(scheme, Uint8Array.from(bytes)),
                weight,
            };
        });
        const totalWeight = this.publicKeys.reduce((sum, { weight }) => sum + weight, 0);
        if (this.multisigPublicKey.threshold > totalWeight) {
            throw new Error(`Unreachable threshold`);
        }
        if (this.publicKeys.length > exports.MAX_SIGNER_IN_MULTISIG) {
            throw new Error(`Max number of signers in a multisig is ${exports.MAX_SIGNER_IN_MULTISIG}`);
        }
        if (this.publicKeys.length < exports.MIN_SIGNER_IN_MULTISIG) {
            throw new Error(`Min number of signers in a multisig is ${exports.MIN_SIGNER_IN_MULTISIG}`);
        }
    }
    /**
     * 	A static method to create a new MultiSig publickey instance from a set of public keys and their associated weights pairs and threshold.
     */
    static fromPublicKeys({ threshold, publicKeys, }) {
        return new MultiSigPublicKey({
            pk_map: publicKeys.map(({ publicKey, weight }) => {
                const scheme = signature_scheme_js_1.SIGNATURE_FLAG_TO_SCHEME[publicKey.flag()];
                return {
                    pubKey: { [scheme]: Array.from(publicKey.toRawBytes()) },
                    weight,
                };
            }),
            threshold,
        });
    }
    /**
     * Checks if two MultiSig public keys are equal
     */
    equals(publicKey) {
        return super.equals(publicKey);
    }
    /**
     * Return the byte array representation of the MultiSig public key
     */
    toRawBytes() {
        return this.rawBytes;
    }
    getPublicKeys() {
        return this.publicKeys;
    }
    /**
     * Return the Sui address associated with this MultiSig public key
     */
    toSuiAddress() {
        // max length = 1 flag byte + (max pk size + max weight size (u8)) * max signer size + 2 threshold bytes (u16)
        const maxLength = 1 + (64 + 1) * exports.MAX_SIGNER_IN_MULTISIG + 2;
        const tmp = new Uint8Array(maxLength);
        tmp.set([signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG['MultiSig']]);
        tmp.set(index_js_1.bcs.u16().serialize(this.multisigPublicKey.threshold).toBytes(), 1);
        // The initial value 3 ensures that following data will be after the flag byte and threshold bytes
        let i = 3;
        for (const { publicKey, weight } of this.publicKeys) {
            const bytes = publicKey.toSuiBytes();
            tmp.set(bytes, i);
            i += bytes.length;
            tmp.set([weight], i++);
        }
        return (0, sui_types_js_1.normalizeSuiAddress)((0, utils_1.bytesToHex)((0, blake2b_1.blake2b)(tmp.slice(0, i), { dkLen: 32 })));
    }
    /**
     * Return the Sui address associated with this MultiSig public key
     */
    flag() {
        return signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG['MultiSig'];
    }
    /**
     * Verifies that the signature is valid for for the provided message
     */
    async verify(message, multisigSignature) {
        // Multisig verification only supports serialized signature
        const { signatureScheme, multisig } = (0, signature_js_1.parseSerializedSignature)(multisigSignature);
        if (signatureScheme !== 'MultiSig') {
            throw new Error('Invalid signature scheme');
        }
        let signatureWeight = 0;
        if (!(0, publickey_js_1.bytesEqual)(index_js_1.bcs.MultiSigPublicKey.serialize(this.multisigPublicKey).toBytes(), index_js_1.bcs.MultiSigPublicKey.serialize(multisig.multisig_pk).toBytes())) {
            return false;
        }
        for (const { publicKey, weight, signature } of parsePartialSignatures(multisig)) {
            if (!(await publicKey.verify(message, signature))) {
                return false;
            }
            signatureWeight += weight;
        }
        return signatureWeight >= this.multisigPublicKey.threshold;
    }
    /**
     * Combines multiple partial signatures into a single multisig, ensuring that each public key signs only once
     * and that all the public keys involved are known and valid, and then serializes multisig into the standard format
     */
    combinePartialSignatures(signatures) {
        if (signatures.length > exports.MAX_SIGNER_IN_MULTISIG) {
            throw new Error(`Max number of signatures in a multisig is ${exports.MAX_SIGNER_IN_MULTISIG}`);
        }
        let bitmap = 0;
        const compressedSignatures = new Array(signatures.length);
        for (let i = 0; i < signatures.length; i++) {
            let parsed = (0, signature_js_1.parseSerializedSignature)(signatures[i]);
            if (parsed.signatureScheme === 'MultiSig') {
                throw new Error('MultiSig is not supported inside MultiSig');
            }
            let publicKey;
            if (parsed.signatureScheme === 'ZkLogin') {
                publicKey = (0, publickey_js_2.toZkLoginPublicIdentifier)(parsed.zkLogin?.addressSeed, parsed.zkLogin?.iss).toRawBytes();
            }
            else {
                publicKey = parsed.publicKey;
            }
            compressedSignatures[i] = {
                [parsed.signatureScheme]: Array.from(parsed.signature.map((x) => Number(x))),
            };
            let publicKeyIndex;
            for (let j = 0; j < this.publicKeys.length; j++) {
                if ((0, publickey_js_1.bytesEqual)(publicKey, this.publicKeys[j].publicKey.toRawBytes())) {
                    if (bitmap & (1 << j)) {
                        throw new Error('Received multiple signatures from the same public key');
                    }
                    publicKeyIndex = j;
                    break;
                }
            }
            if (publicKeyIndex === undefined) {
                throw new Error('Received signature from unknown public key');
            }
            bitmap |= 1 << publicKeyIndex;
        }
        let multisig = {
            sigs: compressedSignatures,
            bitmap,
            multisig_pk: this.multisigPublicKey,
        };
        const bytes = index_js_1.bcs.MultiSig.serialize(multisig, { maxSize: 8192 }).toBytes();
        let tmp = new Uint8Array(bytes.length + 1);
        tmp.set([signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG['MultiSig']]);
        tmp.set(bytes, 1);
        return (0, bcs_1.toB64)(tmp);
    }
}
exports.MultiSigPublicKey = MultiSigPublicKey;
/**
 * Parse multisig structure into an array of individual signatures: signature scheme, the actual individual signature, public key and its weight.
 */
function parsePartialSignatures(multisig) {
    let res = new Array(multisig.sigs.length);
    for (let i = 0; i < multisig.sigs.length; i++) {
        const [signatureScheme, signature] = Object.entries(multisig.sigs[i])[0];
        const pkIndex = asIndices(multisig.bitmap).at(i);
        const pair = multisig.multisig_pk.pk_map[pkIndex];
        const pkBytes = Uint8Array.from(Object.values(pair.pubKey)[0]);
        if (signatureScheme === 'MultiSig') {
            throw new Error('MultiSig is not supported inside MultiSig');
        }
        const publicKey = (0, index_js_2.publicKeyFromRawBytes)(signatureScheme, pkBytes);
        res[i] = {
            signatureScheme,
            signature: Uint8Array.from(signature),
            publicKey: publicKey,
            weight: pair.weight,
        };
    }
    return res;
}
function asIndices(bitmap) {
    if (bitmap < 0 || bitmap > 1024) {
        throw new Error('Invalid bitmap');
    }
    let res = [];
    for (let i = 0; i < 10; i++) {
        if ((bitmap & (1 << i)) !== 0) {
            res.push(i);
        }
    }
    return Uint8Array.from(res);
}
//# sourceMappingURL=publickey.js.map