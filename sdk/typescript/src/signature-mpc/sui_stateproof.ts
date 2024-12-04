// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { recovery_id_sha256 as recoveryIdSha256 } from '@dwallet-network/signature-mpc-wasm';
import { bcs } from '@mysten/bcs';
import type { SuiClient } from '@mysten/sui.js/client';
import { TransactionBlock as TransactionBlockSui } from '@mysten/sui.js/transactions';
import axios from 'axios';
import { ethers } from 'ethers';
import { assert } from 'vitest';

import { createAuthorityAck } from '../authority-binder/authority-binder.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient, EventId, SuiEventFilter } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { getDWalletBinderByID } from '../eth-light-client/utils.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import { getSharedObjectRefById } from '../utils/sui-types.js';
import type { CreatedDwallet } from './dwallet_2pc_mpc_ecdsa_k1_module.js';
import {
	approveAndSignAckWithAuthority,
	getDwalletByObjID,
	signAndVerifySignature,
} from './dwallet.js';
import { presignWithDWalletID } from './sign.js';

const packageId = '0x3';
const suiStateProofModuleName = 'sui_state_proof';
const suiDWalletCapModuleName = 'dwallet_test1';
const suiChainType = 'Sui';

const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

const SHA256 = 'SHA256';
const GAS_BUDGET = 100_000_000;

const dWalletCapPackageAddressInSui =
	// '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec';
	// '0xecdd1511f3debf876e98534d44c638c6e77f21b92d82f464c6902d4b4a82a1ae';
	'0x1a6dd0f1ed4bc7299ad82a272e5bbb0c8280d721ce89a7698b0260cb57dca4e2';

type TxDataResponse = {
	ckp_epoch_id: number;
	checkpoint_summary_bytes: Uint8Array;
	checkpoint_contents_bytes: Uint8Array;
	transaction_bytes: Uint8Array;
};

/**
 * Submits a proof of dWallet creation (in Sui blockchain) to the dWallet Network.
 *
 * This function retrieves the transaction block associated with the provided digest
 * from Sui blockchain, queries additional transaction data, and submits it to
 * the dWallet Network for verification.
 *
 * @param {DWalletClient} dWalletClient - The dWallet client instance.
 * @param {SuiClient} suiClient - The Sui client instance.
 * @param {string} authorityId - The ID of the authority.
 * @param {string} dWalletBinder - The dWallet binder ID.
 * @param {string} createDWalletTxDigest - The digest of the transaction that created the dWallet.
 * @param {string} serviceUrl - The URL of the service to query transaction data.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @throws {Error} - Throws an error if the checkpoint is undefined or null.
 */
export async function submitDWalletCreationProof(
	dWalletClient: DWalletClient,
	suiClient: SuiClient,
	authorityId: string,
	dWalletBinder: string,
	createDWalletTxDigest: string,
	serviceUrl: string,
	keypair: Keypair,
) {
	let tx = await suiClient.getTransactionBlock({
		digest: createDWalletTxDigest,
		options: {},
	});

	let seq = tx.checkpoint;

	if (!seq) {
		throw new Error('Checkpoint is undefined or null');
	}

	let { ckp_epoch_id, checkpoint_summary_bytes, checkpoint_contents_bytes, transaction_bytes } =
		await queryTxData(createDWalletTxDigest, serviceUrl);

	let txb = new TransactionBlock();

	let dWalletBinderSharedRef = await getSharedObjectRefById(dWalletBinder, dWalletClient, true);
	let dWalletBinderArg = txb.sharedObjectRef(dWalletBinderSharedRef);

	let authoritySharedRef = await getSharedObjectRefById(authorityId, dWalletClient);
	let authorityArg = txb.sharedObjectRef(authoritySharedRef);

	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(dWalletClient, ckp_epoch_id - 1);
	let epochCommitteeObject = await getImmutableOrOwnedObject(dWalletClient, epoch_committee_id);

	let committeArg = txb.object(epochCommitteeObject);
	let checkpoint_arg = txb.pure(checkpoint_summary_bytes);
	let checkpoint_contents_arg = txb.pure(checkpoint_contents_bytes);
	let transaction_arg = txb.pure(transaction_bytes);

	txb.moveCall({
		target: `${packageId}::${suiStateProofModuleName}::verify_dwallet_cap_and_sui_cap_match`,
		arguments: [
			authorityArg,
			dWalletBinderArg,
			committeArg,
			checkpoint_arg,
			checkpoint_contents_arg,
			transaction_arg,
		],
	});
	return dWalletClient.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: txb,
		options: {
			showEffects: true,
		},
	});
}

