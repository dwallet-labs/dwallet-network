"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.Transactions = exports.TransactionType = exports.UpgradeTransaction = exports.UpgradePolicy = exports.PublishTransaction = exports.MakeMoveVecTransaction = exports.MergeCoinsTransaction = exports.SplitCoinsTransaction = exports.TransferObjectsTransaction = exports.MoveCallTransaction = exports.TransactionArgument = exports.TransactionBlockInput = void 0;
exports.getTransactionType = getTransactionType;
const bcs_1 = require("@mysten/bcs");
const superstruct_1 = require("superstruct");
const index_js_1 = require("../bcs/index.js");
const type_tag_serializer_js_1 = require("../bcs/type-tag-serializer.js");
const sui_types_js_1 = require("../utils/sui-types.js");
const Inputs_js_1 = require("./Inputs.js");
const utils_js_1 = require("./utils.js");
const option = (some) => (0, superstruct_1.union)([(0, superstruct_1.object)({ None: (0, superstruct_1.union)([(0, superstruct_1.literal)(true), (0, superstruct_1.literal)(null)]) }), (0, superstruct_1.object)({ Some: some })]);
exports.TransactionBlockInput = (0, superstruct_1.union)([
    (0, superstruct_1.object)({
        kind: (0, superstruct_1.literal)('Input'),
        index: (0, superstruct_1.integer)(),
        value: (0, superstruct_1.optional)((0, superstruct_1.any)()),
        type: (0, superstruct_1.optional)((0, superstruct_1.literal)('object')),
    }),
    (0, superstruct_1.object)({
        kind: (0, superstruct_1.literal)('Input'),
        index: (0, superstruct_1.integer)(),
        value: (0, superstruct_1.optional)((0, superstruct_1.any)()),
        type: (0, superstruct_1.literal)('pure'),
    }),
]);
const TransactionArgumentTypes = [
    exports.TransactionBlockInput,
    (0, superstruct_1.object)({ kind: (0, superstruct_1.literal)('GasCoin') }),
    (0, superstruct_1.object)({ kind: (0, superstruct_1.literal)('Result'), index: (0, superstruct_1.integer)() }),
    (0, superstruct_1.object)({
        kind: (0, superstruct_1.literal)('NestedResult'),
        index: (0, superstruct_1.integer)(),
        resultIndex: (0, superstruct_1.integer)(),
    }),
];
// Generic transaction argument
exports.TransactionArgument = (0, superstruct_1.union)([...TransactionArgumentTypes]);
exports.MoveCallTransaction = (0, superstruct_1.object)({
    kind: (0, superstruct_1.literal)('MoveCall'),
    target: (0, superstruct_1.define)('target', (0, superstruct_1.string)().validator),
    typeArguments: (0, superstruct_1.array)((0, superstruct_1.string)()),
    arguments: (0, superstruct_1.array)(exports.TransactionArgument),
});
exports.TransferObjectsTransaction = (0, superstruct_1.object)({
    kind: (0, superstruct_1.literal)('TransferObjects'),
    objects: (0, superstruct_1.array)(exports.TransactionArgument),
    address: exports.TransactionArgument,
});
exports.SplitCoinsTransaction = (0, superstruct_1.object)({
    kind: (0, superstruct_1.literal)('SplitCoins'),
    coin: exports.TransactionArgument,
    amounts: (0, superstruct_1.array)(exports.TransactionArgument),
});
exports.MergeCoinsTransaction = (0, superstruct_1.object)({
    kind: (0, superstruct_1.literal)('MergeCoins'),
    destination: exports.TransactionArgument,
    sources: (0, superstruct_1.array)(exports.TransactionArgument),
});
exports.MakeMoveVecTransaction = (0, superstruct_1.object)({
    kind: (0, superstruct_1.literal)('MakeMoveVec'),
    // TODO: ideally we should use `TypeTag` instead of `record()` here,
    // but TypeTag is recursively defined and it's tricky to define a
    // recursive struct in superstruct
    type: (0, superstruct_1.optional)(option((0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.unknown)()))),
    objects: (0, superstruct_1.array)(exports.TransactionArgument),
});
exports.PublishTransaction = (0, superstruct_1.object)({
    kind: (0, superstruct_1.literal)('Publish'),
    modules: (0, superstruct_1.array)((0, superstruct_1.array)((0, superstruct_1.integer)())),
    dependencies: (0, superstruct_1.array)((0, superstruct_1.string)()),
});
// Keep in sync with constants in
// crates/sui-framework/packages/dwallet-framework/sources/package.move
var UpgradePolicy;
(function (UpgradePolicy) {
    UpgradePolicy[UpgradePolicy["COMPATIBLE"] = 0] = "COMPATIBLE";
    UpgradePolicy[UpgradePolicy["ADDITIVE"] = 128] = "ADDITIVE";
    UpgradePolicy[UpgradePolicy["DEP_ONLY"] = 192] = "DEP_ONLY";
})(UpgradePolicy || (exports.UpgradePolicy = UpgradePolicy = {}));
exports.UpgradeTransaction = (0, superstruct_1.object)({
    kind: (0, superstruct_1.literal)('Upgrade'),
    modules: (0, superstruct_1.array)((0, superstruct_1.array)((0, superstruct_1.integer)())),
    dependencies: (0, superstruct_1.array)((0, superstruct_1.string)()),
    packageId: (0, superstruct_1.string)(),
    ticket: exports.TransactionArgument,
});
const TransactionTypes = [
    exports.MoveCallTransaction,
    exports.TransferObjectsTransaction,
    exports.SplitCoinsTransaction,
    exports.MergeCoinsTransaction,
    exports.PublishTransaction,
    exports.UpgradeTransaction,
    exports.MakeMoveVecTransaction,
];
exports.TransactionType = (0, superstruct_1.union)([...TransactionTypes]);
function getTransactionType(data) {
    (0, superstruct_1.assert)(data, exports.TransactionType);
    return TransactionTypes.find((schema) => (0, superstruct_1.is)(data, schema));
}
/**
 * Simple helpers used to construct transactions:
 */
