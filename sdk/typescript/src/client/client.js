"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.DWalletClient = exports.DWALLET_CLIENT_BRAND = void 0;
exports.isDWalletClient = isDWalletClient;
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
const bcs_1 = require("@mysten/bcs");
const index_js_1 = require("../builder/index.js");
const sui_types_js_1 = require("../utils/sui-types.js");
const http_transport_js_1 = require("./http-transport.js");
exports.DWALLET_CLIENT_BRAND = Symbol.for('@dwallet-network/SuiClient');
function isDWalletClient(client) {
    return (typeof client === 'object' &&
        client !== null &&
        client[exports.DWALLET_CLIENT_BRAND] === true);
}
class DWalletClient {
    transport;
    get [exports.DWALLET_CLIENT_BRAND]() {
        return true;
    }
    /**
     * Establish a connection to a Sui RPC endpoint
     *
     * @param options configuration options for the API Client
     */
    constructor(options) {
        this.transport = options.transport ?? new http_transport_js_1.SuiHTTPTransport({ url: options.url });
    }
    async getRpcApiVersion() {
        const resp = await this.transport.request({
            method: 'rpc.discover',
            params: [],
        });
        return resp.info.version;
    }
    /**
     * Get all Coin<`coin_type`> objects owned by an address.
     */
    async getCoins(input) {
        if (!input.owner || !(0, sui_types_js_1.isValidSuiAddress)((0, sui_types_js_1.normalizeSuiAddress)(input.owner))) {
            throw new Error('Invalid Sui address');
        }
        return await this.transport.request({
            method: 'suix_getCoins',
            params: [input.owner, input.coinType, input.cursor, input.limit],
        });
    }
    /**
     * Get all Coin objects owned by an address.
     */
    async getAllCoins(input) {
        if (!input.owner || !(0, sui_types_js_1.isValidSuiAddress)((0, sui_types_js_1.normalizeSuiAddress)(input.owner))) {
            throw new Error('Invalid Sui address');
        }
        return await this.transport.request({
            method: 'suix_getAllCoins',
            params: [input.owner, input.cursor, input.limit],
        });
    }
    /**
     * Get the total coin balance for one coin type, owned by the address owner.
     */
    async getBalance(input) {
        if (!input.owner || !(0, sui_types_js_1.isValidSuiAddress)((0, sui_types_js_1.normalizeSuiAddress)(input.owner))) {
            throw new Error('Invalid Sui address');
        }
        return await this.transport.request({
            method: 'suix_getBalance',
            params: [input.owner, input.coinType],
        });
    }
    /**
     * Get the total coin balance for all coin types, owned by the address owner.
     */
    async getAllBalances(input) {
        if (!input.owner || !(0, sui_types_js_1.isValidSuiAddress)((0, sui_types_js_1.normalizeSuiAddress)(input.owner))) {
            throw new Error('Invalid Sui address');
        }
        return await this.transport.request({ method: 'suix_getAllBalances', params: [input.owner] });
    }
    /**
     * Fetch CoinMetadata for a given coin type
     */
    async getCoinMetadata(input) {
        return await this.transport.request({
            method: 'suix_getCoinMetadata',
            params: [input.coinType],
        });
    }
    /**
     *  Fetch total supply for a coin
     */
    async getTotalSupply(input) {
        return await this.transport.request({
            method: 'suix_getTotalSupply',
            params: [input.coinType],
        });
    }
    /**
     * Invoke any RPC method
     * @param method the method to be invoked
     * @param args the arguments to be passed to the RPC request
     */
    async call(method, params) {
        return await this.transport.request({ method, params });
    }
    /**
     * Get Move function argument types like read, write and full access
     */
    async getMoveFunctionArgTypes(input) {
        return await this.transport.request({
            method: 'sui_getMoveFunctionArgTypes',
            params: [input.package, input.module, input.function],
        });
    }
    /**
     * Get a map from module name to
     * structured representations of Move modules
     */
    async getNormalizedMoveModulesByPackage(input) {
        return await this.transport.request({
            method: 'sui_getNormalizedMoveModulesByPackage',
            params: [input.package],
        });
    }
    /**
     * Get a structured representation of Move module
     */
    async getNormalizedMoveModule(input) {
        return await this.transport.request({
            method: 'sui_getNormalizedMoveModule',
            params: [input.package, input.module],
        });
    }
    /**
     * Get a structured representation of Move function
     */
    async getNormalizedMoveFunction(input) {
        return await this.transport.request({
            method: 'sui_getNormalizedMoveFunction',
            params: [input.package, input.module, input.function],
        });
    }
    /**
     * Get a structured representation of Move struct
     */
    async getNormalizedMoveStruct(input) {
        return await this.transport.request({
            method: 'sui_getNormalizedMoveStruct',
            params: [input.package, input.module, input.struct],
        });
    }
    /**
     * Get all objects owned by an address
     */
    async getOwnedObjects(input) {
        if (!input.owner || !(0, sui_types_js_1.isValidSuiAddress)((0, sui_types_js_1.normalizeSuiAddress)(input.owner))) {
            throw new Error('Invalid Sui address');
        }
        return await this.transport.request({
            method: 'suix_getOwnedObjects',
            params: [
                input.owner,
                {
                    filter: input.filter,
                    options: input.options,
                },
                input.cursor,
                input.limit,
            ],
        });
    }
    /**
     * Get details about an object
     */
    async getObject(input) {
        if (!input.id || !(0, sui_types_js_1.isValidSuiObjectId)((0, sui_types_js_1.normalizeSuiObjectId)(input.id))) {
            throw new Error('Invalid Sui Object id');
        }
        return await this.transport.request({
            method: 'sui_getObject',
            params: [input.id, input.options],
        });
    }
    async tryGetPastObject(input) {
        return await this.transport.request({
            method: 'sui_tryGetPastObject',
            params: [input.id, input.version, input.options],
        });
    }
    /**
     * Batch get details about a list of objects. If any of the object ids are duplicates the call will fail
     */
    async multiGetObjects(input) {
        input.ids.forEach((id) => {
            if (!id || !(0, sui_types_js_1.isValidSuiObjectId)((0, sui_types_js_1.normalizeSuiObjectId)(id))) {
                throw new Error(`Invalid Sui Object id ${id}`);
            }
        });
        const hasDuplicates = input.ids.length !== new Set(input.ids).size;
        if (hasDuplicates) {
            throw new Error(`Duplicate object ids in batch call ${input.ids}`);
        }
        return await this.transport.request({
            method: 'sui_multiGetObjects',
            params: [input.ids, input.options],
        });
    }
    /**
     * Get transaction blocks for a given query criteria
     */
    async queryTransactionBlocks(input) {
        return await this.transport.request({
            method: 'suix_queryTransactionBlocks',
            params: [
                {
                    filter: input.filter,
                    options: input.options,
                },
                input.cursor,
                input.limit,
                (input.order || 'descending') === 'descending',
            ],
        });
    }
    async getTransactionBlock(input) {
        if (!(0, sui_types_js_1.isValidTransactionDigest)(input.digest)) {
            throw new Error('Invalid Transaction digest');
        }
        return await this.transport.request({
            method: 'sui_getTransactionBlock',
            params: [input.digest, input.options],
        });
    }
    async multiGetTransactionBlocks(input) {
        input.digests.forEach((d) => {
            if (!(0, sui_types_js_1.isValidTransactionDigest)(d)) {
                throw new Error(`Invalid Transaction digest ${d}`);
            }
        });
        const hasDuplicates = input.digests.length !== new Set(input.digests).size;
        if (hasDuplicates) {
            throw new Error(`Duplicate digests in batch call ${input.digests}`);
        }
        return await this.transport.request({
            method: 'sui_multiGetTransactionBlocks',
            params: [input.digests, input.options],
        });
    }
    async executeTransactionBlock(input) {
        return await this.transport.request({
            method: 'sui_executeTransactionBlock',
            params: [
                typeof input.transactionBlock === 'string'
                    ? input.transactionBlock
                    : (0, bcs_1.toB64)(input.transactionBlock),
                Array.isArray(input.signature) ? input.signature : [input.signature],
                input.options,
                input.requestType,
            ],
        });
    }
    async signAndExecuteTransactionBlock({ transactionBlock, signer, ...input }) {
        let transactionBytes;
        if (transactionBlock instanceof Uint8Array) {
            transactionBytes = transactionBlock;
        }
        else {
            transactionBlock.setSenderIfNotSet(signer.toSuiAddress());
            transactionBytes = await transactionBlock.build({ client: this });
        }
        const { signature, bytes } = await signer.signTransactionBlock(transactionBytes);
        return this.executeTransactionBlock({
            transactionBlock: bytes,
            signature,
            ...input,
        });
    }
    /**
     * Get total number of transactions
     */
    async getTotalTransactionBlocks() {
        const resp = await this.transport.request({
            method: 'sui_getTotalTransactionBlocks',
            params: [],
        });
        return BigInt(resp);
    }
    /**
     * Getting the reference gas price for the network
     */
    async getReferenceGasPrice() {
        const resp = await this.transport.request({
            method: 'suix_getReferenceGasPrice',
            params: [],
        });
        return BigInt(resp);
    }
    /**
     * Return the delegated stakes for an address
     */
    async getStakes(input) {
        if (!input.owner || !(0, sui_types_js_1.isValidSuiAddress)((0, sui_types_js_1.normalizeSuiAddress)(input.owner))) {
            throw new Error('Invalid Sui address');
        }
        return await this.transport.request({ method: 'suix_getStakes', params: [input.owner] });
    }
    /**
     * Return the delegated stakes queried by id.
     */
    async getStakesByIds(input) {
        input.stakedSuiIds.forEach((id) => {
            if (!id || !(0, sui_types_js_1.isValidSuiObjectId)((0, sui_types_js_1.normalizeSuiObjectId)(id))) {
                throw new Error(`Invalid Sui Stake id ${id}`);
            }
        });
        return await this.transport.request({
            method: 'suix_getStakesByIds',
            params: [input.stakedSuiIds],
        });
    }
    /**
     * Return the latest system state content.
     */
    async getLatestSuiSystemState() {
        return await this.transport.request({ method: 'suix_getLatestSuiSystemState', params: [] });
    }
    /**
     * Get events for a given query criteria
     */
    async queryEvents(input) {
        return await this.transport.request({
            method: 'suix_queryEvents',
            params: [
                input.query,
                input.cursor,
                input.limit,
                (input.order || 'descending') === 'descending',
            ],
        });
    }
    /**
     * Subscribe to get notifications whenever an event matching the filter occurs
     */
    async subscribeEvent(input) {
        return this.transport.subscribe({
            method: 'suix_subscribeEvent',
            unsubscribe: 'suix_unsubscribeEvent',
            params: [input.filter],
            onMessage: input.onMessage,
        });
    }
    async subscribeTransaction(input) {
        return this.transport.subscribe({
            method: 'suix_subscribeTransaction',
            unsubscribe: 'suix_unsubscribeTransaction',
            params: [input.filter],
            onMessage: input.onMessage,
        });
    }
    /**
     * Runs the transaction block in dev-inspect mode. Which allows for nearly any
     * transaction (or Move call) with any arguments. Detailed results are
     * provided, including both the transaction effects and any return values.
     */
    async devInspectTransactionBlock(input) {
        let devInspectTxBytes;
        if ((0, index_js_1.isTransactionBlock)(input.transactionBlock)) {
            input.transactionBlock.setSenderIfNotSet(input.sender);
            devInspectTxBytes = (0, bcs_1.toB64)(await input.transactionBlock.build({
                client: this,
                onlyTransactionKind: true,
            }));
        }
        else if (typeof input.transactionBlock === 'string') {
            devInspectTxBytes = input.transactionBlock;
        }
        else if (input.transactionBlock instanceof Uint8Array) {
            devInspectTxBytes = (0, bcs_1.toB64)(input.transactionBlock);
        }
        else {
            throw new Error('Unknown transaction block format.');
        }
        return await this.transport.request({
            method: 'sui_devInspectTransactionBlock',
            params: [input.sender, devInspectTxBytes, input.gasPrice?.toString(), input.epoch],
        });
    }
    /**
     * Dry run a transaction block and return the result.
     */
    async dryRunTransactionBlock(input) {
        return await this.transport.request({
            method: 'sui_dryRunTransactionBlock',
            params: [
                typeof input.transactionBlock === 'string'
                    ? input.transactionBlock
                    : (0, bcs_1.toB64)(input.transactionBlock),
            ],
        });
    }
    /**
     * Return the list of dynamic field objects owned by an object
     */
    async getDynamicFields(input) {
        if (!input.parentId || !(0, sui_types_js_1.isValidSuiObjectId)((0, sui_types_js_1.normalizeSuiObjectId)(input.parentId))) {
            throw new Error('Invalid Sui Object id');
        }
        return await this.transport.request({
            method: 'suix_getDynamicFields',
            params: [input.parentId, input.cursor, input.limit],
        });
    }
    /**
     * Return the dynamic field object information for a specified object
     */
    async getDynamicFieldObject(input) {
        return await this.transport.request({
            method: 'suix_getDynamicFieldObject',
            params: [input.parentId, input.name],
        });
    }
    /**
     * Get the sequence number of the latest checkpoint that has been executed
     */
    async getLatestCheckpointSequenceNumber() {
        const resp = await this.transport.request({
            method: 'sui_getLatestCheckpointSequenceNumber',
            params: [],
        });
        return String(resp);
    }
    /**
     * Returns information about a given checkpoint
     */
    async getCheckpoint(input) {
        return await this.transport.request({ method: 'sui_getCheckpoint', params: [input.id] });
    }
    /**
     * Returns historical checkpoints paginated
     */
    async getCheckpoints(input) {
        return await this.transport.request({
            method: 'sui_getCheckpoints',
            params: [input.cursor, input?.limit, input.descendingOrder],
        });
    }
    /**
     * Return the committee information for the asked epoch
     */
    async getCommitteeInfo(input) {
        return await this.transport.request({
            method: 'suix_getCommitteeInfo',
            params: [input?.epoch],
        });
    }
    async getNetworkMetrics() {
        return await this.transport.request({ method: 'suix_getNetworkMetrics', params: [] });
    }
    async getAddressMetrics() {
        return await this.transport.request({ method: 'suix_getLatestAddressMetrics', params: [] });
    }
    async getEpochMetrics(input) {
        return await this.transport.request({
            method: 'suix_getEpochMetrics',
            params: [input?.cursor, input?.limit, input?.descendingOrder],
        });
    }
    async getAllEpochAddressMetrics(input) {
        return await this.transport.request({
            method: 'suix_getAllEpochAddressMetrics',
            params: [input?.descendingOrder],
        });
    }
    /**
     * Return the committee information for the asked epoch
     */
    async getEpochs(input) {
        return await this.transport.request({
            method: 'suix_getEpochs',
            params: [input?.cursor, input?.limit, input?.descendingOrder],
        });
    }
    /**
     * Returns list of top move calls by usage
     */
    async getMoveCallMetrics() {
        return await this.transport.request({ method: 'suix_getMoveCallMetrics', params: [] });
    }
    /**
     * Return the committee information for the asked epoch
     */
    async getCurrentEpoch() {
        return await this.transport.request({ method: 'suix_getCurrentEpoch', params: [] });
    }
    /**
     * Return the Validators APYs
     */
    async getValidatorsApy() {
        return await this.transport.request({ method: 'suix_getValidatorsApy', params: [] });
    }
    // TODO: Migrate this to `sui_getChainIdentifier` once it is widely available.
    async getChainIdentifier() {
        const checkpoint = await this.getCheckpoint({ id: '0' });
        const bytes = (0, bcs_1.fromB58)(checkpoint.digest);
        return (0, bcs_1.toHEX)(bytes.slice(0, 4));
    }
    async resolveNameServiceAddress(input) {
        return await this.transport.request({
            method: 'suix_resolveNameServiceAddress',
            params: [input.name],
        });
    }
    async resolveNameServiceNames(input) {
        return await this.transport.request({
            method: 'suix_resolveNameServiceNames',
            params: [input.address, input.cursor, input.limit],
        });
    }
    async getProtocolConfig(input) {
        return await this.transport.request({
            method: 'sui_getProtocolConfig',
            params: [input?.version],
        });
    }
    /**
     * Wait for a transaction block result to be available over the API.
     * This can be used in conjunction with `executeTransactionBlock` to wait for the transaction to
     * be available via the API.
     * This currently polls the `getTransactionBlock` API to check for the transaction.
     */
    async waitForTransactionBlock({ signal, timeout = 60 * 1000, pollInterval = 2 * 1000, ...input }) {
        const timeoutSignal = AbortSignal.timeout(timeout);
        const timeoutPromise = new Promise((_, reject) => {
            timeoutSignal.addEventListener('abort', () => reject(timeoutSignal.reason));
        });
        timeoutPromise.catch(() => {
            // Swallow unhandled rejections that might be thrown after early return
        });
        while (!timeoutSignal.aborted) {
            signal?.throwIfAborted();
            try {
                return await this.getTransactionBlock(input);
            }
            catch (e) {
                // Wait for either the next poll interval, or the timeout.
                await Promise.race([
                    new Promise((resolve) => setTimeout(resolve, pollInterval)),
                    timeoutPromise,
                ]);
            }
        }
        timeoutSignal.throwIfAborted();
        // This should never happen, because the above case should always throw, but just adding it in the event that something goes horribly wrong.
        throw new Error('Unexpected error while waiting for transaction block.');
    }
}
exports.DWalletClient = DWalletClient;
//# sourceMappingURL=client.js.map