/**
 * Approves a message in the Sui blockchain.
 *
 * This function creates a transaction block to approve a message in the Sui blockchain
 * using the provided dWallet capability ID, message to sign, and other necessary parameters.
 *
 * @param {string} suiDWalletCapId - The ID of the dWallet capability in Sui.
 * @param {string} messagesToSign - The message to be signed and approved.
 * @param {SuiClient} suiClient - The Sui client instance.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @returns {Promise<string>} - A promise that resolves to the transaction digest.
 * @throws {Error} - Throws an error if the transaction fails to approve the message.
 */
export async function approveMessageInSui(
	suiDWalletCapId: string,
	messagesToSign: Uint8Array[],
	suiClient: SuiClient,
	keypair: Keypair,
): Promise<string> {
	let txb = new TransactionBlockSui();

	const dWalletCapArg = txb.pure(suiDWalletCapId);
	const signMsgArg = txb.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messagesToSign));

	txb.moveCall({
		target: `${dWalletCapPackageAddressInSui}::${suiDWalletCapModuleName}::approve_message`,
		arguments: [dWalletCapArg, signMsgArg],
	});

	txb.setGasBudget(GAS_BUDGET);
	let result = await suiClient.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: txb,
		options: {
			showEffects: true,
		},
	});

	if (result.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to approve a message in Sui. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}

	return result.effects.transactionDigest;
}
/**
 * Creates a DWalletCap in the Sui blockchain.
 *
 * This function creates a DWalletCap in the Sui blockchain using the provided DWalletBinder ID
 * and other necessary parameters.
 * The Ack message is created and signed by the authority to bind the DWalletCap to the Sui blockchain.
 * The Ack message is then sent to the Sui blockchain and the signature is verified.
 * The DWalletCap is then bound to the Sui blockchain.
 *
 * @param {CreatedDwallet} dWalletToBind - The created dWallet object.
 * @param {string} activeEncryptionKeysTableID - The ID of the active encryption keys table.
 * @param {string} authorityDWalletId - The ID of the authority dWallet.
 * @param {string} authorityId - The ID of the authority.
 * @param {bigint} chainIdentifier - The chain identifier.
 * @param {string} domainName - The domain name.
 * @param {string} domainVersion - The domain version.
 * @param {string} bindToAuthorityId - The ID of the authority to bind to.
 * @param {boolean} virginBound - A boolean indicating if the DWalletCap is virgin bound.
 * @param {SuiClient} suiClient - The Sui client instance.
 * @param {DWalletClient} dWalletClient - The dWallet client instance.
 * @param {Ed25519Keypair} keypair - The keypair used to sign the transaction.
 * @returns {Promise<{ createDWalletTxDigest: string, suiDWalletCapId: string }>} - A promise that resolves to the transaction digest and the Sui DWalletCap ID.
 */
export async function createSuiDWalletCap(
	dWalletToBind: CreatedDwallet,
	activeEncryptionKeysTableID: string,
	authorityDWalletId: string,
	authorityId: string,
	chainIdentifier: bigint,
	domainName: string,
	domainVersion: string,
	bindToAuthorityId: string,
	virginBound: boolean,
	suiClient: SuiClient,
	dWalletClient: DWalletClient,
	keypair: Ed25519Keypair,
): Promise<{ createDWalletTxDigest: string; suiDWalletCapId: string }> {
	let transactionHash = await createAuthorityAck(
		dWalletToBind.dWalletBinderID,
		true,
		chainIdentifier,
		suiChainType,
		domainName,
		domainVersion,
		keypair,
		dWalletClient,
	);
	assert(transactionHash !== undefined);

	const message: Uint8Array = ethers.getBytes(transactionHash);

	let preSignObjID = await presignWithDWalletID(
		dWalletClient,
		keypair,
		authorityDWalletId,
		message,
		SHA256,
		activeEncryptionKeysTableID,
	);

	let dwalletBinderObj = await getDWalletBinderByID(dWalletToBind.dWalletBinderID, dWalletClient);
	let bindToAuthorityObj = dwalletBinderObj?.bind_to_authority;

	let signatures = await approveAndSignAckWithAuthority(
		authorityId,
		preSignObjID!,
		message,
		authorityDWalletId,
		dWalletToBind.dWalletBinderID,
		dWalletToBind.dwalletCapID,
		bindToAuthorityId,
		bindToAuthorityObj.nonce,
		virginBound,
		SHA256,
		keypair,
		dWalletClient,
	);

	const dwalletObj = await getDwalletByObjID(dWalletClient, authorityDWalletId);
	const publicKey = new Uint8Array(dwalletObj?.publicKey!);

	let recoveryId = recoveryIdSha256(publicKey, message, signatures[0]!);
	let recoverableSignature = new Uint8Array([...signatures[0], recoveryId]);

	return await sendSuiBindingTransaction(
		dWalletToBind.dWalletBinderID,
		dWalletToBind.dwalletCapID,
		message,
		recoverableSignature,
		publicKey,
		keypair,
		dWalletClient,
		suiClient,
	);
}

