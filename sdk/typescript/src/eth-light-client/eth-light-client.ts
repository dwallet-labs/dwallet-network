// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import {
	get_current_period,
	get_initial_state_bcs,
	try_verify_proof,
} from '@dwallet-network/eth-light-client-wasm';
import { ethers } from 'ethers';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { getSharedObjectRefById } from '../utils/sui-types.js';
import {
	getBeaconBlockData,
	getBootstrapData,
	getFinalityUpdate,
	getOptimisticUpdate,
	getProof,
	getUpdates,
} from './rpc.js';
import type { EthereumState } from './utils.js';
import {
	getAuthorityBinderByID,
	getAuthorityByID,
	getEthereumStateById,
	stringToArrayU8Bcs,
} from './utils.js';

const packageId = '0x3';
const ethereumStateModuleName = 'ethereum_authority';

/**
 * Creates a new Ethereum authority.
 *
 * This function constructs a transaction block to call the `create_ethereum_authority`
 * function in the Ethereum state module.
 *
 * @param {string} authorityName - The name of the Ethereum authority.
 * @param {string} chainIdentifier - The chain identifier for the Ethereum network.
 * @param {string} configObjID - The ObjectID of the configuration object.
 * @param {string} authorityOwnerDWalletCapID - The ObjectID of the authority owner dWallet capability.
 * @param {string} network - The network identifier (e.g., 'mainnet', 'holesky').
 * @param {string} rpc - The Ethereum RPC endpoint.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 * @returns The ObjectID of the created Ethereum authority.
 * @throws Will throw an error if the network is invalid or if the transaction fails to verify the Ethereum state.
 */
export const createEthereumAuthority = async (
	authorityName: string,
	chainIdentifier: string,
	configObjID: string,
	authorityOwnerDWalletCapID: string,
	network: string,
	rpc: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let checkpoint = '';
	switch (network) {
		case 'mainnet': {
			checkpoint = '0x886083d6ba589617fabc0e69127982299f60426ddbf863ade18b3dd30259c11d';
			break;
		}
		case 'holesky': {
			checkpoint = '0x089ad025c4a629091ea8ff20ba34f3eaf5b2c690f1a9e2c29a64022d95ddf1a4';
			break;
		}
		default: {
			throw new Error('Invalid network');
		}
	}
	let bootstrapJson = await getBootstrapData(rpc, checkpoint);
	let bootstrap = bootstrapJson['data'];

	let state = get_initial_state_bcs(checkpoint, rpc, network, bootstrap);
	let stateBytes: Uint8Array = state['bytes'];
	let syncPeriod = get_current_period(stateBytes);

	let updatesResponseJson = await getUpdates(rpc, syncPeriod);
	let updatesJson = JSON.stringify(updatesResponseJson.map((update: any) => update['data']));
	let updatesBcs = stringToArrayU8Bcs(updatesJson);

	let finalityUpdateResponse = await getFinalityUpdate(rpc);
	let finalityUpdateJson = JSON.stringify(finalityUpdateResponse['data']);
	let finalityUpdateBcs = stringToArrayU8Bcs(finalityUpdateJson);

	let optimisticUpdateResponse = await getOptimisticUpdate(rpc);
	let optimisticUpdateJson = JSON.stringify(optimisticUpdateResponse['data']);
	let optimisticUpdateBcs = stringToArrayU8Bcs(optimisticUpdateJson);

	let stateBcs = bcs.vector(bcs.u8()).serialize(stateBytes, {
		size: stateBytes.length,
		maxSize: stateBytes.length * 2,
		allocateSize: stateBytes.length,
	});

	// Get Beacon block data for the latest finalized block.
	let beaconBlockData = await getBeaconBlockData(rpc, finalityUpdateResponse);
	let beaconBlockTypeBcs = stringToArrayU8Bcs(beaconBlockData.blockType);
	let beaconBlockBcs = stringToArrayU8Bcs(beaconBlockData.blockJsonString);
	let beaconBlockBodyBcs = stringToArrayU8Bcs(beaconBlockData.blockBodyJsonString);
	let beaconBlockExecutionPayloadBcs = stringToArrayU8Bcs(
		beaconBlockData.blockExecutionPayloadJsonString,
	);

	let chainIdentifierBcs = stringToArrayU8Bcs(chainIdentifier);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${ethereumStateModuleName}::create_ethereum_authority`,
		arguments: [
			tx.pure.string(authorityName),
			tx.pure(chainIdentifierBcs),
			tx.object(configObjID),
			tx.object(authorityOwnerDWalletCapID),
			tx.pure(stateBcs),
			tx.pure(updatesBcs),
			tx.pure(finalityUpdateBcs),
			tx.pure(optimisticUpdateBcs),
			tx.pure(beaconBlockBcs),
			tx.pure(beaconBlockBodyBcs),
			tx.pure(beaconBlockExecutionPayloadBcs),
			tx.pure(beaconBlockTypeBcs),
		],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (result.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to verify Ethereum state. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}

	return result.effects?.created?.filter(
		(o) =>
			typeof o.owner === 'object' &&
			'Shared' in o.owner &&
			o.owner.Shared.initial_shared_version !== undefined,
	)[0].reference.objectId!;
};

/**
 * Creates a new Ethereum smart contract configuration.
 *
 * This function constructs a transaction block to call the `create_ethereum_smart_contract_config`
 * function in the Ethereum state module.
 *
 * @param {number} contractApprovedTxSlot - The slot number of the approved transaction.
 * @param {string} network - The network identifier (e.g., 'mainnet', 'holesky').
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 * @returns The ObjectID of the created Ethereum smart contract configuration.
 * @throws Will throw an error if the transaction fails to verify the Ethereum state.
 */
export const createEthereumSmartContractConfig = async (
	contractApprovedTxSlot: number,
	network: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let networkBcs = stringToArrayU8Bcs(network);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${ethereumStateModuleName}::create_ethereum_smart_contract_config`,
		arguments: [tx.pure.u64(contractApprovedTxSlot), tx.pure(networkBcs)],
		typeArguments: [],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (result.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to verify Ethereum state. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}

	return result.effects?.created?.at(0)?.reference?.objectId!;
};

