"use strict";
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
Object.defineProperty(exports, "__esModule", { value: true });
exports.TransactionBlock = void 0;
exports.isTransactionBlock = isTransactionBlock;
const bcs_1 = require("@mysten/bcs");
const superstruct_1 = require("superstruct");
const index_js_1 = require("../bcs/index.js");
const index_js_2 = require("../types/index.js");
const index_js_3 = require("../utils/index.js");
const sui_types_js_1 = require("../utils/sui-types.js");
const Inputs_js_1 = require("./Inputs.js");
const pure_js_1 = require("./pure.js");
const serializer_js_1 = require("./serializer.js");
const TransactionBlockData_js_1 = require("./TransactionBlockData.js");
const Transactions_js_1 = require("./Transactions.js");
const utils_js_1 = require("./utils.js");
const DefaultOfflineLimits = {
    maxPureArgumentSize: 16 * 1024,
    maxTxGas: 50_000_000_000,
    maxGasObjects: 256,
    maxTxSizeBytes: 128 * 1024,
};
function createTransactionResult(index) {
    const baseResult = { kind: 'Result', index };
    const nestedResults = [];
    const nestedResultFor = (resultIndex) => (nestedResults[resultIndex] ??= {
        kind: 'NestedResult',
        index,
        resultIndex,
    });
    return new Proxy(baseResult, {
        set() {
            throw new Error('The transaction result is a proxy, and does not support setting properties directly');
        },
        // TODO: Instead of making this return a concrete argument, we should ideally
        // make it reference-based (so that this gets resolved at build-time), which
        // allows re-ordering transactions.
        get(target, property) {
            // This allows this transaction argument to be used in the singular form:
            if (property in target) {
                return Reflect.get(target, property);
            }
            // Support destructuring:
            if (property === Symbol.iterator) {
                return function* () {
                    let i = 0;
                    while (true) {
                        yield nestedResultFor(i);
                        i++;
                    }
                };
            }
            if (typeof property === 'symbol')
                return;
            const resultIndex = parseInt(property, 10);
            if (Number.isNaN(resultIndex) || resultIndex < 0)
                return;
            return nestedResultFor(resultIndex);
        },
    });
}
function isReceivingType(normalizedType) {
    const tag = (0, index_js_2.extractStructTag)(normalizedType);
    if (tag) {
        return (tag.Struct.address === '0x2' &&
            tag.Struct.module === 'transfer' &&
            tag.Struct.name === 'Receiving');
    }
    return false;
}
function expectClient(options) {
    if (!options.client) {
        throw new Error(`No provider passed to Transaction#build, but transaction data was not sufficient to build offline.`);
    }
    return options.client;
}
const TRANSACTION_BRAND = Symbol.for('@mysten/transaction');
const LIMITS = {
    // The maximum gas that is allowed.
    maxTxGas: 'max_tx_gas',
    // The maximum number of gas objects that can be selected for one transaction.
    maxGasObjects: 'max_gas_payment_objects',
    // The maximum size (in bytes) that the transaction can be:
    maxTxSizeBytes: 'max_tx_size_bytes',
    // The maximum size (in bytes) that pure arguments can be:
    maxPureArgumentSize: 'max_pure_argument_size',
};
// An amount of gas (in gas units) that is added to transactions as an overhead to ensure transactions do not fail.
const GAS_SAFE_OVERHEAD = 1000n;
// The maximum objects that can be fetched at once using multiGetObjects.
const MAX_OBJECTS_PER_FETCH = 50;
const chunk = (arr, size) => Array.from({ length: Math.ceil(arr.length / size) }, (_, i) => arr.slice(i * size, i * size + size));
function isTransactionBlock(obj) {
    return !!obj && typeof obj === 'object' && obj[TRANSACTION_BRAND] === true;
}
/**
 * Transaction Builder
 */
