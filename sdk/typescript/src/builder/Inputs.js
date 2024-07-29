"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.Inputs = exports.BuilderCallArg = exports.ObjectCallArg = exports.PureCallArg = void 0;
exports.getIdFromCallArg = getIdFromCallArg;
exports.getSharedObjectInput = getSharedObjectInput;
exports.isSharedObjectInput = isSharedObjectInput;
exports.isMutableSharedObjectInput = isMutableSharedObjectInput;
const bcs_1 = require("@mysten/bcs");
const superstruct_1 = require("superstruct");
const index_js_1 = require("../bcs/index.js");
const index_js_2 = require("../types/index.js");
const sui_types_js_1 = require("../utils/sui-types.js");
const ObjectArg = (0, superstruct_1.union)([
    (0, superstruct_1.object)({ ImmOrOwned: index_js_2.SuiObjectRef }),
    (0, superstruct_1.object)({
        Shared: (0, superstruct_1.object)({
            objectId: (0, superstruct_1.string)(),
            initialSharedVersion: (0, superstruct_1.union)([(0, superstruct_1.integer)(), (0, superstruct_1.string)()]),
            mutable: (0, superstruct_1.boolean)(),
        }),
    }),
    (0, superstruct_1.object)({ Receiving: index_js_2.SuiObjectRef }),
]);
exports.PureCallArg = (0, superstruct_1.object)({ Pure: (0, superstruct_1.array)((0, superstruct_1.integer)()) });
exports.ObjectCallArg = (0, superstruct_1.object)({ Object: ObjectArg });
exports.BuilderCallArg = (0, superstruct_1.union)([exports.PureCallArg, exports.ObjectCallArg]);
function Pure(data, type) {
    return {
        Pure: Array.from(data instanceof Uint8Array
            ? data
            : (0, bcs_1.isSerializedBcs)(data)
                ? data.toBytes()
                : // NOTE: We explicitly set this to be growable to infinity, because we have maxSize validation at the builder-level:
                    index_js_1.bcs.ser(type, data, { maxSize: Infinity }).toBytes()),
    };
}
exports.Inputs = {
    Pure,
    ObjectRef({ objectId, digest, version }) {
        return {
            Object: {
                ImmOrOwned: {
                    digest,
                    version,
                    objectId: (0, sui_types_js_1.normalizeSuiAddress)(objectId),
                },
            },
        };
    },
    SharedObjectRef({ objectId, mutable, initialSharedVersion }) {
        return {
            Object: {
                Shared: {
                    mutable,
                    initialSharedVersion,
                    objectId: (0, sui_types_js_1.normalizeSuiAddress)(objectId),
                },
            },
        };
    },
    ReceivingRef({ objectId, digest, version }) {
        return {
            Object: {
                Receiving: {
                    digest,
                    version,
                    objectId: (0, sui_types_js_1.normalizeSuiAddress)(objectId),
                },
            },
        };
    },
};
function getIdFromCallArg(arg) {
    if (typeof arg === 'string') {
        return (0, sui_types_js_1.normalizeSuiAddress)(arg);
    }
    if ('ImmOrOwned' in arg.Object) {
        return (0, sui_types_js_1.normalizeSuiAddress)(arg.Object.ImmOrOwned.objectId);
    }
    if ('Receiving' in arg.Object) {
        return (0, sui_types_js_1.normalizeSuiAddress)(arg.Object.Receiving.objectId);
    }
    return (0, sui_types_js_1.normalizeSuiAddress)(arg.Object.Shared.objectId);
}
function getSharedObjectInput(arg) {
    return typeof arg === 'object' && 'Object' in arg && 'Shared' in arg.Object
        ? arg.Object.Shared
        : undefined;
}
function isSharedObjectInput(arg) {
    return !!getSharedObjectInput(arg);
}
function isMutableSharedObjectInput(arg) {
    return getSharedObjectInput(arg)?.mutable ?? false;
}
//# sourceMappingURL=Inputs.js.map