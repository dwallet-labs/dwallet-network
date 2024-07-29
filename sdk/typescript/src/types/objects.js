"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.ObjectRead = exports.PaginatedObjectsResponse = exports.CheckpointedObjectId = exports.SuiObjectResponse = exports.GetOwnedObjectsResponse = exports.ObjectStatus = exports.SuiObjectDataOptions = exports.SuiObjectData = exports.DisplayFieldsBackwardCompatibleResponse = exports.DisplayFieldsResponse = exports.SuiObjectResponseError = exports.MIST_PER_SUI = exports.SUI_DECIMALS = exports.SuiRawData = exports.SuiRawMovePackage = exports.SuiRawMoveObject = exports.SuiParsedData = exports.SuiMovePackage = exports.SuiMoveObject = exports.MovePackageContent = exports.ObjectContentFields = exports.SuiObjectInfo = exports.SuiGasData = exports.TransactionEffectsModifiedAtVersions = exports.OwnedObjectRef = exports.SuiObjectRef = exports.ObjectType = void 0;
exports.getSuiObjectData = getSuiObjectData;
exports.getObjectDeletedResponse = getObjectDeletedResponse;
exports.getObjectNotExistsResponse = getObjectNotExistsResponse;
exports.getObjectReference = getObjectReference;
exports.getObjectId = getObjectId;
exports.getObjectVersion = getObjectVersion;
exports.isSuiObjectResponse = isSuiObjectResponse;
exports.getObjectType = getObjectType;
exports.getObjectPreviousTransactionDigest = getObjectPreviousTransactionDigest;
exports.getObjectOwner = getObjectOwner;
exports.getObjectDisplay = getObjectDisplay;
exports.getSharedObjectInitialVersion = getSharedObjectInitialVersion;
exports.isSharedObject = isSharedObject;
exports.isImmutableObject = isImmutableObject;
exports.getMoveObjectType = getMoveObjectType;
exports.getObjectFields = getObjectFields;
exports.getMoveObject = getMoveObject;
exports.hasPublicTransfer = hasPublicTransfer;
exports.getMovePackageContent = getMovePackageContent;
const superstruct_1 = require("superstruct");
const common_js_1 = require("./common.js");
exports.ObjectType = (0, superstruct_1.union)([(0, superstruct_1.string)(), (0, superstruct_1.literal)('package')]);
exports.SuiObjectRef = (0, superstruct_1.object)({
    /** Base64 string representing the object digest */
    digest: (0, superstruct_1.string)(),
    /** Hex code as string representing the object id */
    objectId: (0, superstruct_1.string)(),
    /** Object version */
    version: (0, superstruct_1.union)([(0, superstruct_1.number)(), (0, superstruct_1.string)(), (0, superstruct_1.bigint)()]),
});
exports.OwnedObjectRef = (0, superstruct_1.object)({
    owner: common_js_1.ObjectOwner,
    reference: exports.SuiObjectRef,
});
exports.TransactionEffectsModifiedAtVersions = (0, superstruct_1.object)({
    objectId: (0, superstruct_1.string)(),
    sequenceNumber: (0, superstruct_1.string)(),
});
exports.SuiGasData = (0, superstruct_1.object)({
    payment: (0, superstruct_1.array)(exports.SuiObjectRef),
    /** Gas Object's owner */
    owner: (0, superstruct_1.string)(),
    price: (0, superstruct_1.string)(),
    budget: (0, superstruct_1.string)(),
});
exports.SuiObjectInfo = (0, superstruct_1.assign)(exports.SuiObjectRef, (0, superstruct_1.object)({
    type: (0, superstruct_1.string)(),
    owner: common_js_1.ObjectOwner,
    previousTransaction: (0, superstruct_1.string)(),
}));
exports.ObjectContentFields = (0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.any)());
exports.MovePackageContent = (0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.unknown)());
exports.SuiMoveObject = (0, superstruct_1.object)({
    /** Move type (e.g., "0x2::coin::Coin<0x2::dwlt::DWLT>") */
    type: (0, superstruct_1.string)(),
    /** Fields and values stored inside the Move object */
    fields: exports.ObjectContentFields,
    hasPublicTransfer: (0, superstruct_1.boolean)(),
});
exports.SuiMovePackage = (0, superstruct_1.object)({
    /** A mapping from module name to disassembled Move bytecode */
    disassembled: exports.MovePackageContent,
});
exports.SuiParsedData = (0, superstruct_1.union)([
    (0, superstruct_1.assign)(exports.SuiMoveObject, (0, superstruct_1.object)({ dataType: (0, superstruct_1.literal)('moveObject') })),
    (0, superstruct_1.assign)(exports.SuiMovePackage, (0, superstruct_1.object)({ dataType: (0, superstruct_1.literal)('package') })),
]);
exports.SuiRawMoveObject = (0, superstruct_1.object)({
    /** Move type (e.g., "0x2::coin::Coin<0x2::dwlt::DWLT>") */
    type: (0, superstruct_1.string)(),
    hasPublicTransfer: (0, superstruct_1.boolean)(),
    version: (0, superstruct_1.string)(),
    bcsBytes: (0, superstruct_1.string)(),
});
exports.SuiRawMovePackage = (0, superstruct_1.object)({
    id: (0, superstruct_1.string)(),
    /** A mapping from module name to Move bytecode enocded in base64*/
    moduleMap: (0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.string)()),
});
// TODO(chris): consolidate SuiRawParsedData and SuiRawObject using generics
exports.SuiRawData = (0, superstruct_1.union)([
    (0, superstruct_1.assign)(exports.SuiRawMoveObject, (0, superstruct_1.object)({ dataType: (0, superstruct_1.literal)('moveObject') })),
    (0, superstruct_1.assign)(exports.SuiRawMovePackage, (0, superstruct_1.object)({ dataType: (0, superstruct_1.literal)('package') })),
]);
exports.SUI_DECIMALS = 9;
exports.MIST_PER_SUI = BigInt(1000000000);
exports.SuiObjectResponseError = (0, superstruct_1.object)({
    code: (0, superstruct_1.string)(),
    error: (0, superstruct_1.optional)((0, superstruct_1.string)()),
    object_id: (0, superstruct_1.optional)((0, superstruct_1.string)()),
    parent_object_id: (0, superstruct_1.optional)((0, superstruct_1.string)()),
    version: (0, superstruct_1.optional)((0, superstruct_1.string)()),
    digest: (0, superstruct_1.optional)((0, superstruct_1.string)()),
});
exports.DisplayFieldsResponse = (0, superstruct_1.object)({
    data: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.string)()))),
    error: (0, superstruct_1.nullable)((0, superstruct_1.optional)(exports.SuiObjectResponseError)),
});
// TODO: remove after all envs support the new DisplayFieldsResponse;
exports.DisplayFieldsBackwardCompatibleResponse = (0, superstruct_1.union)([
    exports.DisplayFieldsResponse,
    (0, superstruct_1.optional)((0, superstruct_1.record)((0, superstruct_1.string)(), (0, superstruct_1.string)())),
]);
exports.SuiObjectData = (0, superstruct_1.object)({
    objectId: (0, superstruct_1.string)(),
    version: (0, superstruct_1.string)(),
    digest: (0, superstruct_1.string)(),
    /**
     * Type of the object, default to be undefined unless SuiObjectDataOptions.showType is set to true
     */
    type: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.string)())),
    /**
     * Move object content or package content, default to be undefined unless SuiObjectDataOptions.showContent is set to true
     */
    content: (0, superstruct_1.nullable)((0, superstruct_1.optional)(exports.SuiParsedData)),
    /**
     * Move object content or package content in BCS bytes, default to be undefined unless SuiObjectDataOptions.showBcs is set to true
     */
    bcs: (0, superstruct_1.nullable)((0, superstruct_1.optional)(exports.SuiRawData)),
    /**
     * The owner of this object. Default to be undefined unless SuiObjectDataOptions.showOwner is set to true
     */
    owner: (0, superstruct_1.nullable)((0, superstruct_1.optional)(common_js_1.ObjectOwner)),
    /**
     * The digest of the transaction that created or last mutated this object.
     * Default to be undefined unless SuiObjectDataOptions.showPreviousTransaction is set to true
     */
    previousTransaction: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.string)())),
    /**
     * The amount of SUI we would rebate if this object gets deleted.
     * This number is re-calculated each time the object is mutated based on
     * the present storage gas price.
     * Default to be undefined unless SuiObjectDataOptions.showStorageRebate is set to true
     */
    storageRebate: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.string)())),
    /**
     * Display metadata for this object, default to be undefined unless SuiObjectDataOptions.showDisplay is set to true
     * This can also be None if the struct type does not have Display defined
     * See more details in https://forums.sui.io/t/nft-object-display-proposal/4872
     */
    display: (0, superstruct_1.nullable)((0, superstruct_1.optional)(exports.DisplayFieldsBackwardCompatibleResponse)),
});
/**
 * Config for fetching object data
 */
