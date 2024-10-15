// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SuiClient } from '@mysten/sui.js/client';
import axios from 'axios';

import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient, EventId, SuiEventFilter, SuiObjectRef } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import type { SignOutputEventData } from './dwallet.js';

const packageId = '0x3';
const stateProofModuleName = 'sui_state_proof';
const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

type TxDataResponse = {
	ckp_epoch_id: number;
	checkpoint_summary_bytes: Uint8Array;
	checkpoint_contents_bytes: Uint8Array;
	transaction_bytes: Uint8Array;
};

export async function submitDWalletCreationProof(
	dwallet_client: DWalletClient,
	sui_client: SuiClient,
	configObjectId: string,
	registryObjectId: string,
	dWalletCapId: string,
	txId: string,
	serviceUrl: string,
	keypair: Keypair,
) {
	let tx = await sui_client.getTransactionBlock({
		digest: txId,
		options: {},
	});

	let seq = tx.checkpoint;

	if (!seq) {
		throw new Error('Checkpoint is undefined or null');
	}

	let { ckp_epoch_id, checkpoint_summary_bytes, checkpoint_contents_bytes, transaction_bytes } =
		await queryTxData(txId, serviceUrl);
	let txb = new TransactionBlock();

	let dWalletCap = await getOwnedObject(dwallet_client, dWalletCapId);
	let dWalletCapArg = txb.object(dWalletCap);

	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(
		dwallet_client,
		ckp_epoch_id - 1,
		registryObjectId,
	);

	let epochCommitteeObject = await getOwnedObject(dwallet_client, epoch_committee_id);
	let committeeArg = txb.object(epochCommitteeObject);

	let configObject = await getOwnedObject(dwallet_client, configObjectId);
	let configArg = txb.object(configObject);

	let checkpoint_arg = txb.pure(checkpoint_summary_bytes);
	let checkpoint_contents_arg = txb.pure(checkpoint_contents_bytes);
	let transaction_arg = txb.pure(transaction_bytes);

	txb.moveCall({
		target: `${packageId}::${stateProofModuleName}::create_dwallet_wrapper`,
		arguments: [
			configArg,
			dWalletCapArg,
			committeeArg,
			checkpoint_arg,
			checkpoint_contents_arg,
			transaction_arg,
		],
	});
	return dwallet_client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: txb,
		options: {
			showEffects: true,
		},
	});
}

export async function submitTxStateProof(
	dwallet_client: DWalletClient,
	sui_client: SuiClient,
	dWalletId: string,
	configObjectId: string,
	registryObjectId: string,
	capWrapperRef: SuiObjectRef,
	signMessagesId: string,
	txId: string,
	serviceUrl: string,
	keypair: Keypair,
) {
	let tx = await sui_client.getTransactionBlock({
		digest: txId,
		options: {},
	});

	let seq = tx.checkpoint;

	if (!seq) {
		throw new Error('Checkpoint is undefined or null');
	}

	let { ckp_epoch_id, checkpoint_summary_bytes, checkpoint_contents_bytes, transaction_bytes } =
		await queryTxData(txId, serviceUrl);

	let txb = new TransactionBlock();

	let configObject = await getOwnedObject(dwallet_client, configObjectId);
	let configArg = txb.object(configObject);

	let capWrapperArg = txb.object({
		Object: {
			Shared: {
				objectId: capWrapperRef.objectId,
				initialSharedVersion: capWrapperRef.version,
				mutable: true,
			},
		},
	});

	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(
		dwallet_client,
		ckp_epoch_id - 1,
		registryObjectId,
	);
	let epochCommitteeObject = await getOwnedObject(dwallet_client, epoch_committee_id);

	let committeeArg = txb.object(epochCommitteeObject);
	let checkpointArg = txb.pure(checkpoint_summary_bytes);
	let checkpointContentsArg = txb.pure(checkpoint_contents_bytes);
	let transactionArg = txb.pure(transaction_bytes);

	let [messageApprovalsVec] = txb.moveCall({
		target: `${packageId}::${stateProofModuleName}::transaction_state_proof`,
		arguments: [
			configArg,
			capWrapperArg,
			committeeArg,
			checkpointArg,
			checkpointContentsArg,
			transactionArg,
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

	// Sign the message approvals so only signing the first vec<vec<u8>> is supported.
	txb.moveCall({
		target: `${packageId}::${dWalletModuleName}::sign`,
		typeArguments: [
			`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignData`,
			`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CreatedSignDataEvent`,
		],
		arguments: [txb.object(signMessagesId), messageApprovals],
	});

	await dwallet_client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: txb,
		options: {
			showEffects: true,
		},
	});

	return await retrieveSignResult(dwallet_client, dWalletId);
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

async function getOwnedObject(client: DWalletClient, id: string) {
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
async function retrieveSignResult(client: DWalletClient, dWalletId: string): Promise<Uint8Array[]> {
	let reqEventFiltered: any[] = [];
	const queryInterval = 100;

	while (reqEventFiltered.length === 0) {
		const requestedEvents = await client.queryEvents({
			query: {
				MoveEventType: `${packageId}::${dWalletModuleName}::SignOutputEvent`,
			},
			order: 'descending',
		});

		reqEventFiltered = requestedEvents.data.filter((event) => {
			let eventData = event.parsedJson! as SignOutputEventData;
			return eventData.dwallet_id === dWalletId;
		});

		if (reqEventFiltered.length === 0) {
			await new Promise((resolve) => setTimeout(resolve, queryInterval));
		}
	}

	let eventData = reqEventFiltered[0].parsedJson! as SignOutputEventData;
	return eventData.signatures;
}
async function retrieveEpochCommitteeIdByEpoch(
	client: DWalletClient,
	targetEpoch: number,
	targetRegistryId: string,
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
			if ('epoch' in json && 'registry_id' in json) {
				const epoch = (event.parsedJson as { epoch: number })?.epoch;
				const registryId = (event.parsedJson as { registry_id: string })?.registry_id;

				return (
					epoch !== undefined && Number(epoch) === targetEpoch && registryId === targetRegistryId
				);
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
