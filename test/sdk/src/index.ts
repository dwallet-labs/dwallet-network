// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SuiClient } from '@mysten/sui.js/client';
import { requestSuiFromFaucetV0 } from '@mysten/sui.js/faucet';
import { Ed25519Keypair } from '@mysten/sui.js/keypairs/ed25519';
import { initDwallet } from '@mysten/sui.js/signature-mpc';

async function main() {
	try {
		console.log('Hello, World!');

		const serviceUrl = 'http://localhost:6920/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		const suiDevnetURL = 'https://fullnode.devnet.sui.io:443';

		const txId = 'DgA1WVxY1qF2e2zAtnicD1RfSdQmmReudniMbm6hP6CP';

		const configObjectId = '0xd2a75dee88327cf9147db1ea07725700ecee38878b2497f43085256e88891657'; // Replace with actual value

		// const client = new SuiClient({ url: dWalletNodeUrl });

		// const data = await queryTxData(tid, serviceUrl);
		const sui_client = new SuiClient({ url: suiDevnetURL });
		const dwallet_client = new SuiClient({ url: dWalletNodeUrl });

		// TOOD dwallet cap id of dwallet network
		const dWalletCapId = '0xe629a667799299a9a3c5353b946748f070520af89f086daece4ef37b2c64ad63'; // Replace with actual value

		const keyPair = new Ed25519Keypair();

		await requestSuiFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		let x = await dwallet_client.getOwnedObjects({ owner: keyPair.getPublicKey().toSuiAddress() });

		console.log('owned objects', x);

		console.log('address', keyPair.getPublicKey().toSuiAddress());

		await initDwallet(
			dwallet_client,
			sui_client,
			configObjectId,
			dWalletCapId,
			txId,
			serviceUrl,
			keyPair,
		);
		// Additional processing can be done here if necessary
	} catch (error) {
		console.error('Failed to retrieve transaction data:', error);
	}
}

main();

// import

// type TxDataRequest = {
// 	tx_id: String;
// };

// type TxDataResponse = {
// 	ckp_epoch_id: number;
// 	checkpoint_summary_bytes: Uint8Array;
// 	checkpoint_contents_bytes: Uint8Array;
// 	transaction_bytes: Uint8Array;
// };

// // Function to query the Rust service
// async function queryTxData(txId: string, url: string): Promise<TxDataResponse> {
// 	const params = { tx_id: txId }; // Ensure the parameter name matches what the server expects

// 	try {
// 		const response = await axios.get(url, { params });
// 		return response.data;
// 	} catch (error) {
// 		console.error('Error querying transaction data:', error);
// 		throw error;
// 	}
// }

// export async function getSharedObjectVersion(client: SuiClient, id: string): Promise<string> {
// 	const res = await client.getObject({ id, options: { showOwner: true } });
// 	if (!res.data?.owner || typeof res.data?.owner !== 'object' || !('Shared' in res.data?.owner)) {
// 		throw new Error('No object found');
// 	}

// 	const version = res.data.owner.Shared.initial_shared_version;

// 	return version;
// }

// export async function getOwnedObject(client: SuiClient, id: string) {
// 	const res = await client.getObject({ id });

// 	if (!res.data) {
// 		throw new Error('No object found');
// 	}

// 	return {
// 		Object: {
// 			ImmOrOwned: {
// 				digest: res.data.digest,
// 				objectId: id,
// 				version: res.data.version,
// 			},
// 		},
// 	};
// }

// // import { SuiClient, ObjectID, EventFilter, Identifier, EventId } from '@mysten/sui.js';

// interface Config {
// 	dwalletFullNodeUrl: string;
// }

// async function retrieveEpochCommitteeIdByEpoch(
// 	client: SuiClient,
// 	targetEpoch: number,
// ): Promise<string> {
// 	const query: SuiEventFilter = {
// 		MoveModule: {
// 			package: '0x0000000000000000000000000000000000000000000000000000000000000003',
// 			module: 'sui_state_proof',
// 		},
// 	};

// 	let hasNext = true;
// 	let cursor: EventId | null | undefined = null;

// 	while (hasNext) {
// 		const res = await client.queryEvents({ query, cursor });

// 		const filtered = res.data.find((event) => {
// 			let json = event.parsedJson as object;
// 			if ('epoch' in json) {
// 				const epoch = (event.parsedJson as { epoch: number })?.epoch;
// 				return epoch !== undefined && Number(epoch) === targetEpoch;
// 			}
// 		});

