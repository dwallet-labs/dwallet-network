// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@mysten/bcs';
import type { SuiClient } from '@mysten/sui.js/client';
import { TransactionBlock as TransactionBlockSui } from '@mysten/sui.js/transactions';
import axios from 'axios';

import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient, EventId, SuiEventFilter, SuiObjectRef } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { getSharedObjectRefById } from '../utils/sui-types';
import { fetchObjectBySessionId } from './utils.js';

const packageId = '0x3';
const stateProofModuleName = 'sui_state_proof';
const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

const GAS_BUDGET = 100_000_000;

type TxDataResponse = {
	ckp_epoch_id: number;
	checkpoint_summary_bytes: Uint8Array;
	checkpoint_contents_bytes: Uint8Array;
	transaction_bytes: Uint8Array;
};

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
	let epochCommitteeObject = await getOwnedObject(dWalletClient, epoch_committee_id);

	let committeArg = txb.object(epochCommitteeObject);
	let checkpoint_arg = txb.pure(checkpoint_summary_bytes);
	let checkpoint_contents_arg = txb.pure(checkpoint_contents_bytes);
	let transaction_arg = txb.pure(transaction_bytes);

	txb.moveCall({
		target: `${packageId}::${stateProofModuleName}::verify_dwallet_cap_and_sui_cap_match`,
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

export async function approveMessageInSui(
	suiDWalletCapId: string,
	messageToSign: string,
	dWalletCapPackageAddressInSui: string,
	suiClient: SuiClient,
	keypair: Keypair,
): Promise<string> {
	let txb = new TransactionBlockSui();

	const dWalletCapArg = txb.pure(suiDWalletCapId);
	const messageSign = new TextEncoder().encode(messageToSign);
	const signMsgArg = txb.pure(bcs.vector(bcs.vector(bcs.u8())).serialize([messageSign]));

	txb.moveCall({
		// todo(yuval): dwallet_cap to const
		target: `${dWalletCapPackageAddressInSui}::dwallet_cap::approve_message`,
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

// todo(yuval): document this function
export async function createSuiDWalletCap(
	dWalletCapId: string,
	dWalletCapPackageAddressInSui: string,
	suiClient: SuiClient,
	keypair: Keypair,
) {
	let txb = new TransactionBlockSui();
	let dWalletCapArg1 = txb.pure(dWalletCapId);

	let [suiDWalletCap] = txb.moveCall({
		target: `${dWalletCapPackageAddressInSui}::dwallet_cap::create_cap`,
		arguments: [dWalletCapArg1],
	});
	txb.transferObjects([suiDWalletCap], keypair.toSuiAddress());

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
			'Failed to create dWallet in Sui. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}

	let res = await suiClient.devInspectTransactionBlock({
		sender: keypair.toSuiAddress(),
		transactionBlock: txb,
	});

	// todo(yuval): make sure the cap was created + we grab the right reference
	const suiDWalletCapId = result.effects?.created?.at(0)?.reference!.objectId!;
	const txDigest = result.digest;
	return { createDWalletTxDigest: txDigest, suiDWalletCapId };
}

export async function submitTxStateProof(
	dWalletClient: DWalletClient,
	suiClient: SuiClient,
	authorityId: string,
	dWalletBinderId: string,
	signMessagesId: string,
	suiTxId: string,
	dWalletId: string,
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
	let epochCommitteeObject = await getOwnedObject(dWalletClient, epoch_committee_id);

	let dWalletArg = txb.object(dWalletId);
	let committeArg = txb.object(epochCommitteeObject);
	let checkpointArg = txb.pure(checkpoint_summary_bytes);
	let checkpointContentsArg = txb.pure(checkpoint_contents_bytes);
	let transactionArg = txb.pure(transaction_bytes);

	let [messageApprovals] = txb.moveCall({
		target: `${packageId}::${stateProofModuleName}::transaction_state_proof`,
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

	// sign the message approvals
	// txb.moveCall({
	// 	target: `${packageId}::${dWalletModuleName}::sign_messages`,
	// 	typeArguments: [`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignData`],
	// 	arguments: [txb.object(signMessagesId), messageApprovals],
	// });
	const result = await dWalletClient.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: txb,
		options: {
			showEffects: true,
		},
	});

	// make sure transaction success
	if (result.effects?.status.status !== 'success') {
		throw new Error('Failed to submit transaction state proof');
	}

	let res = await dWalletClient.devInspectTransactionBlock({
		sender: keypair.toSuiAddress(),
		transactionBlock: txb,
	});

	let messageApprovalsBcs = new Uint8Array(
		res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[],
	);
	return messageApprovalsBcs;
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
