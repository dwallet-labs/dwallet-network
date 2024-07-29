"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.isValidHardenedPath = isValidHardenedPath;
exports.isValidBIP32Path = isValidBIP32Path;
exports.mnemonicToSeed = mnemonicToSeed;
exports.mnemonicToSeedHex = mnemonicToSeedHex;
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
const bcs_1 = require("@mysten/bcs");
const bip39_1 = require("@scure/bip39");
/**
 * Parse and validate a path that is compliant to SLIP-0010 in form m/44'/784'/{account_index}'/{change_index}'/{address_index}'.
 *
 * @param path path string (e.g. `m/44'/784'/0'/0'/0'`).
 */
function isValidHardenedPath(path) {
    if (!new RegExp("^m\\/44'\\/784'\\/[0-9]+'\\/[0-9]+'\\/[0-9]+'+$").test(path)) {
        return false;
    }
    return true;
}
/**
 * Parse and validate a path that is compliant to BIP-32 in form m/54'/784'/{account_index}'/{change_index}/{address_index}
 * for Secp256k1 and m/74'/784'/{account_index}'/{change_index}/{address_index} for Secp256r1.
 *
 * Note that the purpose for Secp256k1 is registered as 54, to differentiate from Ed25519 with purpose 44.
 *
 * @param path path string (e.g. `m/54'/784'/0'/0/0`).
 */
function isValidBIP32Path(path) {
    if (!new RegExp("^m\\/(54|74)'\\/784'\\/[0-9]+'\\/[0-9]+\\/[0-9]+$").test(path)) {
        return false;
    }
    return true;
}
/**
 * Uses KDF to derive 64 bytes of key data from mnemonic with empty password.
 *
 * @param mnemonics 12 words string split by spaces.
 */
function mnemonicToSeed(mnemonics) {
    return (0, bip39_1.mnemonicToSeedSync)(mnemonics, '');
}
/**
 * Derive the seed in hex format from a 12-word mnemonic string.
 *
 * @param mnemonics 12 words string split by spaces.
 */
function mnemonicToSeedHex(mnemonics) {
    return (0, bcs_1.toHEX)(mnemonicToSeed(mnemonics));
}
//# sourceMappingURL=mnemonics.js.map