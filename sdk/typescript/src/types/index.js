"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.SuiMoveNormalizedFunction = exports.SuiMoveNormalizedField = exports.SuiMoveModuleId = exports.SuiMoveFunctionArgTypes = exports.SuiMoveFunctionArgType = exports.SuiMoveAbilitySet = exports.MoveCallMetrics = exports.MoveCallMetric = exports.isSuiObjectResponse = exports.isSharedObject = exports.isImmutableObject = exports.hasPublicTransfer = exports.getSuiObjectData = exports.getSharedObjectInitialVersion = exports.getObjectVersion = exports.getObjectType = exports.getObjectReference = exports.getObjectPreviousTransactionDigest = exports.getObjectOwner = exports.getObjectNotExistsResponse = exports.getObjectId = exports.getObjectFields = exports.getObjectDisplay = exports.getObjectDeletedResponse = exports.getMovePackageContent = exports.getMoveObjectType = exports.getMoveObject = exports.SuiRawMovePackage = exports.SuiRawMoveObject = exports.SuiRawData = exports.SuiParsedData = exports.SuiObjectResponseError = exports.SuiObjectResponse = exports.SuiObjectRef = exports.SuiObjectInfo = exports.SuiObjectDataOptions = exports.SuiObjectData = exports.SuiMovePackage = exports.SuiMoveObject = exports.SuiGasData = exports.PaginatedObjectsResponse = exports.ObjectType = exports.ObjectStatus = exports.ObjectRead = exports.ObjectContentFields = exports.MovePackageContent = exports.GetOwnedObjectsResponse = exports.DisplayFieldsResponse = exports.DisplayFieldsBackwardCompatibleResponse = exports.CheckpointedObjectId = void 0;
exports.extractStructTag = exports.extractReference = exports.extractMutableReference = exports.SuiMoveVisibility = exports.SuiMoveStructTypeParameter = exports.SuiMoveNormalizedTypeParameterType = exports.SuiMoveNormalizedType = exports.SuiMoveNormalizedStructType = exports.SuiMoveNormalizedStruct = exports.SuiMoveNormalizedModules = exports.SuiMoveNormalizedModule = void 0;
var objects_js_1 = require("./objects.js");
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "CheckpointedObjectId", { enumerable: true, get: function () { return objects_js_1.CheckpointedObjectId; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "DisplayFieldsBackwardCompatibleResponse", { enumerable: true, get: function () { return objects_js_1.DisplayFieldsBackwardCompatibleResponse; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "DisplayFieldsResponse", { enumerable: true, get: function () { return objects_js_1.DisplayFieldsResponse; } });
/** @deprecated This type will be removed in a future version */
Object.defineProperty(exports, "GetOwnedObjectsResponse", { enumerable: true, get: function () { return objects_js_1.GetOwnedObjectsResponse; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "MovePackageContent", { enumerable: true, get: function () { return objects_js_1.MovePackageContent; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "ObjectContentFields", { enumerable: true, get: function () { return objects_js_1.ObjectContentFields; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "ObjectRead", { enumerable: true, get: function () { return objects_js_1.ObjectRead; } });
/** @deprecated This type will be removed in a future version */
Object.defineProperty(exports, "ObjectStatus", { enumerable: true, get: function () { return objects_js_1.ObjectStatus; } });
/** @deprecated This type will be removed in a future version */
Object.defineProperty(exports, "ObjectType", { enumerable: true, get: function () { return objects_js_1.ObjectType; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "PaginatedObjectsResponse", { enumerable: true, get: function () { return objects_js_1.PaginatedObjectsResponse; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiGasData", { enumerable: true, get: function () { return objects_js_1.SuiGasData; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveObject", { enumerable: true, get: function () { return objects_js_1.SuiMoveObject; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMovePackage", { enumerable: true, get: function () { return objects_js_1.SuiMovePackage; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiObjectData", { enumerable: true, get: function () { return objects_js_1.SuiObjectData; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiObjectDataOptions", { enumerable: true, get: function () { return objects_js_1.SuiObjectDataOptions; } });
/** @deprecated This type will be removed in a future version */
Object.defineProperty(exports, "SuiObjectInfo", { enumerable: true, get: function () { return objects_js_1.SuiObjectInfo; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiObjectRef", { enumerable: true, get: function () { return objects_js_1.SuiObjectRef; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiObjectResponse", { enumerable: true, get: function () { return objects_js_1.SuiObjectResponse; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiObjectResponseError", { enumerable: true, get: function () { return objects_js_1.SuiObjectResponseError; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiParsedData", { enumerable: true, get: function () { return objects_js_1.SuiParsedData; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiRawData", { enumerable: true, get: function () { return objects_js_1.SuiRawData; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiRawMoveObject", { enumerable: true, get: function () { return objects_js_1.SuiRawMoveObject; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiRawMovePackage", { enumerable: true, get: function () { return objects_js_1.SuiRawMovePackage; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getMoveObject", { enumerable: true, get: function () { return objects_js_1.getMoveObject; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getMoveObjectType", { enumerable: true, get: function () { return objects_js_1.getMoveObjectType; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getMovePackageContent", { enumerable: true, get: function () { return objects_js_1.getMovePackageContent; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectDeletedResponse", { enumerable: true, get: function () { return objects_js_1.getObjectDeletedResponse; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectDisplay", { enumerable: true, get: function () { return objects_js_1.getObjectDisplay; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectFields", { enumerable: true, get: function () { return objects_js_1.getObjectFields; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectId", { enumerable: true, get: function () { return objects_js_1.getObjectId; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectNotExistsResponse", { enumerable: true, get: function () { return objects_js_1.getObjectNotExistsResponse; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectOwner", { enumerable: true, get: function () { return objects_js_1.getObjectOwner; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectPreviousTransactionDigest", { enumerable: true, get: function () { return objects_js_1.getObjectPreviousTransactionDigest; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectReference", { enumerable: true, get: function () { return objects_js_1.getObjectReference; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectType", { enumerable: true, get: function () { return objects_js_1.getObjectType; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getObjectVersion", { enumerable: true, get: function () { return objects_js_1.getObjectVersion; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getSharedObjectInitialVersion", { enumerable: true, get: function () { return objects_js_1.getSharedObjectInitialVersion; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "getSuiObjectData", { enumerable: true, get: function () { return objects_js_1.getSuiObjectData; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "hasPublicTransfer", { enumerable: true, get: function () { return objects_js_1.hasPublicTransfer; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "isImmutableObject", { enumerable: true, get: function () { return objects_js_1.isImmutableObject; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "isSharedObject", { enumerable: true, get: function () { return objects_js_1.isSharedObject; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "isSuiObjectResponse", { enumerable: true, get: function () { return objects_js_1.isSuiObjectResponse; } });
var normalized_js_1 = require("./normalized.js");
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "MoveCallMetric", { enumerable: true, get: function () { return normalized_js_1.MoveCallMetric; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "MoveCallMetrics", { enumerable: true, get: function () { return normalized_js_1.MoveCallMetrics; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveAbilitySet", { enumerable: true, get: function () { return normalized_js_1.SuiMoveAbilitySet; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveFunctionArgType", { enumerable: true, get: function () { return normalized_js_1.SuiMoveFunctionArgType; } });
/* @deprecated Use SuiMoveFunctionArgType[] from `@mysten/sui-js/client` instead */
Object.defineProperty(exports, "SuiMoveFunctionArgTypes", { enumerable: true, get: function () { return normalized_js_1.SuiMoveFunctionArgTypes; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveModuleId", { enumerable: true, get: function () { return normalized_js_1.SuiMoveModuleId; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedField", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedField; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedFunction", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedFunction; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedModule", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedModule; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedModules", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedModules; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedStruct", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedStruct; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedStructType", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedStructType; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedType", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedType; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveNormalizedTypeParameterType", { enumerable: true, get: function () { return normalized_js_1.SuiMoveNormalizedTypeParameterType; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveStructTypeParameter", { enumerable: true, get: function () { return normalized_js_1.SuiMoveStructTypeParameter; } });
/** @deprecated Import type from `@dwallet-network/dwallet.js/client` instead */
Object.defineProperty(exports, "SuiMoveVisibility", { enumerable: true, get: function () { return normalized_js_1.SuiMoveVisibility; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "extractMutableReference", { enumerable: true, get: function () { return normalized_js_1.extractMutableReference; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "extractReference", { enumerable: true, get: function () { return normalized_js_1.extractReference; } });
/** @deprecated This method will be removed in a future version of the SDK */
Object.defineProperty(exports, "extractStructTag", { enumerable: true, get: function () { return normalized_js_1.extractStructTag; } });
//# sourceMappingURL=index.js.map