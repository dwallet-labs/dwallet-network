// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import axios from 'axios';

import { TransactionBlock } from '../builder';
import { EventId, SuiClient, SuiEventFilter, SuiObjectRef } from '../client';
import { Keypair } from '../cryptography';

type TxDataResponse = {
	ckp_epoch_id: number;
	checkpoint_summary_bytes: Uint8Array;
	checkpoint_contents_bytes: Uint8Array;
	transaction_bytes: Uint8Array;
};

export async function submitDwalletCreationProof(
	dwallet_client: SuiClient,
	sui_client: SuiClient,
	configObjectId: string,
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

	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(dwallet_client, ckp_epoch_id - 1);
	let epochCommitteeObject = await getOwnedObject(dwallet_client, epoch_committee_id);
	let committeArg = txb.object(epochCommitteeObject);

	let configObject = await getOwnedObject(dwallet_client, configObjectId);
	let configArg = txb.object(configObject);

	let checkpoint_arg = txb.pure(checkpoint_summary_bytes);
	let checkpoint_contents_arg = txb.pure(checkpoint_contents_bytes);
	let transaction_arg = txb.pure(transaction_bytes);

	txb.moveCall({
		target:
			'0x0000000000000000000000000000000000000000000000000000000000000003::sui_state_proof::create_dwallet_wrapper',
		arguments: [
			configArg,
			dWalletCapArg,
			committeArg,
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
	dwallet_client: SuiClient,
	sui_client: SuiClient,
	configObjectId: string,
	capWrapperId: SuiObjectRef,
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
				objectId: capWrapperId.objectId,
				initialSharedVersion: capWrapperId.version,
				mutable: true,
			},
		},
	});

	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(dwallet_client, ckp_epoch_id - 1);
	let epochCommitteeObject = await getOwnedObject(dwallet_client, epoch_committee_id);

	let committeArg = txb.object(epochCommitteeObject);
	let checkpointArg = txb.pure(checkpoint_summary_bytes);
	let checkpointContentsArg = txb.pure(checkpoint_contents_bytes);
	let transactionArg = txb.pure(transaction_bytes);

	let [approvals] = txb.moveCall({
		target:
			'0x0000000000000000000000000000000000000000000000000000000000000003::sui_state_proof::transaction_state_proof',
		arguments: [
			configArg,
			capWrapperArg,
			committeArg,
			checkpointArg,
			checkpointContentsArg,
			transactionArg,
		],
	});

	txb.moveCall({
		target:
			'0x0000000000000000000000000000000000000000000000000000000000000003::dwallet::create_approvals_holder',
		arguments: [approvals],
	});

	return dwallet_client.signAndExecuteTransactionBlock({ signer: keypair, transactionBlock: txb });
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

async function getOwnedObject(client: SuiClient, id: string) {
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
	client: SuiClient,
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
