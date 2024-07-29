"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.bcsRegistry = exports.bcs = exports.TypeTagSerializer = void 0;
exports.isPureArg = isPureArg;
const bcs_1 = require("@mysten/bcs");
const sui_types_js_1 = require("../utils/sui-types.js");
const type_tag_serializer_js_1 = require("./type-tag-serializer.js");
var type_tag_serializer_js_2 = require("./type-tag-serializer.js");
Object.defineProperty(exports, "TypeTagSerializer", { enumerable: true, get: function () { return type_tag_serializer_js_2.TypeTagSerializer; } });
function isPureArg(arg) {
    return arg.Pure !== undefined;
}
const bcsRegistry = new bcs_1.BCS({
    ...(0, bcs_1.getSuiMoveConfig)(),
    types: {
        enums: {
            'Option<T>': {
                None: null,
                Some: 'T',
            },
        },
    },
});
exports.bcsRegistry = bcsRegistry;
function unsafe_u64(options) {
    return bcs_1.bcs
        .u64({
        name: 'unsafe_u64',
        ...options,
    })
        .transform({
        input: (val) => val,
        output: (val) => Number(val),
    });
}
function optionEnum(type) {
    return bcs_1.bcs.enum('Option', {
        None: null,
        Some: type,
    });
}
/**
 * Wrapper around Enum, which transforms any `T` into an object with `kind` property:
 * @example
 * ```
 * let bcsEnum = { TransferObjects: { objects: [], address: ... } }
 * // becomes
 * let translatedEnum = { kind: 'TransferObjects', objects: [], address: ... };
 * ```
 */