/**
 * Updates the Ethereum authority state.
 *
 * This function fetches the latest updates from the Ethereum network and updates the state
 * of the Ethereum authority. It constructs a transaction block to call the
 * `update_authority_state` function in the Ethereum state module.
 *
 * @param {string} authorityId - The ObjectID of the Ethereum authority.
 * @param {EthereumState} currentEthereumStateObj - The current Ethereum state object.
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 * @param {DWalletClient} client - The dWallet client instance.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @throws Will throw an error if the transaction fails to verify the Ethereum state.
 */
async function updateEthereumAuthorityState(
	authorityId: string,
	currentEthereumStateObj: EthereumState,
	consensusRpc: string,
	client: DWalletClient,
	keypair: Keypair,
) {
	let currentEthereumStateData = currentEthereumStateObj?.data as number[];
	let currentEthereumStateArrayU8 = Uint8Array.from(currentEthereumStateData);

	let syncPeriod = get_current_period(currentEthereumStateArrayU8);

	let updatesResponseJson = await getUpdates(consensusRpc, syncPeriod);
	let updatesJson = JSON.stringify(updatesResponseJson.map((update: any) => update['data']));
	let updatesBcs = stringToArrayU8Bcs(updatesJson);

	let finalityUpdateResponseJson = await getFinalityUpdate(consensusRpc);
	let finalityUpdateJson = JSON.stringify(finalityUpdateResponseJson['data']);
	let finalityUpdateBcs = stringToArrayU8Bcs(finalityUpdateJson);

	let optimisticUpdateResponse = await getOptimisticUpdate(consensusRpc);
	let optimisticUpdateJson = JSON.stringify(optimisticUpdateResponse['data']);
	let optimisticUpdateBcs = stringToArrayU8Bcs(optimisticUpdateJson);

	let beaconBlockData = await getBeaconBlockData(consensusRpc, finalityUpdateResponseJson);
	let beaconBlockTypeBcs = stringToArrayU8Bcs(beaconBlockData.blockType);
	let beaconBlockBcs = stringToArrayU8Bcs(beaconBlockData.blockJsonString);
	let beaconBlockBodyBcs = stringToArrayU8Bcs(beaconBlockData.blockBodyJsonString);
	let beaconBlockExecutionPayloadBcs = stringToArrayU8Bcs(
		beaconBlockData.blockExecutionPayloadJsonString,
	);

	let authoritySharedObjectRef = await getSharedObjectRefById(authorityId, client, true);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${ethereumStateModuleName}::update_authority_state`,
		arguments: [
			tx.sharedObjectRef(authoritySharedObjectRef),
			tx.object(currentEthereumStateObj.id.id),
			tx.pure(updatesBcs),
			tx.pure(finalityUpdateBcs),
			tx.pure(optimisticUpdateBcs),
			tx.pure(beaconBlockBcs),
			tx.pure(beaconBlockBodyBcs),
			tx.pure(beaconBlockExecutionPayloadBcs),
			tx.pure(beaconBlockTypeBcs),
		],
	});

	let txResult = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (txResult.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to verify Ethereum state. Transaction effects: ' + JSON.stringify(txResult.effects),
		);
	}
}

/**
 * Approves an Ethereum transaction for a given dWallet.
 *
 * Interacts with the Ethereum light client to verify and approve a transaction message
 * using an Ethereum smart contract linked to a dWallet within the dWallet blockchain context.
 * The verification of the state and message is done offline, inside the dWallet module.
 *
 * **Logic**
 * 1. **Retrieve Configuration**: Starts by retrieving the latest Ethereum state object.
 * 2. **Fetch Ethereum Objects**: Retrieves and deserializes the latest Ethereum state and the current Ethereum state data to collect the latest Ethereum state data.
 * 3. **Initialize Light Client**: Initializes the Ethereum light client with the deserialized Ethereum state.
 * 4. **Prepare Proof Parameters**: Constructs proof request parameters using the message, dWallet ID, and data slot from the latest Ethereum state object.
 * 5. **Fetch Updates and Proofs**: Retrieves the necessary updates and cryptographic proofs from the Ethereum light client.
 * 6. **Build Transaction**: Uses the transaction builder to serialize transaction parameters, including the Ethereum state, updates, and shared state object, and prepares the transaction to call the `verify_new_state` function in the Ethereum state module.
 * 7. **Send Transaction**: Constructs the transaction data, including the proof and dWallet ID, and executes it.
 *
 * **Arguments**
 * @param {string} authorityId - The ObjectID of the Ethereum authority.
 * @param {string} message - The Ethereum transaction message to be approved.
 * @param {string} dWalletID - The ObjectID of the dWallet to which the transaction belongs.
 * @param {string} dwalletBinderId - The ObjectID of the dWallet binder.
 * @param {string} executionRpc - The Ethereum execution RPC endpoint.
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 * @returns The result of the message approval transaction.
 * @throws Will throw an error if the transaction fails to verify the Ethereum state.
 */
export const approveEthereumMessage = async (
	authorityId: string,
	dwalletBinderId: string,
	message: string,
	dWalletID: string,
	executionRpc: string,
	consensusRpc: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let authorityObj = await getAuthorityByID(authorityId, client);
	let currentEthereumStateID = authorityObj?.latest.id as string;
	let currentEthereumStateObj = await getEthereumStateById(client, currentEthereumStateID);

	let dwalletBinderObj = await getAuthorityBinderByID(dwalletBinderId, client);
	let bindToAuthorityObj = dwalletBinderObj?.bind_to_authority;

	let dataSlot = authorityObj?.config.approved_tx_slot as number;
	let contractAddress = bindToAuthorityObj?.owner as number[];
	let contractAddressArrayU8 = Uint8Array.from(contractAddress);
	let contractAddressString = ethers.hexlify(contractAddressArrayU8);

	// Get the proof for the (message + dWalletID).
	let proof = await getProof(
		message,
		dWalletID,
		dataSlot,
		contractAddressString,
		currentEthereumStateObj?.block_number as number,
		executionRpc,
	);
	let state_root = currentEthereumStateObj?.state_root as number[];

	let successful_proof = try_verify_proof(
		proof,
		contractAddressString,
		message,
		ethers.getBytes(dWalletID),
		dataSlot,
		state_root,
	);

	// If the proof has failed, then we need to update the state and try again.
	if (!successful_proof) {
		await updateEthereumAuthorityState(
			authorityId,
			currentEthereumStateObj!,
			consensusRpc,
			client,
			keypair,
		);

		// Get the latest Ethereum state again, to get the updated state after it is verified.
		authorityObj = await getAuthorityByID(authorityId, client);
		currentEthereumStateID = authorityObj?.latest.id as string;
		currentEthereumStateObj = await getEthereumStateById(client, currentEthereumStateID);

		// Get the proof again, using the updated state.
		proof = await getProof(
			message,
			dWalletID,
			dataSlot,
			contractAddressString,
			currentEthereumStateObj?.block_number as number,
			executionRpc,
		);
	}

	let dWalletBinderSharedObjectRef = await getSharedObjectRefById(dwalletBinderId, client);
	let authoritySharedObjectRef = await getSharedObjectRefById(authorityId, client);
	let proofBcs = stringToArrayU8Bcs(JSON.stringify(proof));
	let messageBcs = stringToArrayU8Bcs(message);

	const tx = new TransactionBlock();
	const [messageApprovals] = tx.moveCall({
		target: `${packageId}::${ethereumStateModuleName}::approve_message`,
		arguments: [
			tx.sharedObjectRef(authoritySharedObjectRef),
			tx.sharedObjectRef(dWalletBinderSharedObjectRef),
			tx.object(currentEthereumStateID),
			tx.pure(messageBcs),
			tx.object(dWalletID),
			tx.pure(proofBcs),
		],
	});

	let txResult = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (txResult.effects?.status.status !== 'success') {
		throw new Error('Failed to verify Ethereum state');
	}
	return messageApprovals;
};