/**
 * Submits a transaction state proof to the dWallet Network.
 *
 * This function retrieves the transaction block associated with the provided digest
 * from the Sui blockchain, queries additional transaction data, and submits it to
 * the dWallet Network for verification.
 *
 * @param {DWalletClient} dWalletClient - The dWallet client instance.
 * @param {SuiClient} suiClient - The Sui client instance.
 * @param {string} authorityId - The ID of the authority.
 * @param {string} dWalletBinderId - The dWallet binder ID.
 * @param signMessagesId
 * @param {string} suiTxId - The digest of the transaction in the Sui blockchain.
 * @param {CreatedDwallet} dWalletObj - The created dWallet object.
 * @param {string} serviceUrl - The URL of the service to query transaction data.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @throws {Error} - Throws an error if the checkpoint is undefined or null.
 */
export async function submitTxStateProof(
	dWalletClient: DWalletClient,
	suiClient: SuiClient,
	authorityId: string,
	dWalletBinderId: string,
	signMessagesId: string,
	suiTxId: string,
	dWalletObj: CreatedDwallet,
	serviceUrl: string,
	keypair: Keypair,
) {
	let tx = await suiClient.getTransactionBlock({
		digest: suiTxId,
		options: {},
	});

	let seq = tx.checkpoint;

	if (!seq) {
		throw new Error('Checkpoint is undefined or null');
	}

	let { ckp_epoch_id, checkpoint_summary_bytes, checkpoint_contents_bytes, transaction_bytes } =
		await queryTxData(suiTxId, serviceUrl);

	let txb = new TransactionBlock();

	let authoritySharedRef = await getSharedObjectRefById(authorityId, dWalletClient);
	let authorityArg = txb.sharedObjectRef(authoritySharedRef);

	let dWalletBinderSharedRef = await getSharedObjectRefById(dWalletBinderId, dWalletClient, true);
	let dWalletBinderArg = txb.sharedObjectRef(dWalletBinderSharedRef);

	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(dWalletClient, ckp_epoch_id - 1);
	let epochCommitteeObject = await getImmutableOrOwnedObject(dWalletClient, epoch_committee_id);

	let transactionArg = txb.pure(transaction_bytes);
	let dWalletArg = txb.object(dWalletObj?.dwalletID!);
	let committeArg = txb.object(epochCommitteeObject);
	let checkpointArg = txb.pure(checkpoint_summary_bytes);
	let checkpointContentsArg = txb.pure(checkpoint_contents_bytes);

	let [messageApprovalsVec] = txb.moveCall({
		target: `${packageId}::${suiStateProofModuleName}::transaction_state_proof`,
		arguments: [
			dWalletBinderArg,
			authorityArg,
			committeArg,
			checkpointArg,
			checkpointContentsArg,
			transactionArg,
			dWalletArg,
		],
	});

	let messageApprovals = txb.moveCall({
		target: `0x1::vector::pop_back`,
		typeArguments: ['vector<0x3::dwallet::MessageApproval>'],
		arguments: [messageApprovalsVec],
	});

	txb.moveCall({
		target: `0x1::vector::destroy_empty`,
		typeArguments: ['vector<0x3::dwallet::MessageApproval>'],
		arguments: [messageApprovalsVec],
	});

	// txb.moveCall({
	// 	target: `${packageId}::${dWalletModuleName}::sign`,
	// 	typeArguments: [
	// 		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignData`,
	// 		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CreatedSignDataEvent`,
	// 	],
	// 	arguments: [txb.object(signMessagesId), messageApprovals],
	// });
	//
	// const result = await dWalletClient.signAndExecuteTransactionBlock({
	// 	signer: keypair,
	// 	transactionBlock: txb,
	// 	options: {
	// 		showEffects: true,
	// 	},
	// });
	//
	let res = await dWalletClient.devInspectTransactionBlock({
		sender: keypair.toSuiAddress(),
		transactionBlock: txb,
	});
	let messagesAsVecU8 = new Uint8Array(res.results?.at(0)?.returnValues?.at(1)?.at(0)! as number[]);
	let messages = parseUint8Array(messagesAsVecU8);
	return await signAndVerifySignature(
		txb,
		signMessagesId,
		messageApprovals,
		dWalletClient,
		keypair,
		dWalletObj.dwalletID!,
		messages,
		'SHA256',
	);
	// Sign the message approvals so only signing the first vec<vec<u8>> is supported.

	// if (result.effects?.status.status !== 'success') {
	// 	throw new Error('Failed to submit transaction state proof');
	// }

	// let res = await dWalletClient.devInspectTransactionBlock({
	// 	sender: keypair.toSuiAddress(),
	// 	transactionBlock: txb,
	// });
	//
	// Return the `MessageApproval` as bcs bytes
	// return new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[]);
}