class TransactionBlock {
    /**
     * Converts from a serialize transaction kind (built with `build({ onlyTransactionKind: true })`) to a `Transaction` class.
     * Supports either a byte array, or base64-encoded bytes.
     */
    static fromKind(serialized) {
        const tx = new TransactionBlock();
        tx.#blockData = TransactionBlockData_js_1.TransactionBlockDataBuilder.fromKindBytes(typeof serialized === 'string' ? (0, bcs_1.fromB64)(serialized) : serialized);
        return tx;
    }
    /**
     * Converts from a serialized transaction format to a `Transaction` class.
     * There are two supported serialized formats:
     * - A string returned from `Transaction#serialize`. The serialized format must be compatible, or it will throw an error.
     * - A byte array (or base64-encoded bytes) containing BCS transaction data.
     */
    static from(serialized) {
        const tx = new TransactionBlock();
        // Check for bytes:
        if (typeof serialized !== 'string' || !serialized.startsWith('{')) {
            tx.#blockData = TransactionBlockData_js_1.TransactionBlockDataBuilder.fromBytes(typeof serialized === 'string' ? (0, bcs_1.fromB64)(serialized) : serialized);
        }
        else {
            tx.#blockData = TransactionBlockData_js_1.TransactionBlockDataBuilder.restore(JSON.parse(serialized));
        }
        return tx;
    }
    setSender(sender) {
        this.#blockData.sender = sender;
    }
    /**
     * Sets the sender only if it has not already been set.
     * This is useful for sponsored transaction flows where the sender may not be the same as the signer address.
     */
    setSenderIfNotSet(sender) {
        if (!this.#blockData.sender) {
            this.#blockData.sender = sender;
        }
    }
    setExpiration(expiration) {
        this.#blockData.expiration = expiration;
    }
    setGasPrice(price) {
        this.#blockData.gasConfig.price = String(price);
    }
    setGasBudget(budget) {
        this.#blockData.gasConfig.budget = String(budget);
    }
    setGasOwner(owner) {
        this.#blockData.gasConfig.owner = owner;
    }
    setGasPayment(payments) {
        this.#blockData.gasConfig.payment = payments.map((payment) => (0, superstruct_1.mask)(payment, index_js_2.SuiObjectRef));
    }
    #blockData;
    /** Get a snapshot of the transaction data, in JSON form: */
    get blockData() {
        return this.#blockData.snapshot();
    }
    // Used to brand transaction classes so that they can be identified, even between multiple copies
    // of the builder.
    get [TRANSACTION_BRAND]() {
        return true;
    }
    // Temporary workaround for the wallet interface accidentally serializing transaction blocks via postMessage
    get pure() {
        Object.defineProperty(this, 'pure', {
            enumerable: false,
            value: (0, pure_js_1.createPure)((value, type) => {
                if ((0, bcs_1.isSerializedBcs)(value)) {
                    return this.#input('pure', {
                        Pure: Array.from(value.toBytes()),
                    });
                }
                // TODO: we can also do some deduplication here
                return this.#input('pure', value instanceof Uint8Array
                    ? Inputs_js_1.Inputs.Pure(value)
                    : type
                        ? Inputs_js_1.Inputs.Pure(value, type)
                        : value);
            }),
        });
        return this.pure;
    }
    constructor(transaction) {
        this.#blockData = new TransactionBlockData_js_1.TransactionBlockDataBuilder(transaction ? transaction.blockData : undefined);
    }
    /** Returns an argument for the gas coin, to be used in a transaction. */
    get gas() {
        return { kind: 'GasCoin' };
    }
    /**
     * Dynamically create a new input, which is separate from the `input`. This is important
     * for generated clients to be able to define unique inputs that are non-overlapping with the
     * defined inputs.
     *
     * For `Uint8Array` type automatically convert the input into a `Pure` CallArg, since this
     * is the format required for custom serialization.
     *
     */
    #input(type, value) {
        const index = this.#blockData.inputs.length;
        const input = (0, utils_js_1.create)({
            kind: 'Input',
            // bigints can't be serialized to JSON, so just string-convert them here:
            value: typeof value === 'bigint' ? String(value) : value,
            index,
            type,
        }, Transactions_js_1.TransactionBlockInput);
        this.#blockData.inputs.push(input);
        return input;
    }
    /**
     * Add a new object input to the transaction.
     */
    object(value) {
        if (typeof value === 'object' && 'kind' in value) {
            return value;
        }
        const id = (0, Inputs_js_1.getIdFromCallArg)(value);
        // deduplicate
        const inserted = this.#blockData.inputs.find((i) => i.type === 'object' && id === (0, Inputs_js_1.getIdFromCallArg)(i.value));
        return (inserted ??
            this.#input('object', typeof value === 'string' ? (0, sui_types_js_1.normalizeSuiAddress)(value) : value));
    }
    /**
     * Add a new object input to the transaction using the fully-resolved object reference.
     * If you only have an object ID, use `builder.object(id)` instead.
     */
    objectRef(...args) {
        return this.object(Inputs_js_1.Inputs.ObjectRef(...args));
    }
    /**
     * Add a new receiving input to the transaction using the fully-resolved object reference.
     * If you only have an object ID, use `builder.object(id)` instead.
     */
    receivingRef(...args) {
        return this.object(Inputs_js_1.Inputs.ReceivingRef(...args));
    }
    /**
     * Add a new shared object input to the transaction using the fully-resolved shared object reference.
     * If you only have an object ID, use `builder.object(id)` instead.
     */
    sharedObjectRef(...args) {
        return this.object(Inputs_js_1.Inputs.SharedObjectRef(...args));
    }
    /** Add a transaction to the transaction block. */
    add(transaction) {
        const index = this.#blockData.transactions.push(transaction);
        return createTransactionResult(index - 1);
    }
    #normalizeTransactionArgument(arg) {
        if ((0, bcs_1.isSerializedBcs)(arg)) {
            return this.pure(arg);
        }
        return arg;
    }
    // Method shorthands:
    splitCoins(coin, amounts) {
        return this.add(Transactions_js_1.Transactions.SplitCoins(typeof coin === 'string' ? this.object(coin) : coin, amounts.map((amount) => typeof amount === 'number' || typeof amount === 'bigint' || typeof amount === 'string'
            ? this.pure.u64(amount)
            : this.#normalizeTransactionArgument(amount))));
    }
    mergeCoins(destination, sources) {
        return this.add(Transactions_js_1.Transactions.MergeCoins(typeof destination === 'string' ? this.object(destination) : destination, sources.map((src) => (typeof src === 'string' ? this.object(src) : src))));
    }
    publish({ modules, dependencies }) {
        return this.add(Transactions_js_1.Transactions.Publish({
            modules,
            dependencies,
        }));
    }
    upgrade({ modules, dependencies, packageId, ticket, }) {
        return this.add(Transactions_js_1.Transactions.Upgrade({
            modules,
            dependencies,
            packageId,
            ticket: typeof ticket === 'string' ? this.object(ticket) : ticket,
        }));
    }
    moveCall({ arguments: args, typeArguments, target, }) {
        return this.add(Transactions_js_1.Transactions.MoveCall({
            arguments: args?.map((arg) => this.#normalizeTransactionArgument(arg)),
            typeArguments,
            target,
        }));
    }
    transferObjects(objects, address) {
        return this.add(Transactions_js_1.Transactions.TransferObjects(objects.map((obj) => (typeof obj === 'string' ? this.object(obj) : obj)), typeof address === 'string'
            ? this.pure.address(address)
            : this.#normalizeTransactionArgument(address)));
    }
    makeMoveVec({ type, objects, }) {
        return this.add(Transactions_js_1.Transactions.MakeMoveVec({
            type,
            objects: objects.map((obj) => (typeof obj === 'string' ? this.object(obj) : obj)),
        }));
    }
    /**
     * Serialize the transaction to a string so that it can be sent to a separate context.
     * This is different from `build` in that it does not serialize to BCS bytes, and instead
     * uses a separate format that is unique to the transaction builder. This allows
     * us to serialize partially-complete transactions, that can then be completed and
     * built in a separate context.
     *
     * For example, a dapp can construct a transaction, but not provide gas objects
     * or a gas budget. The transaction then can be sent to the wallet, where this
     * information is automatically filled in (e.g. by querying for coin objects
     * and performing a dry run).
     */
    serialize() {
        return JSON.stringify(this.#blockData.snapshot());
    }
    #getConfig(key, { protocolConfig, limits }) {
        // Use the limits definition if that exists:
        if (limits && typeof limits[key] === 'number') {
            return limits[key];
        }
        if (!protocolConfig) {
            return DefaultOfflineLimits[key];
        }
        // Fallback to protocol config:
        const attribute = protocolConfig?.attributes[LIMITS[key]];
        if (!attribute) {
            throw new Error(`Missing expected protocol config: "${LIMITS[key]}"`);
        }
        const value = 'u64' in attribute ? attribute.u64 : 'u32' in attribute ? attribute.u32 : attribute.f64;
        if (!value) {
            throw new Error(`Unexpected protocol config value found for: "${LIMITS[key]}"`);
        }
        // NOTE: Technically this is not a safe conversion, but we know all of the values in protocol config are safe
        return Number(value);
    }
    /** Build the transaction to BCS bytes, and sign it with the provided keypair. */
    async sign(options) {
        const { signer, ...buildOptions } = options;
        const bytes = await this.build(buildOptions);
        return signer.signTransactionBlock(bytes);
    }
    /** Build the transaction to BCS bytes. */
    async build(options = {}) {
        await this.#prepare(options);
        return this.#blockData.build({
            maxSizeBytes: this.#getConfig('maxTxSizeBytes', options),
            onlyTransactionKind: options.onlyTransactionKind,
        });
    }
    /** Derive transaction digest */
    async getDigest(options = {}) {
        await this.#prepare(options);
        return this.#blockData.getDigest();
    }
    #validate(options) {
        const maxPureArgumentSize = this.#getConfig('maxPureArgumentSize', options);
        // Validate all inputs are the correct size:
        this.#blockData.inputs.forEach((input, index) => {
            if ((0, superstruct_1.is)(input.value, Inputs_js_1.PureCallArg)) {
                if (input.value.Pure.length > maxPureArgumentSize) {
                    throw new Error(`Input at index ${index} is too large, max pure input size is ${maxPureArgumentSize} bytes, got ${input.value.Pure.length} bytes`);
                }
            }
        });
    }
    // The current default is just picking _all_ coins we can which may not be ideal.
    async #prepareGasPayment(options) {
        if (this.#blockData.gasConfig.payment) {
            const maxGasObjects = this.#getConfig('maxGasObjects', options);
            if (this.#blockData.gasConfig.payment.length > maxGasObjects) {
                throw new Error(`Payment objects exceed maximum amount: ${maxGasObjects}`);
            }
        }
        // Early return if the payment is already set:
        if (options.onlyTransactionKind || this.#blockData.gasConfig.payment) {
            return;
        }
        const gasOwner = this.#blockData.gasConfig.owner ?? this.#blockData.sender;
        const coins = await expectClient(options).getCoins({
            owner: gasOwner,
            coinType: index_js_3.SUI_TYPE_ARG,
        });
        const paymentCoins = coins.data
            // Filter out coins that are also used as input:
            .filter((coin) => {
            const matchingInput = this.#blockData.inputs.find((input) => {
                if ((0, superstruct_1.is)(input.value, Inputs_js_1.BuilderCallArg) &&
                    'Object' in input.value &&
                    'ImmOrOwned' in input.value.Object) {
                    return coin.coinObjectId === input.value.Object.ImmOrOwned.objectId;
                }
                return false;
            });
            return !matchingInput;
        })
            .slice(0, this.#getConfig('maxGasObjects', options) - 1)
            .map((coin) => ({
            objectId: coin.coinObjectId,
            digest: coin.digest,
            version: coin.version,
        }));
        if (!paymentCoins.length) {
            throw new Error('No valid gas coins found for the transaction.');
        }
        this.setGasPayment(paymentCoins);
    }
    async #prepareGasPrice(options) {
        if (options.onlyTransactionKind || this.#blockData.gasConfig.price) {
            return;
        }
        this.setGasPrice(await expectClient(options).getReferenceGasPrice());
    }
    async #prepareTransactions(options) {
        const { inputs, transactions } = this.#blockData;
        const moveModulesToResolve = [];
        // Keep track of the object references that will need to be resolved at the end of the transaction.
        // We keep the input by-reference to avoid needing to re-resolve it:
        const objectsToResolve = [];
        inputs.forEach((input) => {
            if (input.type === 'object' && typeof input.value === 'string') {
                // The input is a string that we need to resolve to an object reference:
                objectsToResolve.push({ id: (0, sui_types_js_1.normalizeSuiAddress)(input.value), input });
                return;
            }
        });
        transactions.forEach((transaction) => {
            // Special case move call:
            if (transaction.kind === 'MoveCall') {
                // Determine if any of the arguments require encoding.
                // - If they don't, then this is good to go.
                // - If they do, then we need to fetch the normalized move module.
                const needsResolution = transaction.arguments.some((arg) => arg.kind === 'Input' && !(0, superstruct_1.is)(inputs[arg.index].value, Inputs_js_1.BuilderCallArg));
                if (needsResolution) {
                    moveModulesToResolve.push(transaction);
                }
            }
            // Special handling for values that where previously encoded using the wellKnownEncoding pattern.
            // This should only happen when transaction block data was hydrated from an old version of the SDK
            if (transaction.kind === 'SplitCoins') {
                transaction.amounts.forEach((amount) => {
                    if (amount.kind === 'Input') {
                        const input = inputs[amount.index];
                        if (typeof input.value !== 'object') {
                            input.value = Inputs_js_1.Inputs.Pure(index_js_1.bcs.U64.serialize(input.value));
                        }
                    }
                });
            }
            if (transaction.kind === 'TransferObjects') {
                if (transaction.address.kind === 'Input') {
                    const input = inputs[transaction.address.index];
                    if (typeof input.value !== 'object') {
                        input.value = Inputs_js_1.Inputs.Pure(index_js_1.bcs.Address.serialize(input.value));
                    }
                }
            }
        });
        if (moveModulesToResolve.length) {
            await Promise.all(moveModulesToResolve.map(async (moveCall) => {
                const [packageId, moduleName, functionName] = moveCall.target.split('::');
                const normalized = await expectClient(options).getNormalizedMoveFunction({
                    package: (0, sui_types_js_1.normalizeSuiObjectId)(packageId),
                    module: moduleName,
                    function: functionName,
                });
                // Entry functions can have a mutable reference to an instance of the TxContext
                // struct defined in the TxContext module as the last parameter. The caller of
                // the function does not need to pass it in as an argument.
                const hasTxContext = normalized.parameters.length > 0 && (0, serializer_js_1.isTxContext)(normalized.parameters.at(-1));
                const params = hasTxContext
                    ? normalized.parameters.slice(0, normalized.parameters.length - 1)
                    : normalized.parameters;
                if (params.length !== moveCall.arguments.length) {
                    throw new Error('Incorrect number of arguments.');
                }
                params.forEach((param, i) => {
                    const arg = moveCall.arguments[i];
                    if (arg.kind !== 'Input')
                        return;
                    const input = inputs[arg.index];
                    // Skip if the input is already resolved
                    if ((0, superstruct_1.is)(input.value, Inputs_js_1.BuilderCallArg))
                        return;
                    const inputValue = input.value;
                    const serType = (0, serializer_js_1.getPureSerializationType)(param, inputValue);
                    if (serType) {
                        input.value = Inputs_js_1.Inputs.Pure(inputValue, serType);
                        return;
                    }
                    const structVal = (0, index_js_2.extractStructTag)(param);
                    if (structVal != null || (typeof param === 'object' && 'TypeParameter' in param)) {
                        if (typeof inputValue !== 'string') {
                            throw new Error(`Expect the argument to be an object id string, got ${JSON.stringify(inputValue, null, 2)}`);
                        }
                        objectsToResolve.push({
                            id: inputValue,
                            input,
                            normalizedType: param,
                        });
                        return;
                    }
                    throw new Error(`Unknown call arg type ${JSON.stringify(param, null, 2)} for value ${JSON.stringify(inputValue, null, 2)}`);
                });
            }));
        }
        if (objectsToResolve.length) {
            const dedupedIds = [...new Set(objectsToResolve.map(({ id }) => id))];
            const objectChunks = chunk(dedupedIds, MAX_OBJECTS_PER_FETCH);
            const objects = (await Promise.all(objectChunks.map((chunk) => expectClient(options).multiGetObjects({
                ids: chunk,
                options: { showOwner: true },
            })))).flat();
            let objectsById = new Map(dedupedIds.map((id, index) => {
                return [id, objects[index]];
            }));
            const invalidObjects = Array.from(objectsById)
                .filter(([_, obj]) => obj.error)
                .map(([id, _]) => id);
            if (invalidObjects.length) {
                throw new Error(`The following input objects are invalid: ${invalidObjects.join(', ')}`);
            }
            objectsToResolve.forEach(({ id, input, normalizedType }) => {
                const object = objectsById.get(id);
                const owner = object.data?.owner;
                const initialSharedVersion = owner && typeof owner === 'object' && 'Shared' in owner
                    ? owner.Shared.initial_shared_version
                    : undefined;
                if (initialSharedVersion) {
                    // There could be multiple transactions that reference the same shared object.
                    // If one of them is a mutable reference or taken by value, then we should mark the input
                    // as mutable.
                    const isByValue = normalizedType != null &&
                        (0, index_js_2.extractMutableReference)(normalizedType) == null &&
                        (0, index_js_2.extractReference)(normalizedType) == null;
                    const mutable = (0, Inputs_js_1.isMutableSharedObjectInput)(input.value) ||
                        isByValue ||
                        (normalizedType != null && (0, index_js_2.extractMutableReference)(normalizedType) != null);
                    input.value = Inputs_js_1.Inputs.SharedObjectRef({
                        objectId: id,
                        initialSharedVersion,
                        mutable,
                    });
                }
                else if (normalizedType && isReceivingType(normalizedType)) {
                    input.value = Inputs_js_1.Inputs.ReceivingRef((0, index_js_2.getObjectReference)(object));
                }
                else {
                    input.value = Inputs_js_1.Inputs.ObjectRef((0, index_js_2.getObjectReference)(object));
                }
            });
        }
    }
    /**
     * Prepare the transaction by valdiating the transaction data and resolving all inputs
     * so that it can be built into bytes.
     */
    async #prepare(options) {
        if (!options.onlyTransactionKind && !this.#blockData.sender) {
            throw new Error('Missing transaction sender');
        }
        if (!options.protocolConfig && !options.limits && options.client) {
            options.protocolConfig = await options.client.getProtocolConfig();
        }
        await Promise.all([this.#prepareGasPrice(options), this.#prepareTransactions(options)]);
        if (!options.onlyTransactionKind) {
            await this.#prepareGasPayment(options);
            if (!this.#blockData.gasConfig.budget) {
                const dryRunResult = await expectClient(options).dryRunTransactionBlock({
                    transactionBlock: this.#blockData.build({
                        maxSizeBytes: this.#getConfig('maxTxSizeBytes', options),
                        overrides: {
                            gasConfig: {
                                budget: String(this.#getConfig('maxTxGas', options)),
                                payment: [],
                            },
                        },
                    }),
                });
                if (dryRunResult.effects.status.status !== 'success') {
                    throw new Error(`Dry run failed, could not automatically determine a budget: ${dryRunResult.effects.status.error}`, { cause: dryRunResult });
                }
                const safeOverhead = GAS_SAFE_OVERHEAD * BigInt(this.blockData.gasConfig.price || 1n);
                const baseComputationCostWithOverhead = BigInt(dryRunResult.effects.gasUsed.computationCost) + safeOverhead;
                const gasBudget = baseComputationCostWithOverhead +
                    BigInt(dryRunResult.effects.gasUsed.storageCost) -
                    BigInt(dryRunResult.effects.gasUsed.storageRebate);
                // Set the budget to max(computation, computation + storage - rebate)
                this.setGasBudget(gasBudget > baseComputationCostWithOverhead ? gasBudget : baseComputationCostWithOverhead);
            }
        }
        // Perform final validation on the transaction:
        this.#validate(options);
    }
}
exports.TransactionBlock = TransactionBlock;
//# sourceMappingURL=TransactionBlock.js.map