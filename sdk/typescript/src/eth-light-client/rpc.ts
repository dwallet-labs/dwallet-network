// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ssz } from '@lodestar/types';
import { ethers } from 'ethers';

import { calculateMessageStorageSlot, compareUint8Arrays, keysToSnakeCase } from './utils.js';

// Maximum number of light client updates per request
const MAX_REQUEST_LIGHT_CLIENT_UPDATES = 128;

interface VerifiedFinalityHeader {
	version: 'phase0' | 'altair' | 'bellatrix' | 'capella' | 'deneb';
	data: any;
}

type BeaconBlockData = {
	blockJsonString: string;
	blockBodyJsonString: string;
	blockExecutionPayloadJsonString: string;
	blockType: string;
	latestFinalizedBlockNumber: number;
};

/**
 * Retrieves a Merkle proof for a specific storage slot in an Ethereum smart contract.
 *
 * **Logic**
 * 1. **Calculate Storage Slot**: Calculates the storage slot for the given message and dWallet ID.
 * 2. **Fetch Proof**: Calls `getProofByStorageSlot` to retrieve the Merkle proof from the Ethereum execution client.
 *
 * **Arguments**
 * @param {string} message - The message associated with the storage slot.
 * @param {string} dwalletID - The dWallet ID.
 * @param {number} dataSlot - The data slot in the smart contract's storage.
 * @param {string} contractAddress - The Ethereum smart contract address.
 * @param {number} latestFinalizedBlockNumber - The latest finalized block number to query.
 * @param {string} executionRpc - The Ethereum execution RPC endpoint.
 *
 * **Returns**
 * The proof object retrieved from the Ethereum execution client.
 */
export async function getProof(
	message: string,
	dwalletID: string,
	dataSlot: number,
	contractAddress: string,
	latestFinalizedBlockNumber: number,
	executionRpc: string,
) {
	let storageSlot = calculateMessageStorageSlot(message, dwalletID, dataSlot);
	return await getProofByStorageSlot(
		executionRpc,
		contractAddress,
		[storageSlot],
		latestFinalizedBlockNumber,
	);
}

/**
 * Retrieves beacon block data and processes it for use in the dWallet verification process.
 *
 * **Logic**
 * 1. **Get Verified Beacon Block**: Calls `getVerifiedBeaconBlock` to retrieve and verify the beacon block.
 * 2. **Convert Keys**: Converts keys in the block data to snake_case.
 * 3. **Serialize Block Data**: Serializes the block data, excluding certain fields for efficiency.
 * 4. **Extract Execution Payload**: Extracts the execution payload from the block body.
 * 5. **Prepare Data for Transaction**: Prepares the block data strings and block type for inclusion in a transaction.
 *
 * **Arguments**
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 * @param {VerifiedFinalityHeader} finalityUpdateJson - The finality update header used for verification.
 *
 * **Returns**
 * An object containing serialized block data strings, block type, and the latest finalized block number.
 * Note that the block data strings are serialized JSON objects, excluding certain fields for proper deserialization.
 *
 * **Errors**
 * Throws an error if the beacon block cannot be retrieved or verified.
 */
export async function getBeaconBlockData(
	consensusRpc: string,
	finalityUpdateJson: VerifiedFinalityHeader,
) {
	let block = await getVerifiedBeaconBlock(consensusRpc, finalityUpdateJson);
	block = keysToSnakeCase(block);

	let blockJsonString = JSON.stringify(block, (key, value) => {
		if (key === 'body') {
			return undefined;
		}
		return value;
	});

	let blockBody = block.body;
	let blockBodyJsonString = JSON.stringify(blockBody, (key, value) => {
		if (key === 'execution_payload') {
			return undefined;
		}
		return value;
	});

	let blockExecutionPayload = blockBody.execution_payload;
	let blockExecutionPayloadJsonString = JSON.stringify(blockExecutionPayload);

	let result: BeaconBlockData = {
		blockJsonString: blockJsonString,
		blockBodyJsonString: blockBodyJsonString,
		blockExecutionPayloadJsonString: blockExecutionPayloadJsonString,
		blockType: finalityUpdateJson['version'],
		latestFinalizedBlockNumber: blockExecutionPayload.block_number,
	};

	return result;
}

/**
 * Retrieves light client updates from the Ethereum consensus client starting from a specific sync period.
 *
 * **Arguments**
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 * @param {number} syncPeriod - The starting sync period for fetching updates.
 *
 * **Returns**
 * The JSON response containing the light client updates.
 */
export async function getUpdates(consensusRpc: string, syncPeriod: number) {
	let reqUrl = `${consensusRpc}/eth/v1/beacon/light_client/updates?start_period=${syncPeriod}&count=${MAX_REQUEST_LIGHT_CLIENT_UPDATES}`;
	const response = await fetch(reqUrl);
	if (!response.ok) {
		throw new Error(`could not fetch updates. HTTP Response status: ${response.status}`);
	}

	return await response.json();
}

/**
 * Retrieves the latest finality update from the Ethereum consensus client.
 *
 * **Arguments**
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 *
 * **Returns**
 * The JSON response containing the finality update.
 */
export async function getFinalityUpdate(consensusRpc: string) {
	let reqUrl = `${consensusRpc}/eth/v1/beacon/light_client/finality_update`;
	const response = await fetch(reqUrl);
	if (!response.ok) {
		throw new Error(`could not fetch finality update. HTTP Response status: ${response.status}`);
	}

	return await response.json();
}

