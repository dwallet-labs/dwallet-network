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
import {
	getBeaconBlockData,
	getBootstrapData,
	getFinalityUpdate,
	getOptimisticUpdate,
	getProof,
	getUpdates,
} from './rpc.js';
import { getEthereumStateById, getLatestEthereumStateById, stringToArrayU8Bcs } from './utils.js';

const packageId = '0x3';
const ethDWalletModuleName = 'eth_dwallet';
const ethereumStateModuleName = 'ethereum_state';

/**
 * Connects a dWallet to be controlled by an Ethereum smart contract.
 *
 * This function links a dWallet within the dWallet blockchain environment to an Ethereum smart contract.
 * By creating an Ethereum dWallet capability, it allows the dWallet to interact with Ethereum transactions
 * and be managed through the specified smart contract.
 *
 * **Arguments**
 * @param {string} dwalletCapId - The ObjectID of the dWallet capability.
 * @param {string} latestEthereumStateId - The ObjectID of the latest Ethereum state.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 */
export const createEthereumDWallet = async (
	dwalletCapId: string,
	latestEthereumStateId: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${ethDWalletModuleName}::create_eth_dwallet_cap`,
		arguments: [tx.object(dwalletCapId), tx.object(latestEthereumStateId)],
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

	return result.effects?.created?.at(0)?.reference.objectId;
};

/**
 * Initializes a shared LatestEthereumState object in the dWallet network with the given checkpoint.
 *
 * This function should only be called once to initialize the Ethereum state. After the state is initialized,
 * the Ethereum state object ID is saved, and the state is updated whenever a new state is successfully verified.
 *
 * **Logic**
 * 1. **Select Checkpoint**: Determines the initial checkpoint based on the specified Ethereum network.
 * 2. **Fetch Bootstrap Data**: Retrieves the bootstrap data required to initialize the Ethereum light client state.
 * 3. **Initialize State**: Uses the bootstrap data to initialize the Ethereum light client state in BCS format.
 * 4. **Fetch Updates**: Retrieves updates from the Ethereum consensus RPC since the initial sync period.
 * 5. **Prepare Transaction**: Constructs a transaction to call the `init_state` function in the Ethereum state module,
 *    providing the necessary arguments such as the state bytes, network, contract address, and updates.
 * 6. **Execute Transaction**: Signs and executes the transaction to initialize the Ethereum state on the dWallet network.
 *
 * **Arguments**
 * @param {string} network - The Ethereum network to initialize (e.g., 'mainnet' or 'holesky').
 * @param {string} rpc - The Ethereum consensus RPC endpoint.
 * @param {string} contractAddress - The address of the Ethereum smart contract.
 * @param {number} contractApprovedTxSlot - The slot of the data structure that holds approved transactions in the Ethereum smart contract.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 */
export const initEthereumState = async (
	network: string,
	rpc: string,
	contractAddress: string,
	contractApprovedTxSlot: number,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let checkpoint = '';
	switch (network) {
		case 'mainnet': {
			checkpoint = '0x8bfa089414dc5fe78dadc8b160a097fe744f17a80251f08eed0a3cdcc60b42f4';
			break;
		}
		case 'holesky': {
			checkpoint = '0x12e6b81891d23c90502dbc2354de9cb52afe4ff823ca00fd41d411213c6e7bbb';
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

	let contractAddressArrayU8 = ethers.getBytes(contractAddress);
	let contractAddressBcs = bcs.vector(bcs.u8()).serialize(contractAddressArrayU8);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${ethereumStateModuleName}::init_state`,
		arguments: [
			tx.pure(stateBcs),
			tx.pure(network),
			tx.pure(contractAddressBcs),
			tx.pure.u64(contractApprovedTxSlot),
			tx.pure(updatesBcs),
			tx.pure(finalityUpdateBcs),
			tx.pure(optimisticUpdateBcs),
			tx.pure(beaconBlockBcs),
			tx.pure(beaconBlockBodyBcs),
			tx.pure(beaconBlockExecutionPayloadBcs),
			tx.pure(beaconBlockTypeBcs),
		],
		typeArguments: [],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	return result.effects?.created?.filter(
		(o) =>
			typeof o.owner === 'object' &&
			'Shared' in o.owner &&
			o.owner.Shared.initial_shared_version !== undefined,
	)[0].reference!.objectId!;
};

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
 * @param {string} ethDwalletCapId - The ObjectID of the Ethereum dWallet capability, representing the link between the dWallet and Ethereum.
 * @param {string} message - The Ethereum transaction message to be approved.
 * @param {string} dWalletID - The ObjectID of the dWallet to which the transaction belongs.
 * @param {string} latestStateObjectID - The ObjectID of the latest Ethereum state.
 * @param {string} executionRpc - The Ethereum execution RPC endpoint.
 * @param {string} consensusRpc - The Ethereum consensus RPC endpoint.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 */
export const approveEthereumMessage = async (
	ethDwalletCapId: string,
	message: string,
	dWalletID: string,
	latestStateObjectID: string,
	executionRpc: string,
	consensusRpc: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let latestEthereumStateObj = await getLatestEthereumStateById(client, latestStateObjectID);
	let currentEthereumStateID = latestEthereumStateObj?.eth_state_id as string;
	let currentEthereumStateObj = await getEthereumStateById(client, currentEthereumStateID);
	let currentEthereumStateData = currentEthereumStateObj?.data as number[];
	let currentEthereumStateArrayU8 = Uint8Array.from(currentEthereumStateData);

	let dataSlot = latestEthereumStateObj?.eth_smart_contract_slot as number;
	let contractAddress = latestEthereumStateObj?.eth_smart_contract_address as number[];
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

		const tx = new TransactionBlock();
		tx.moveCall({
			target: `${packageId}::${ethereumStateModuleName}::verify_new_state`,
			arguments: [
				tx.pure(updatesBcs),
				tx.pure(finalityUpdateBcs),
				tx.pure(optimisticUpdateBcs),
				tx.object(latestStateObjectID),
				tx.object(currentEthereumStateID),
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

		// Get the latest Ethereum state again, to get the updated state after it is verified.
		latestEthereumStateObj = await getLatestEthereumStateById(client, latestStateObjectID);
		currentEthereumStateID = latestEthereumStateObj?.eth_state_id as string;
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

	// Retry the verification with the updated state. If it fails again, an error will be returned.
	successful_proof = try_verify_proof(
		proof,
		contractAddressString,
		message,
		ethers.getBytes(dWalletID),
		dataSlot,
		state_root,
	);

	if (!successful_proof) {
		throw new Error('Failed to verify Ethereum state');
	}

	let proofBcs = stringToArrayU8Bcs(JSON.stringify(proof));
	let messageBcs = stringToArrayU8Bcs(message);

	const tx2 = new TransactionBlock();
	tx2.moveCall({
		target: `${packageId}::${ethDWalletModuleName}::approve_message`,
		arguments: [
			tx2.object(ethDwalletCapId),
			tx2.pure(messageBcs),
			tx2.object(dWalletID),
			tx2.object(latestStateObjectID),
			tx2.object(currentEthereumStateID),
			tx2.pure(proofBcs),
		],
	});

	let res = await client.devInspectTransactionBlock({
		sender: keypair.toSuiAddress(),
		transactionBlock: tx2,
	});

	const messageApprovalBcs = new Uint8Array(
		res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[],
	);

	let txResult = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx2,
		options: { showEffects: true },
	});

	if (txResult.effects?.status.status !== 'success') {
		throw new Error('Failed to verify Ethereum state');
	}

	return messageApprovalBcs;
};
