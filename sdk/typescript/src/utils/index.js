"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.SUI_SYSTEM_STATE_OBJECT_ID = exports.SUI_TYPE_ARG = exports.SUI_SYSTEM_MODULE_NAME = exports.SUI_CLOCK_OBJECT_ID = exports.SUI_SYSTEM_ADDRESS = exports.SUI_FRAMEWORK_ADDRESS = exports.MOVE_STDLIB_ADDRESS = exports.MIST_PER_SUI = exports.SUI_DECIMALS = exports.SUI_ADDRESS_LENGTH = exports.parseStructTag = exports.normalizeSuiObjectId = exports.normalizeSuiAddress = exports.normalizeStructTag = exports.isValidTransactionDigest = exports.isValidSuiObjectId = exports.isValidSuiAddress = exports.formatDigest = exports.formatAddress = exports.assert = exports.is = exports.toHEX = exports.fromHEX = exports.toB64 = exports.fromB64 = void 0;
const format_js_1 = require("./format.js");
Object.defineProperty(exports, "formatAddress", { enumerable: true, get: function () { return format_js_1.formatAddress; } });
Object.defineProperty(exports, "formatDigest", { enumerable: true, get: function () { return format_js_1.formatDigest; } });
const sui_types_js_1 = require("./sui-types.js");
Object.defineProperty(exports, "isValidSuiAddress", { enumerable: true, get: function () { return sui_types_js_1.isValidSuiAddress; } });
Object.defineProperty(exports, "isValidSuiObjectId", { enumerable: true, get: function () { return sui_types_js_1.isValidSuiObjectId; } });
Object.defineProperty(exports, "isValidTransactionDigest", { enumerable: true, get: function () { return sui_types_js_1.isValidTransactionDigest; } });
Object.defineProperty(exports, "normalizeStructTag", { enumerable: true, get: function () { return sui_types_js_1.normalizeStructTag; } });
Object.defineProperty(exports, "normalizeSuiAddress", { enumerable: true, get: function () { return sui_types_js_1.normalizeSuiAddress; } });
Object.defineProperty(exports, "normalizeSuiObjectId", { enumerable: true, get: function () { return sui_types_js_1.normalizeSuiObjectId; } });
Object.defineProperty(exports, "parseStructTag", { enumerable: true, get: function () { return sui_types_js_1.parseStructTag; } });
Object.defineProperty(exports, "SUI_ADDRESS_LENGTH", { enumerable: true, get: function () { return sui_types_js_1.SUI_ADDRESS_LENGTH; } });
var bcs_1 = require("@mysten/bcs");
Object.defineProperty(exports, "fromB64", { enumerable: true, get: function () { return bcs_1.fromB64; } });
Object.defineProperty(exports, "toB64", { enumerable: true, get: function () { return bcs_1.toB64; } });
Object.defineProperty(exports, "fromHEX", { enumerable: true, get: function () { return bcs_1.fromHEX; } });
Object.defineProperty(exports, "toHEX", { enumerable: true, get: function () { return bcs_1.toHEX; } });
var superstruct_1 = require("superstruct");
Object.defineProperty(exports, "is", { enumerable: true, get: function () { return superstruct_1.is; } });
Object.defineProperty(exports, "assert", { enumerable: true, get: function () { return superstruct_1.assert; } });
exports.SUI_DECIMALS = 9;
exports.MIST_PER_SUI = BigInt(1000000000);
exports.MOVE_STDLIB_ADDRESS = '0x1';
exports.SUI_FRAMEWORK_ADDRESS = '0x2';
exports.SUI_SYSTEM_ADDRESS = '0x3';
exports.SUI_CLOCK_OBJECT_ID = (0, sui_types_js_1.normalizeSuiObjectId)('0x6');
exports.SUI_SYSTEM_MODULE_NAME = 'sui_system';
exports.SUI_TYPE_ARG = `${exports.SUI_FRAMEWORK_ADDRESS}::dwlt::DWLT`;
exports.SUI_SYSTEM_STATE_OBJECT_ID = (0, sui_types_js_1.normalizeSuiObjectId)('0x5');
//# sourceMappingURL=index.js.map