// Function to query the Rust service
async function queryTxData(txId: string, url: string): Promise<TxDataResponse> {
	const params = { tx_id: txId };

	try {
		const response = await axios.get(url, { params });
		return response.data;
	} catch (error) {
		console.error('Error querying transaction data:', error);
		throw error;
	}
}

async function getImmutableOrOwnedObject(client: DWalletClient, id: string) {
	const res = await client.getObject({ id });

	if (!res.data) {
		throw new Error('No object found');
	}

	return {
		Object: {
			ImmOrOwned: {
				digest: res.data.digest,
				objectId: id,
				version: res.data.version,
			},
		},
	};
}

async function retrieveEpochCommitteeIdByEpoch(
	client: DWalletClient,
	targetEpoch: number,
): Promise<string> {
	const query: SuiEventFilter = {
		MoveModule: {
			package: '0x0000000000000000000000000000000000000000000000000000000000000003',
			module: 'sui_state_proof',
		},
	};

	let hasNext = true;
	let cursor: EventId | null | undefined = null;

	while (hasNext) {
		const res = await client.queryEvents({ query, cursor });

		if (!res.data || res.data.length === 0) {
			throw new Error('No events returned by the query');
		}

		const filtered = res.data.find((event) => {
			let json = event.parsedJson as object;
			if ('epoch' in json) {
				const epoch = (event.parsedJson as { epoch: number })?.epoch;
				return epoch !== undefined && Number(epoch) === targetEpoch;
			}
			return false;
		});

		if (filtered && (filtered.parsedJson as { epoch_committee_id: string }).epoch_committee_id) {
			return (filtered.parsedJson as { epoch_committee_id: string }).epoch_committee_id;
		}

		cursor = res.nextCursor
			? { eventSeq: res.nextCursor.eventSeq, txDigest: res.nextCursor.txDigest }
			: null;
		hasNext = res.hasNextPage;
	}

	throw new Error('Epoch not found');
}

const sendSuiBindingTransaction = async (
	dWalletBinderId: string,
	virginEthDwalletCapId: string,
	message: Uint8Array,
	signature: Uint8Array,
	publicKey: Uint8Array,
	keypair: Keypair,
	dWalletClient: DWalletClient,
	suiClient: SuiClient,
) => {
	let binderObject = await getDWalletBinderByID(dWalletBinderId, dWalletClient);
	let bindToAuthority = binderObject?.bind_to_authority!;
	let bindToAuthorityId = bindToAuthority.id!.id;
	let bindToAuthorityNonce = bindToAuthority.nonce!;
	let virginBound = binderObject?.virgin_bound!;

	let messageBcs = bcs.vector(bcs.u8()).serialize(message);
	let signatureBcs = bcs.vector(bcs.u8()).serialize(signature);
	let publicKeyBcs = bcs.vector(bcs.u8()).serialize(publicKey);

	let tx = new TransactionBlockSui();
	let [suiDWalletCap] = tx.moveCall({
		target: `${dWalletCapPackageAddressInSui}::${suiDWalletCapModuleName}::bind_dwallet_cap_to_sui`,
		arguments: [
			tx.pure.id(dWalletBinderId),
			tx.pure.id(virginEthDwalletCapId),
			tx.pure.id(bindToAuthorityId),
			tx.pure.u64(bindToAuthorityNonce),
			tx.pure.bool(virginBound),
			tx.pure(messageBcs),
			tx.pure(signatureBcs),
			tx.pure(publicKeyBcs),
		],
		typeArguments: [],
	});
	tx.transferObjects([suiDWalletCap], keypair.toSuiAddress());

	let result = await suiClient.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (result.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to create dWallet in Sui. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}

	const suiDWalletCapId = result.effects?.created?.at(0)?.reference!.objectId!;
	const txDigest = result.digest;
	return { createDWalletTxDigest: txDigest, suiDWalletCapId };
};

function parseUint8Array(input: Uint8Array): Uint8Array[] {
	const result: Uint8Array[] = [];
	let offset = 0;

	// The first element is the number of arrays
	const numberOfArrays = input[offset++];

	for (let i = 0; i < numberOfArrays; i++) {
		// Get the length of the current array
		const length = input[offset++];

		// Extract the sequence of numbers of the specified length
		const array = input.slice(offset, offset + length);
		result.push(array);

		// Move the offset to the end of the current sequence
		offset += length;
	}

	return result;
}
