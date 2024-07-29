"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.PROGRAMMABLE_CALL = exports.ARGUMENT = exports.TRANSACTION = exports.ENUM_KIND = exports.MULTISIG = exports.MULTISIG_PK_MAP = exports.MULTISIG_PUBLIC_KEY = exports.PUBLIC_KEY = exports.COMPRESSED_SIGNATURE = exports.TRANSACTION_INNER = exports.PROGRAMMABLE_CALL_INNER = exports.PROGRAMMABLE_TX_BLOCK = exports.OBJECT_ARG = exports.TYPE_TAG = exports.CALL_ARG = exports.OPTION = exports.VECTOR = exports.ARGUMENT_INNER = void 0;
exports.ARGUMENT_INNER = 'Argument';
exports.VECTOR = 'vector';
exports.OPTION = 'Option';
exports.CALL_ARG = 'CallArg';
exports.TYPE_TAG = 'TypeTag';
exports.OBJECT_ARG = 'ObjectArg';
exports.PROGRAMMABLE_TX_BLOCK = 'ProgrammableTransaction';
exports.PROGRAMMABLE_CALL_INNER = 'ProgrammableMoveCall';
exports.TRANSACTION_INNER = 'Transaction';
exports.COMPRESSED_SIGNATURE = 'CompressedSignature';
exports.PUBLIC_KEY = 'PublicKey';
exports.MULTISIG_PUBLIC_KEY = 'MultiSigPublicKey';
exports.MULTISIG_PK_MAP = 'MultiSigPkMap';
exports.MULTISIG = 'MultiSig';
exports.ENUM_KIND = 'EnumKind';
/** Wrapper around transaction Enum to support `kind` matching in TS */
exports.TRANSACTION = exports.TRANSACTION_INNER;
/** Wrapper around Argument Enum to support `kind` matching in TS */
exports.ARGUMENT = exports.ARGUMENT_INNER;
/** Custom serializer for decoding package, module, function easier */
exports.PROGRAMMABLE_CALL = 'ProgrammableMoveCall';
//# sourceMappingURL=bcs.js.map