exports.SuiObjectDataOptions = (0, superstruct_1.object)({
    /* Whether to fetch the object type, default to be true */
    showType: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.boolean)())),
    /* Whether to fetch the object content, default to be false */
    showContent: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.boolean)())),
    /* Whether to fetch the object content in BCS bytes, default to be false */
    showBcs: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.boolean)())),
    /* Whether to fetch the object owner, default to be false */
    showOwner: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.boolean)())),
    /* Whether to fetch the previous transaction digest, default to be false */
    showPreviousTransaction: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.boolean)())),
    /* Whether to fetch the storage rebate, default to be false */
    showStorageRebate: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.boolean)())),
    /* Whether to fetch the display metadata, default to be false */
    showDisplay: (0, superstruct_1.nullable)((0, superstruct_1.optional)((0, superstruct_1.boolean)())),
});
exports.ObjectStatus = (0, superstruct_1.union)([(0, superstruct_1.literal)('Exists'), (0, superstruct_1.literal)('notExists'), (0, superstruct_1.literal)('Deleted')]);
exports.GetOwnedObjectsResponse = (0, superstruct_1.array)(exports.SuiObjectInfo);
exports.SuiObjectResponse = (0, superstruct_1.object)({
    data: (0, superstruct_1.nullable)((0, superstruct_1.optional)(exports.SuiObjectData)),
    error: (0, superstruct_1.nullable)((0, superstruct_1.optional)(exports.SuiObjectResponseError)),
});
/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */
/* -------------------------- SuiObjectResponse ------------------------- */
function getSuiObjectData(resp) {
    return resp.data;
}
function getObjectDeletedResponse(resp) {
    if (resp.error &&
        'object_id' in resp.error &&
        'version' in resp.error &&
        'digest' in resp.error) {
        const error = resp.error;
        return {
            objectId: error.object_id,
            version: error.version,
            digest: error.digest,
        };
    }
    return undefined;
}
function getObjectNotExistsResponse(resp) {
    if (resp.error &&
        'object_id' in resp.error &&
        !('version' in resp.error) &&
        !('digest' in resp.error)) {
        return resp.error.object_id;
    }
    return undefined;
}
function getObjectReference(resp) {
    if ('reference' in resp) {
        return resp.reference;
    }
    const exists = getSuiObjectData(resp);
    if (exists) {
        return {
            objectId: exists.objectId,
            version: exists.version,
            digest: exists.digest,
        };
    }
    return getObjectDeletedResponse(resp);
}
/* ------------------------------ SuiObjectRef ------------------------------ */
function getObjectId(data) {
    if ('objectId' in data) {
        return data.objectId;
    }
    return (getObjectReference(data)?.objectId ?? getObjectNotExistsResponse(data));
}
function getObjectVersion(data) {
    if ('version' in data) {
        return data.version;
    }
    return getObjectReference(data)?.version;
}
/* -------------------------------- SuiObject ------------------------------- */
function isSuiObjectResponse(resp) {
    return resp.data !== undefined;
}
/**
 * Deriving the object type from the object response
 * @returns 'package' if the object is a package, move object type(e.g., 0x2::coin::Coin<0x2::dwlt::DWLT>)
 * if the object is a move object
 */
