"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.ProtocolConfig = exports.SuiJsonValue = exports.ObjectOwner = void 0;
const superstruct_1 = require("superstruct");
exports.ObjectOwner = (0, superstruct_1.union)([
    (0, superstruct_1.object)({
        AddressOwner: (0, superstruct_1.string)(),
    }),
    (0, superstruct_1.object)({
        ObjectOwner: (0, superstruct_1.string)(),
    }),
    (0, superstruct_1.object)({
        Shared: (0, superstruct_1.object)({
            initial_shared_version: (0, superstruct_1.nullable)((0, superstruct_1.string)()),
        }),
    }),
    (0, superstruct_1.literal)('Immutable'),
]);
exports.SuiJsonValue = (0, superstruct_1.define)('SuiJsonValue', () => true);
const ProtocolConfigValue = (0, superstruct_1.union)([
    (0, superstruct_1.object)({ u32: (0, superstruct_1.string)() }),
    (0, superstruct_1.object)({ u64: (0, superstruct_1.string)() }),
    (0, superstruct_1.object)({ f64: (0, superstruct_1.string)() }),
]);
exports.ProtocolConfig = (0, superstruct_1.object)({
    attributes: (0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.nullable)(ProtocolConfigValue)),
    featureFlags: (0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.boolean)()),
    maxSupportedProtocolVersion: (0, superstruct_1.string)(),
    minSupportedProtocolVersion: (0, superstruct_1.string)(),
    protocolVersion: (0, superstruct_1.string)(),
});
//# sourceMappingURL=common.js.map