"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.toSerializedSignature = toSerializedSignature;
exports.parseSerializedSignature = parseSerializedSignature;
const bcs_1 = require("@mysten/bcs");
const index_js_1 = require("../bcs/index.js");
const address_js_1 = require("../zklogin/address.js");
const jwt_utils_js_1 = require("../zklogin/jwt-utils.js");
const signature_js_1 = require("../zklogin/signature.js");
const signature_scheme_js_1 = require("./signature-scheme.js");
/**
 * Takes in a signature, its associated signing scheme and a public key, then serializes this data
 */
function toSerializedSignature({ signature, signatureScheme, publicKey, }) {
    if (!publicKey) {
        throw new Error('`publicKey` is required');
    }
    const pubKeyBytes = publicKey.toRawBytes();
    const serializedSignature = new Uint8Array(1 + signature.length + pubKeyBytes.length);
    serializedSignature.set([signature_scheme_js_1.SIGNATURE_SCHEME_TO_FLAG[signatureScheme]]);
    serializedSignature.set(signature, 1);
    serializedSignature.set(pubKeyBytes, 1 + signature.length);
    return (0, bcs_1.toB64)(serializedSignature);
}
/**
 * Decodes a serialized signature into its constituent components: the signature scheme, the actual signature, and the public key
 */
function parseSerializedSignature(serializedSignature) {
    const bytes = (0, bcs_1.fromB64)(serializedSignature);
    const signatureScheme = signature_scheme_js_1.SIGNATURE_FLAG_TO_SCHEME[bytes[0]];
    switch (signatureScheme) {
        case 'MultiSig':
            const multisig = index_js_1.bcs.MultiSig.parse(bytes.slice(1));
            return {
                serializedSignature,
                signatureScheme,
                multisig,
                bytes,
            };
        case 'ZkLogin':
            const signatureBytes = bytes.slice(1);
            const { inputs, maxEpoch, userSignature } = (0, signature_js_1.parseZkLoginSignature)(signatureBytes);
            const { issBase64Details, addressSeed } = inputs;
            const iss = (0, jwt_utils_js_1.extractClaimValue)(issBase64Details, 'iss');
            const address = (0, address_js_1.computeZkLoginAddressFromSeed)(BigInt(addressSeed), iss);
            return {
                serializedSignature,
                signatureScheme,
                zkLogin: {
                    inputs,
                    maxEpoch,
                    userSignature,
                    iss,
                    address,
                    addressSeed: BigInt(addressSeed),
                },
                signature: bytes,
            };
        case 'ED25519':
        case 'Secp256k1':
        case 'Secp256r1':
            const size = signature_scheme_js_1.SIGNATURE_SCHEME_TO_SIZE[signatureScheme];
            const signature = bytes.slice(1, bytes.length - size);
            const publicKey = bytes.slice(1 + signature.length);
            return {
                serializedSignature,
                signatureScheme,
                signature,
                publicKey,
                bytes,
            };
        default:
            throw new Error('Unsupported signature scheme');
    }
}
//# sourceMappingURL=signature.js.map