function enumKind(type) {
    return type.transform({
        input: (val) => ({
            [val.kind]: val,
        }),
        output: (val) => {
            const key = Object.keys(val)[0];
            return { kind: key, ...val[key] };
        },
    });
}
const Address = bcs_1.bcs.bytes(sui_types_js_1.SUI_ADDRESS_LENGTH).transform({
    input: (val) => typeof val === 'string' ? (0, bcs_1.fromHEX)((0, sui_types_js_1.normalizeSuiAddress)(val)) : val,
    output: (val) => (0, sui_types_js_1.normalizeSuiAddress)((0, bcs_1.toHEX)(val)),
});
const ObjectDigest = bcs_1.bcs.vector(bcs_1.bcs.u8()).transform({
    name: 'ObjectDigest',
    input: (value) => (0, bcs_1.fromB58)(value),
    output: (value) => (0, bcs_1.toB58)(new Uint8Array(value)),
});
const SuiObjectRef = bcs_1.bcs.struct('SuiObjectRef', {
    objectId: Address,
    version: bcs_1.bcs.u64(),
    digest: ObjectDigest,
});
const SharedObjectRef = bcs_1.bcs.struct('SharedObjectRef', {
    objectId: Address,
    initialSharedVersion: bcs_1.bcs.u64(),
    mutable: bcs_1.bcs.bool(),
});
const ObjectArg = bcs_1.bcs.enum('ObjectArg', {
    ImmOrOwned: SuiObjectRef,
    Shared: SharedObjectRef,
    Receiving: SuiObjectRef,
});
const CallArg = bcs_1.bcs.enum('CallArg', {
    Pure: bcs_1.bcs.vector(bcs_1.bcs.u8()),
    Object: ObjectArg,
    ObjVec: bcs_1.bcs.vector(ObjectArg),
});
const TypeTag = bcs_1.bcs.enum('TypeTag', {
    bool: null,
    u8: null,
    u64: null,
    u128: null,
    address: null,
    signer: null,
    vector: bcs_1.bcs.lazy(() => TypeTag),
    struct: bcs_1.bcs.lazy(() => StructTag),
    u16: null,
    u32: null,
    u256: null,
});
const Argument = enumKind(bcs_1.bcs.enum('Argument', {
    GasCoin: null,
    Input: bcs_1.bcs.struct('Input', { index: bcs_1.bcs.u16() }),
    Result: bcs_1.bcs.struct('Result', { index: bcs_1.bcs.u16() }),
    NestedResult: bcs_1.bcs.struct('NestedResult', { index: bcs_1.bcs.u16(), resultIndex: bcs_1.bcs.u16() }),
}));
/** Custom serializer for decoding package, module, function easier */
const ProgrammableMoveCall = bcs_1.bcs
    .struct('ProgrammableMoveCall', {
    package: Address,
    module: bcs_1.bcs.string(),
    function: bcs_1.bcs.string(),
    type_arguments: bcs_1.bcs.vector(TypeTag),
    arguments: bcs_1.bcs.vector(Argument),
})
    .transform({
    input: (data) => {
        const [pkg, module, fun] = data.target.split('::');
        const type_arguments = data.typeArguments.map((tag) => type_tag_serializer_js_1.TypeTagSerializer.parseFromStr(tag, true));
        return {
            package: (0, sui_types_js_1.normalizeSuiAddress)(pkg),
            module,
            function: fun,
            type_arguments,
            arguments: data.arguments,
        };
    },
    output: (data) => {
        return {
            target: [data.package, data.module, data.function].join('::'),
            arguments: data.arguments,
            typeArguments: data.type_arguments.map(type_tag_serializer_js_1.TypeTagSerializer.tagToString),
        };
    },
});
const Transaction = enumKind(bcs_1.bcs.enum('Transaction', {
    /**
     * A Move Call - any public Move function can be called via
     * this transaction. The results can be used that instant to pass
     * into the next transaction.
     */
    MoveCall: ProgrammableMoveCall,
    /**
     * Transfer vector of objects to a receiver.
     */
    TransferObjects: bcs_1.bcs.struct('TransferObjects', {
        objects: bcs_1.bcs.vector(Argument),
        address: Argument,
    }),
    /**
     * Split `amount` from a `coin`.
     */
    SplitCoins: bcs_1.bcs.struct('SplitCoins', { coin: Argument, amounts: bcs_1.bcs.vector(Argument) }),
    /**
     * Merge Vector of Coins (`sources`) into a `destination`.
     */
    MergeCoins: bcs_1.bcs.struct('MergeCoins', { destination: Argument, sources: bcs_1.bcs.vector(Argument) }),
    /**
     * Publish a Move module.
     */
    Publish: bcs_1.bcs.struct('Publish', {
        modules: bcs_1.bcs.vector(bcs_1.bcs.vector(bcs_1.bcs.u8())),
        dependencies: bcs_1.bcs.vector(Address),
    }),
    /**
     * Build a vector of objects using the input arguments.
     * It is impossible to construct a `vector<T: key>` otherwise,
     * so this call serves a utility function.
     */
    MakeMoveVec: bcs_1.bcs.struct('MakeMoveVec', {
        type: optionEnum(TypeTag),
        objects: bcs_1.bcs.vector(Argument),
    }),
    /**  */
    Upgrade: bcs_1.bcs.struct('Upgrade', {
        modules: bcs_1.bcs.vector(bcs_1.bcs.vector(bcs_1.bcs.u8())),
        dependencies: bcs_1.bcs.vector(Address),
        packageId: Address,
        ticket: Argument,
    }),
}));
const ProgrammableTransaction = bcs_1.bcs.struct('ProgrammableTransaction', {
    inputs: bcs_1.bcs.vector(CallArg),
    transactions: bcs_1.bcs.vector(Transaction),
});
const TransactionKind = bcs_1.bcs.enum('TransactionKind', {
    ProgrammableTransaction: ProgrammableTransaction,
    ChangeEpoch: null,
    Genesis: null,
    ConsensusCommitPrologue: null,
});
const TransactionExpiration = bcs_1.bcs.enum('TransactionExpiration', {
    None: null,
    Epoch: unsafe_u64(),
});
const StructTag = bcs_1.bcs.struct('StructTag', {
    address: Address,
    module: bcs_1.bcs.string(),
    name: bcs_1.bcs.string(),
    typeParams: bcs_1.bcs.vector(TypeTag),
});
const GasData = bcs_1.bcs.struct('GasData', {
    payment: bcs_1.bcs.vector(SuiObjectRef),
    owner: Address,
    price: bcs_1.bcs.u64(),
    budget: bcs_1.bcs.u64(),
});
const TransactionDataV1 = bcs_1.bcs.struct('TransactionDataV1', {
    kind: TransactionKind,
    sender: Address,
    gasData: GasData,
    expiration: TransactionExpiration,
});
const TransactionData = bcs_1.bcs.enum('TransactionData', {
    V1: TransactionDataV1,
});
// Signed transaction data needed to generate transaction digest.
const SenderSignedData = bcs_1.bcs.struct('SenderSignedData', {
    data: TransactionData,
    txSignatures: bcs_1.bcs.vector(bcs_1.bcs.vector(bcs_1.bcs.u8())),
});
const CompressedSignature = bcs_1.bcs.enum('CompressedSignature', {
    ED25519: bcs_1.bcs.fixedArray(64, bcs_1.bcs.u8()),
    Secp256k1: bcs_1.bcs.fixedArray(64, bcs_1.bcs.u8()),
    Secp256r1: bcs_1.bcs.fixedArray(64, bcs_1.bcs.u8()),
    ZkLogin: bcs_1.bcs.vector(bcs_1.bcs.u8()),
});
const PublicKey = bcs_1.bcs.enum('PublicKey', {
    ED25519: bcs_1.bcs.fixedArray(32, bcs_1.bcs.u8()),
    Secp256k1: bcs_1.bcs.fixedArray(33, bcs_1.bcs.u8()),
    Secp256r1: bcs_1.bcs.fixedArray(33, bcs_1.bcs.u8()),
    ZkLogin: bcs_1.bcs.vector(bcs_1.bcs.u8()),
});
const MultiSigPkMap = bcs_1.bcs.struct('MultiSigPkMap', {
    pubKey: PublicKey,
    weight: bcs_1.bcs.u8(),
});
const MultiSigPublicKey = bcs_1.bcs.struct('MultiSigPublicKey', {
    pk_map: bcs_1.bcs.vector(MultiSigPkMap),
    threshold: bcs_1.bcs.u16(),
});
const MultiSig = bcs_1.bcs.struct('MultiSig', {
    sigs: bcs_1.bcs.vector(CompressedSignature),
    bitmap: bcs_1.bcs.u16(),
    multisig_pk: MultiSigPublicKey,
});
const suiBcs = {
    ...bcs_1.bcs,
    U8: bcs_1.bcs.u8(),
    U16: bcs_1.bcs.u16(),
    U32: bcs_1.bcs.u32(),
    U64: bcs_1.bcs.u64(),
    U128: bcs_1.bcs.u128(),
    U256: bcs_1.bcs.u256(),
    ULEB128: bcs_1.bcs.uleb128(),
    Bool: bcs_1.bcs.bool(),
    String: bcs_1.bcs.string(),
    Address,
    Argument,
    CallArg,
    CompressedSignature,
    GasData,
    MultiSig,
    MultiSigPkMap,
    MultiSigPublicKey,
    ObjectArg,
    ObjectDigest,
    ProgrammableMoveCall,
    ProgrammableTransaction,
    PublicKey,
    SenderSignedData,
    SharedObjectRef,
    StructTag,
    SuiObjectRef,
    Transaction,
    TransactionData,
    TransactionDataV1,
    TransactionExpiration,
    TransactionKind,
    TypeTag,
    // preserve backwards compatibility with old bcs export
    ser: bcsRegistry.ser.bind(bcsRegistry),
    de: bcsRegistry.de.bind(bcsRegistry),
    getTypeInterface: bcsRegistry.getTypeInterface.bind(bcsRegistry),
    hasType: bcsRegistry.hasType.bind(bcsRegistry),
    parseTypeName: bcsRegistry.parseTypeName.bind(bcsRegistry),
    registerAddressType: bcsRegistry.registerAddressType.bind(bcsRegistry),
    registerAlias: bcsRegistry.registerAlias.bind(bcsRegistry),
    registerBcsType: bcsRegistry.registerBcsType.bind(bcsRegistry),
    registerEnumType: bcsRegistry.registerEnumType.bind(bcsRegistry),
    registerStructType: bcsRegistry.registerStructType.bind(bcsRegistry),
    registerType: bcsRegistry.registerType.bind(bcsRegistry),
    types: bcsRegistry.types,
};
exports.bcs = suiBcs;
bcsRegistry.registerBcsType('utf8string', () => bcs_1.bcs.string({ name: 'utf8string' }));
bcsRegistry.registerBcsType('unsafe_u64', () => unsafe_u64());
bcsRegistry.registerBcsType('enumKind', (T) => enumKind(T));
[
    Address,
    Argument,
    CallArg,
    CompressedSignature,
    GasData,
    MultiSig,
    MultiSigPkMap,
    MultiSigPublicKey,
    ObjectArg,
    ObjectDigest,
    ProgrammableMoveCall,
    ProgrammableTransaction,
    PublicKey,
    SenderSignedData,
    SharedObjectRef,
    StructTag,
    SuiObjectRef,
    Transaction,
    TransactionData,
    TransactionDataV1,
    TransactionExpiration,
    TransactionKind,
    TypeTag,
].forEach((type) => {
    bcsRegistry.registerBcsType(type.name, () => type);
});
//# sourceMappingURL=index.js.map