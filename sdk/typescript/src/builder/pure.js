"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.createPure = createPure;
const index_js_1 = require("../bcs/index.js");
function createPure(makePure) {
    function pure(value, type) {
        return makePure(value, type);
    }
    pure.u8 = (value) => makePure(index_js_1.bcs.U8.serialize(value));
    pure.u16 = (value) => makePure(index_js_1.bcs.U16.serialize(value));
    pure.u32 = (value) => makePure(index_js_1.bcs.U32.serialize(value));
    pure.u64 = (value) => makePure(index_js_1.bcs.U64.serialize(value));
    pure.u128 = (value) => makePure(index_js_1.bcs.U128.serialize(value));
    pure.u256 = (value) => makePure(index_js_1.bcs.U256.serialize(value));
    pure.bool = (value) => makePure(index_js_1.bcs.Bool.serialize(value));
    pure.string = (value) => makePure(index_js_1.bcs.String.serialize(value));
    pure.address = (value) => makePure(index_js_1.bcs.Address.serialize(value));
    pure.id = pure.address;
    return pure;
}
//# sourceMappingURL=pure.js.map