// 		if (filtered) {
// 			const epochCommitteeId = (filtered.parsedJson as { epoch_committee_id: string })
// 				?.epoch_committee_id;
// 			if (epochCommitteeId) {
// 				return epochCommitteeId;
// 			}
// 		}
// 		cursor = res.nextCursor
// 			? { eventSeq: res.nextCursor.eventSeq, txDigest: res.nextCursor.txDigest }
// 			: null;
// 		hasNext = res.hasNextPage;
// 	}

// 	throw new Error('Epoch not found');
// }

// // pass the dwallet cap as an argumen
// async function initDwallet(
// 	dwallet_client: SuiClient,
// 	sui_client: SuiClient,
// 	configObjectId: string,
// 	dWalletCapId: string,
// 	txId: string,
// 	serviceUrl: string,
// 	keypair: Keypair,
// ) {
// 	console.log('retrieving checkpoint');
// 	let tx = await sui_client.getTransactionBlock({
// 		digest: txId,
// 		options: {
// 			// showBalanceChanges: true,
// 			// showEffects: true,
// 			// showEvents: true,
// 			// showInput: true,
// 			// showObjectChanges: true,
// 			// showRawInput: true,
// 		},
// 	});
// 	let seq = tx.checkpoint;

// 	console.log('checkpoint', seq);
// 	if (!seq) {
// 		throw new Error('Checkpoint is undefined or null');
// 	}

// 	console.log('1');
// 	let { ckp_epoch_id, checkpoint_summary_bytes, checkpoint_contents_bytes, transaction_bytes } =
// 		await queryTxData(txId, serviceUrl);

// 	let txb = new TransactionBlock();

// 	// let dWalletCap = txb.receivingRef(cap);

// 	// TODO move this out
// 	let dWalletCap = txb.moveCall({
// 		target:
// 			'0x0000000000000000000000000000000000000000000000000000000000000003::sui_state_proof::create_dwallet_wrapper',
// 	});

// 	console.log('retrieving committee id', ckp_epoch_id);
// 	let epoch_committee_id = await retrieveEpochCommitteeIdByEpoch(dwallet_client, ckp_epoch_id);

// 	console.log('epoch_committee_id', epoch_committee_id);
// 	let epochCommitteeObject = await getOwnedObject(dwallet_client, epoch_committee_id);

// 	console.log('done');
// 	let committeArg = txb.object(epochCommitteeObject);

// 	let configObject = await getOwnedObject(dwallet_client, configObjectId);
// 	let configArg = txb.object(configObject);

// 	// let capArg = txb.object(await getOwnedObject(dwallet_client, dWalletCapId));

// 	console.log('type', typeof checkpoint_summary_bytes);
// 	let checkpoint_arg = txb.pure(checkpoint_summary_bytes);
// 	let checkpoint_contents_arg = txb.pure(checkpoint_contents_bytes);
// 	let transaction_arg = txb.pure(transaction_bytes);

// 	// let checkpoint_arg = txb.pure(Uint8Array.prototype);
// 	// let checkpoint_contents_arg = txb.pure(Uint8Array.prototype);
// 	// let transaction_arg = txb.pure(Uint8Array.prototype);

// 	let res = txb.moveCall({
// 		target:
// 			'0x0000000000000000000000000000000000000000000000000000000000000003::sui_state_proof::create_dwallet_wrapper',
// 		arguments: [
// 			configArg,
// 			dWalletCap,
// 			committeArg,
// 			checkpoint_arg,
// 			checkpoint_contents_arg,
// 			transaction_arg,
// 		],
// 		// let comittee = retrieveEpochCommitteeIdByEpoch({ dwalletFullNodeUrl: dWalletNodeUrl }, 1);
// 	});
// 	console.log('signing and executing');
// 	dwallet_client.signAndExecuteTransactionBlock({ signer: keypair, transactionBlock: txb });
// }

// // create dwallet cap
// // user would

// import { bcs } from '@mysten/sui.js/bcs';
// // import { ObjectCallArg } from '@mysten/sui.js/builder';
// // import { EventId, getFullnodeUrl, SuiClient, SuiEventFilter } from '@mysten/sui.js/client';
// import { Keypair } from '@mysten/sui.js/dist/cjs/cryptography';
// // import { getFaucetHost, requestSuiFromFaucetV0 } from '@mysten/sui.js/faucet';
// // import { Ed25519Keypair } from '@mysten/sui.js/keypairs/ed25519';
// // import {ObjectId} from '@mysten/sui.js/builder';
// import { TransactionBlock } from '@mysten/sui.js/transactions';
// import axios from 'axios';

// import { SuiClient } from '../../../sdk/typescript/client/';
// import { requestSuiFromFaucetV0 } from '../../../sdk/typescript/faucet/';
// import { Ed25519Keypair } from '../../../sdk/typescript/keypairs/ed25519';
// import { initDwallet } from '../../../sdk/typescript/signature-mpc/';