exports.Transactions = {
    MoveCall(input) {
        return (0, utils_js_1.create)({
            kind: 'MoveCall',
            target: input.target,
            arguments: input.arguments ?? [],
            typeArguments: input.typeArguments ?? [],
        }, exports.MoveCallTransaction);
    },
    TransferObjects(objects, address) {
        if (address.kind === 'Input' && address.type === 'pure' && typeof address.value !== 'object') {
            address.value = Inputs_js_1.Inputs.Pure(index_js_1.bcs.Address.serialize(address.value));
        }
        return (0, utils_js_1.create)({ kind: 'TransferObjects', objects, address }, exports.TransferObjectsTransaction);
    },
    SplitCoins(coin, amounts) {
        // Handle deprecated usage of `Input.Pure(100)`
        amounts.forEach((input) => {
            if (input.kind === 'Input' && input.type === 'pure' && typeof input.value !== 'object') {
                input.value = Inputs_js_1.Inputs.Pure(index_js_1.bcs.U64.serialize(input.value));
            }
        });
        return (0, utils_js_1.create)({
            kind: 'SplitCoins',
            coin,
            amounts,
        }, exports.SplitCoinsTransaction);
    },
    MergeCoins(destination, sources) {
        return (0, utils_js_1.create)({ kind: 'MergeCoins', destination, sources }, exports.MergeCoinsTransaction);
    },
    Publish({ modules, dependencies, }) {
        return (0, utils_js_1.create)({
            kind: 'Publish',
            modules: modules.map((module) => typeof module === 'string' ? Array.from((0, bcs_1.fromB64)(module)) : module),
            dependencies: dependencies.map((dep) => (0, sui_types_js_1.normalizeSuiObjectId)(dep)),
        }, exports.PublishTransaction);
    },
    Upgrade({ modules, dependencies, packageId, ticket, }) {
        return (0, utils_js_1.create)({
            kind: 'Upgrade',
            modules: modules.map((module) => typeof module === 'string' ? Array.from((0, bcs_1.fromB64)(module)) : module),
            dependencies: dependencies.map((dep) => (0, sui_types_js_1.normalizeSuiObjectId)(dep)),
            packageId,
            ticket,
        }, exports.UpgradeTransaction);
    },
    MakeMoveVec({ type, objects, }) {
        return (0, utils_js_1.create)({
            kind: 'MakeMoveVec',
            type: type ? { Some: type_tag_serializer_js_1.TypeTagSerializer.parseFromStr(type) } : { None: null },
            objects,
        }, exports.MakeMoveVecTransaction);
    },
};
//# sourceMappingURL=Transactions.js.map