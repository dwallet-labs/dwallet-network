// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SuiClient } from '@mysten/sui.js/client';
import axios from 'axios';

import { TransactionBlock } from '../builder/index.js';
import { DWalletClient, EventId, SuiEventFilter, SuiObjectRef } from '../client/index.js';
import { Keypair } from '../cryptography/index.js';
import { fetchObjectBySessionId } from './utils.js';

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
		target: `${packageId}::${stateProofModuleName}::create_dwallet_wrapper`,
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
	dwallet_client: DWalletClient,
	sui_client: SuiClient,
	configObjectId: string,
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

	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(dwallet_client, ckp_epoch_id - 1);
	let epochCommitteeObject = await getOwnedObject(dwallet_client, epoch_committee_id);

	let committeArg = txb.object(epochCommitteeObject);
	let checkpointArg = txb.pure(checkpoint_summary_bytes);
	let checkpointContentsArg = txb.pure(checkpoint_contents_bytes);
	let transactionArg = txb.pure(transaction_bytes);

	let [messageApprovals] = txb.moveCall({
		target: `${packageId}::${stateProofModuleName}::transaction_state_proof`,
		arguments: [
			configArg,
			capWrapperArg,
			committeArg,
			checkpointArg,
			checkpointContentsArg,
			transactionArg,
		],
	});

	// sign the message approvals
	txb.moveCall({
		target: `${packageId}::${dWalletModuleName}::sign_messages`,
		typeArguments: [`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignData`],
		arguments: [txb.object(signMessagesId), messageApprovals],
	});
	const result = await dwallet_client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: txb,
		options: {
			showEffects: true,
		},
	});

	const signSessionRef = result.effects?.created?.filter((o) => o.owner == 'Immutable')[0]
		.reference!;

	const signOutput = await fetchObjectBySessionId(
		signSessionRef.objectId,
		`${packageId}::${dWalletModuleName}::SignOutput`,
		keypair,
		dwallet_client,
	);

	if (signOutput?.dataType === 'moveObject') {
		return {
			// @ts-ignore
			signOutputId: signOutput.fields['id']['id'],
			// @ts-ignore
			signatures: signOutput.fields['signatures'],
		};
	}
	return;
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