function getObjectType(resp) {
    const data = isSuiObjectResponse(resp) ? resp.data : resp;
    if (!data?.type && 'data' in resp) {
        if (data?.content?.dataType === 'package') {
            return 'package';
        }
        return getMoveObjectType(resp);
    }
    return data?.type;
}
function getObjectPreviousTransactionDigest(resp) {
    return getSuiObjectData(resp)?.previousTransaction;
}
function getObjectOwner(resp) {
    if ((0, superstruct_1.is)(resp, common_js_1.ObjectOwner)) {
        return resp;
    }
    return getSuiObjectData(resp)?.owner;
}
function getObjectDisplay(resp) {
    const display = getSuiObjectData(resp)?.display;
    if (!display) {
        return { data: null, error: null };
    }
    if ((0, superstruct_1.is)(display, exports.DisplayFieldsResponse)) {
        return display;
    }
    return {
        data: display,
        error: null,
    };
}
function getSharedObjectInitialVersion(resp) {
    const owner = getObjectOwner(resp);
    if (owner && typeof owner === 'object' && 'Shared' in owner) {
        return owner.Shared.initial_shared_version;
    }
    else {
        return undefined;
    }
}
function isSharedObject(resp) {
    const owner = getObjectOwner(resp);
    return !!owner && typeof owner === 'object' && 'Shared' in owner;
}
function isImmutableObject(resp) {
    const owner = getObjectOwner(resp);
    return owner === 'Immutable';
}
function getMoveObjectType(resp) {
    return getMoveObject(resp)?.type;
}
function getObjectFields(resp) {
    if ('fields' in resp) {
        return resp.fields;
    }
    return getMoveObject(resp)?.fields;
}
function isSuiObjectDataWithContent(data) {
    return data.content !== undefined;
}
function getMoveObject(data) {
    const suiObject = 'data' in data ? getSuiObjectData(data) : data;
    if (!suiObject ||
        !isSuiObjectDataWithContent(suiObject) ||
        suiObject.content.dataType !== 'moveObject') {
        return undefined;
    }
    return suiObject.content;
}
function hasPublicTransfer(data) {
    return getMoveObject(data)?.hasPublicTransfer ?? false;
}
function getMovePackageContent(data) {
    if ('disassembled' in data) {
        return data.disassembled;
    }
    const suiObject = getSuiObjectData(data);
    if (suiObject?.content?.dataType !== 'package') {
        return undefined;
    }
    return suiObject.content.disassembled;
}
exports.CheckpointedObjectId = (0, superstruct_1.object)({
    objectId: (0, superstruct_1.string)(),
    atCheckpoint: (0, superstruct_1.optional)((0, superstruct_1.number)()),
});
exports.PaginatedObjectsResponse = (0, superstruct_1.object)({
    data: (0, superstruct_1.array)(exports.SuiObjectResponse),
    nextCursor: (0, superstruct_1.optional)((0, superstruct_1.nullable)((0, superstruct_1.string)())),
    hasNextPage: (0, superstruct_1.boolean)(),
});
exports.ObjectRead = (0, superstruct_1.union)([
    (0, superstruct_1.object)({
        details: exports.SuiObjectData,
        status: (0, superstruct_1.literal)('VersionFound'),
    }),
    (0, superstruct_1.object)({
        details: (0, superstruct_1.string)(),
        status: (0, superstruct_1.literal)('ObjectNotExists'),
    }),
    (0, superstruct_1.object)({
        details: exports.SuiObjectRef,
        status: (0, superstruct_1.literal)('ObjectDeleted'),
    }),
    (0, superstruct_1.object)({
        details: (0, superstruct_1.tuple)([(0, superstruct_1.string)(), (0, superstruct_1.number)()]),
        status: (0, superstruct_1.literal)('VersionNotFound'),
    }),
    (0, superstruct_1.object)({
        details: (0, superstruct_1.object)({
            asked_version: (0, superstruct_1.number)(),
            latest_version: (0, superstruct_1.number)(),
            object_id: (0, superstruct_1.string)(),
        }),
        status: (0, superstruct_1.literal)('VersionTooHigh'),
    }),
]);
//# sourceMappingURL=objects.js.map