/**
 * Retrieves the latest optimistic update from the Ethereum consensus client.
 *
 * **Arguments**
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 *
 * **Returns**
 * The JSON response containing the optimistic update.
 */
export async function getOptimisticUpdate(consensusRpc: string) {
	let reqUrl = `${consensusRpc}/eth/v1/beacon/light_client/finality_update`;
	const response = await fetch(reqUrl);
	if (!response.ok) {
		throw new Error(`could not fetch optimistic update. HTTP Response status: ${response.status}`);
	}

	return await response.json();
}

/**
 * Retrieves bootstrap data for initializing the Ethereum light client.
 *
 * **Arguments**
 * @param {string} rpc - The Ethereum consensus RPC endpoint.
 * @param {string} checkpoint - The checkpoint root (block root) to fetch the bootstrap data for.
 *
 * **Returns**
 * The JSON response containing the bootstrap data.
 */
export async function getBootstrapData(rpc: string, checkpoint: string) {
	const reqUrl = `${rpc}/eth/v1/beacon/light_client/bootstrap/${checkpoint}`;
	const response = await fetch(reqUrl);
	if (!response.ok) {
		throw new Error(`could not fetch bootstrap data. HTTP Response status: ${response.status}`);
	}

	return await response.json();
}

/**
 * Retrieves and verifies a beacon block corresponding to a verified finality header.
 *
 * **Logic**
 * 1. **Determine Block Type**: Determines the block type from the `version` in the finality header.
 * 2. **Deserialize Finalized Header**: Deserializes the finalized header from the finality update.
 * 3. **Fetch Beacon Block**: Retrieves the beacon block corresponding to the slot in the finalized header.
 * 4. **Compute Hashes**: Computes the hash tree root of the beacon block and the finalized header.
 * 5. **Verify Match**: Compares the hashes to ensure the beacon block matches the finalized header.
 *
 * **Arguments**
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 * @param {VerifiedFinalityHeader} verifiedFinalityHeader - The verified finality header.
 *
 * **Returns**
 * The beacon block data if verification succeeds.
 */
export const getVerifiedBeaconBlock = async (
	consensusRpc: string,
	verifiedFinalityHeader: VerifiedFinalityHeader,
) => {
	let blockType = verifiedFinalityHeader['version'];
	const finalizedHeader = ssz.deneb.LightClientHeader.fromJson(
		verifiedFinalityHeader['data']['finalized_header'],
	);
	let verifiedFinalizedHeaderHash = ssz.phase0.BeaconBlockHeader.hashTreeRoot(
		finalizedHeader.beacon,
	);

	let slot = finalizedHeader['beacon']['slot'];
	let beaconBlock = await getBeaconBlock(consensusRpc, slot);
	let beaconBlockData = beaconBlock['data']['message'];

	let beaconBlockHash;
	let block;
	switch (blockType) {
		case 'phase0':
			block = ssz.phase0.BeaconBlock.fromJson(beaconBlockData);
			beaconBlockHash = ssz.phase0.BeaconBlock.hashTreeRoot(block);
			break;
		case 'altair':
			block = ssz.altair.BeaconBlock.fromJson(beaconBlockData);
			beaconBlockHash = ssz.altair.BeaconBlock.hashTreeRoot(block);
			break;
		case 'bellatrix':
			block = ssz.bellatrix.BeaconBlock.fromJson(beaconBlockData);
			beaconBlockHash = ssz.bellatrix.BeaconBlock.hashTreeRoot(block);
			break;
		case 'capella':
			block = ssz.capella.BeaconBlock.fromJson(beaconBlockData);
			beaconBlockHash = ssz.capella.BeaconBlock.hashTreeRoot(block);
			break;
		case 'deneb':
			block = ssz.deneb.BeaconBlock.fromJson(beaconBlockData);
			beaconBlockHash = ssz.deneb.BeaconBlock.hashTreeRoot(block);
			break;
		default:
			throw new Error('Invalid block type');
	}

	if (!compareUint8Arrays(beaconBlockHash, verifiedFinalizedHeaderHash)) {
		throw new Error('Finality header does not match block');
	}

	return beaconBlockData;
};

async function getProofByStorageSlot(
	executionRpc: string,
	contractAddress: string,
	slots: string[],
	latestFinalizedBlockNumber: number,
) {
	let address;
	let provider = new ethers.JsonRpcProvider(executionRpc);
	if (!ethers.isAddress(contractAddress)) {
		// Resolve ENS name to address
		address = await provider.resolveName(contractAddress);
		if (address === null) {
			throw new Error('Invalid contract address');
		}
	}

	let blockNumber =
		latestFinalizedBlockNumber <= 0 ? 'latest' : ethers.toQuantity(latestFinalizedBlockNumber);

	return await provider.send('eth_getProof', [address, slots, blockNumber]);
}

async function getBeaconBlock(consensusRpc: string, slot: number) {
	let reqUrl = `${consensusRpc}/eth/v2/beacon/blocks/${slot}`;
	const response = await fetch(reqUrl);
	if (!response.ok) {
		throw new Error(`could not fetch beacon block. HTTP Response status: ${response.status}`);
	}

	return await response.json();
}
