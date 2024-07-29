"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.SUI_ADDRESS_LENGTH = void 0;
exports.isValidTransactionDigest = isValidTransactionDigest;
exports.isValidSuiAddress = isValidSuiAddress;
exports.isValidSuiObjectId = isValidSuiObjectId;
exports.parseStructTag = parseStructTag;
exports.normalizeStructTag = normalizeStructTag;
exports.normalizeSuiAddress = normalizeSuiAddress;
exports.normalizeSuiObjectId = normalizeSuiObjectId;
const bcs_1 = require("@mysten/bcs");
const TX_DIGEST_LENGTH = 32;
/** Returns whether the tx digest is valid based on the serialization format */
function isValidTransactionDigest(value) {
    try {
        const buffer = (0, bcs_1.fromB58)(value);
        return buffer.length === TX_DIGEST_LENGTH;
    }
    catch (e) {
        return false;
    }
}
// TODO - can we automatically sync this with rust length definition?
// Source of truth is
// https://github.com/MystenLabs/sui/blob/acb2b97ae21f47600e05b0d28127d88d0725561d/crates/sui-types/src/base_types.rs#L67
// which uses the Move account address length
// https://github.com/move-language/move/blob/67ec40dc50c66c34fd73512fcc412f3b68d67235/language/move-core/types/src/account_address.rs#L23 .
exports.SUI_ADDRESS_LENGTH = 32;
function isValidSuiAddress(value) {
    return isHex(value) && getHexByteLength(value) === exports.SUI_ADDRESS_LENGTH;
}
function isValidSuiObjectId(value) {
    return isValidSuiAddress(value);
}
function parseTypeTag(type) {
    if (!type.includes('::'))
        return type;
    return parseStructTag(type);
}
function parseStructTag(type) {
    const [address, module] = type.split('::');
    const rest = type.slice(address.length + module.length + 4);
    const name = rest.includes('<') ? rest.slice(0, rest.indexOf('<')) : rest;
    const typeParams = rest.includes('<')
        ? (0, bcs_1.splitGenericParameters)(rest.slice(rest.indexOf('<') + 1, rest.lastIndexOf('>'))).map((typeParam) => parseTypeTag(typeParam.trim()))
        : [];
    return {
        address: normalizeSuiAddress(address),
        module,
        name,
        typeParams,
    };
}
function normalizeStructTag(type) {
    const { address, module, name, typeParams } = typeof type === 'string' ? parseStructTag(type) : type;
    const formattedTypeParams = typeParams.length > 0
        ? `<${typeParams
            .map((typeParam) => typeof typeParam === 'string' ? typeParam : normalizeStructTag(typeParam))
            .join(',')}>`
        : '';
    return `${address}::${module}::${name}${formattedTypeParams}`;
}
/**
 * Perform the following operations:
 * 1. Make the address lower case
 * 2. Prepend `0x` if the string does not start with `0x`.
 * 3. Add more zeros if the length of the address(excluding `0x`) is less than `SUI_ADDRESS_LENGTH`
 *
 * WARNING: if the address value itself starts with `0x`, e.g., `0x0x`, the default behavior
 * is to treat the first `0x` not as part of the address. The default behavior can be overridden by
 * setting `forceAdd0x` to true
 *
 */
function normalizeSuiAddress(value, forceAdd0x = false) {
    let address = value.toLowerCase();
    if (!forceAdd0x && address.startsWith('0x')) {
        address = address.slice(2);
    }
    return `0x${address.padStart(exports.SUI_ADDRESS_LENGTH * 2, '0')}`;
}
function normalizeSuiObjectId(value, forceAdd0x = false) {
    return normalizeSuiAddress(value, forceAdd0x);
}
function isHex(value) {
    return /^(0x|0X)?[a-fA-F0-9]+$/.test(value) && value.length % 2 === 0;
}
function getHexByteLength(value) {
    return /^(0x|0X)/.test(value) ? (value.length - 2) / 2 : value.length / 2;
}
//# sourceMappingURL=sui-types.js.map