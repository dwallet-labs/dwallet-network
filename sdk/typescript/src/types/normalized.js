"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.SuiMoveNormalizedModules = exports.SuiMoveNormalizedModule = exports.SuiMoveNormalizedStruct = exports.SuiMoveNormalizedField = exports.SuiMoveNormalizedFunction = exports.SuiMoveNormalizedStructType = exports.SuiMoveNormalizedType = exports.MoveCallMetrics = exports.MoveCallMetric = exports.SuiMoveNormalizedTypeParameterType = exports.SuiMoveStructTypeParameter = exports.SuiMoveAbilitySet = exports.SuiMoveVisibility = exports.SuiMoveModuleId = exports.SuiMoveFunctionArgTypes = exports.SuiMoveFunctionArgType = void 0;
exports.extractMutableReference = extractMutableReference;
exports.extractReference = extractReference;
exports.extractStructTag = extractStructTag;
const superstruct_1 = require("superstruct");
exports.SuiMoveFunctionArgType = (0, superstruct_1.union)([(0, superstruct_1.string)(), (0, superstruct_1.object)({ Object: (0, superstruct_1.string)() })]);
exports.SuiMoveFunctionArgTypes = (0, superstruct_1.array)(exports.SuiMoveFunctionArgType);
exports.SuiMoveModuleId = (0, superstruct_1.object)({
    address: (0, superstruct_1.string)(),
    name: (0, superstruct_1.string)(),
});
exports.SuiMoveVisibility = (0, superstruct_1.union)([(0, superstruct_1.literal)('Private'), (0, superstruct_1.literal)('Public'), (0, superstruct_1.literal)('Friend')]);
exports.SuiMoveAbilitySet = (0, superstruct_1.object)({
    abilities: (0, superstruct_1.array)((0, superstruct_1.string)()),
});
exports.SuiMoveStructTypeParameter = (0, superstruct_1.object)({
    constraints: exports.SuiMoveAbilitySet,
    isPhantom: (0, superstruct_1.boolean)(),
});
exports.SuiMoveNormalizedTypeParameterType = (0, superstruct_1.object)({
    TypeParameter: (0, superstruct_1.number)(),
});
exports.MoveCallMetric = (0, superstruct_1.tuple)([
    (0, superstruct_1.object)({
        module: (0, superstruct_1.string)(),
        package: (0, superstruct_1.string)(),
        function: (0, superstruct_1.string)(),
    }),
    (0, superstruct_1.string)(),
]);
exports.MoveCallMetrics = (0, superstruct_1.object)({
    rank3Days: (0, superstruct_1.array)(exports.MoveCallMetric),
    rank7Days: (0, superstruct_1.array)(exports.MoveCallMetric),
    rank30Days: (0, superstruct_1.array)(exports.MoveCallMetric),
});
function isSuiMoveNormalizedType(value) {
    if (!value)
        return false;
    if (typeof value === 'string')
        return true;
    if ((0, superstruct_1.is)(value, exports.SuiMoveNormalizedTypeParameterType))
        return true;
    if (isSuiMoveNormalizedStructType(value))
        return true;
    if (typeof value !== 'object')
        return false;
    const valueProperties = value;
    if ((0, superstruct_1.is)(valueProperties.Reference, exports.SuiMoveNormalizedType))
        return true;
    if ((0, superstruct_1.is)(valueProperties.MutableReference, exports.SuiMoveNormalizedType))
        return true;
    if ((0, superstruct_1.is)(valueProperties.Vector, exports.SuiMoveNormalizedType))
        return true;
    return false;
}
exports.SuiMoveNormalizedType = (0, superstruct_1.define)('SuiMoveNormalizedType', isSuiMoveNormalizedType);
function isSuiMoveNormalizedStructType(value) {
    if (!value || typeof value !== 'object')
        return false;
    const valueProperties = value;
    if (!valueProperties.Struct || typeof valueProperties.Struct !== 'object')
        return false;
    const structProperties = valueProperties.Struct;
    if (typeof structProperties.address !== 'string' ||
        typeof structProperties.module !== 'string' ||
        typeof structProperties.name !== 'string' ||
        !Array.isArray(structProperties.typeArguments) ||
        !structProperties.typeArguments.every((value) => isSuiMoveNormalizedType(value))) {
        return false;
    }
    return true;
}
// NOTE: This type is recursive, so we need to manually implement it:
exports.SuiMoveNormalizedStructType = (0, superstruct_1.define)('SuiMoveNormalizedStructType', isSuiMoveNormalizedStructType);
exports.SuiMoveNormalizedFunction = (0, superstruct_1.object)({
    visibility: exports.SuiMoveVisibility,
    isEntry: (0, superstruct_1.boolean)(),
    typeParameters: (0, superstruct_1.array)(exports.SuiMoveAbilitySet),
    parameters: (0, superstruct_1.array)(exports.SuiMoveNormalizedType),
    return: (0, superstruct_1.array)(exports.SuiMoveNormalizedType),
});
exports.SuiMoveNormalizedField = (0, superstruct_1.object)({
    name: (0, superstruct_1.string)(),
    type: exports.SuiMoveNormalizedType,
});
exports.SuiMoveNormalizedStruct = (0, superstruct_1.object)({
    abilities: exports.SuiMoveAbilitySet,
    typeParameters: (0, superstruct_1.array)(exports.SuiMoveStructTypeParameter),
    fields: (0, superstruct_1.array)(exports.SuiMoveNormalizedField),
});
exports.SuiMoveNormalizedModule = (0, superstruct_1.object)({
    fileFormatVersion: (0, superstruct_1.number)(),
    address: (0, superstruct_1.string)(),
    name: (0, superstruct_1.string)(),
    friends: (0, superstruct_1.array)(exports.SuiMoveModuleId),
    structs: (0, superstruct_1.record)((0, superstruct_1.string)(), exports.SuiMoveNormalizedStruct),
    exposedFunctions: (0, superstruct_1.record)((0, superstruct_1.string)(), exports.SuiMoveNormalizedFunction),
});
exports.SuiMoveNormalizedModules = (0, superstruct_1.record)((0, superstruct_1.string)(), exports.SuiMoveNormalizedModule);
function extractMutableReference(normalizedType) {
    return typeof normalizedType === 'object' && 'MutableReference' in normalizedType
        ? normalizedType.MutableReference
        : undefined;
}
function extractReference(normalizedType) {
    return typeof normalizedType === 'object' && 'Reference' in normalizedType
        ? normalizedType.Reference
        : undefined;
}
function extractStructTag(normalizedType) {
    if (typeof normalizedType === 'object' && 'Struct' in normalizedType) {
        return normalizedType;
    }
    const ref = extractReference(normalizedType);
    const mutRef = extractMutableReference(normalizedType);
    if (typeof ref === 'object' && 'Struct' in ref) {
        return ref;
    }
    if (typeof mutRef === 'object' && 'Struct' in mutRef) {
        return mutRef;
    }
    return undefined;
}
//# sourceMappingURL=normalized.js.map