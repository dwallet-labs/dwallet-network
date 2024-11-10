// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@dwallet-network/dwallet.js/bcs';
import { DWalletClient, SuiHTTPTransport } from '@dwallet-network/dwallet.js/client';
import { requestSuiFromFaucetV0 as requestDwltFromFaucetV0 } from '@dwallet-network/dwallet.js/faucet';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
import {
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
	getOrCreateEncryptionKey,
	submitDWalletCreationProof,
	submitTxStateProof,
} from '@dwallet-network/dwallet.js/signature-mpc';
import { SuiClient } from '@mysten/sui.js/client';
import { TransactionBlock as TransactionBlockSUI } from '@mysten/sui.js/transactions';

async function main() {
	try {
		const serviceUrl = 'http://localhost:6920/gettxdata'; // For local development
		// const serviceUrl = 'http://sui-testnet-light-client.testnet.dwallet.cloud/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';

		const dWalletCapPackageInSUI =
			'0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec';

		// Objects in the dwallet network.
		// TODO: fix this IDS.
		const configObjectId = '0xd0508ac7ca7ff62e3e03bdf830e5f5bbc8425c7be52bea40738904098ba554f6';
		const registryObjectId = '0xb388dfe5386b44415bff2a2f7c4926c7c76b38246ac78536e23fa0bf61f4d51a';

		const sui_client = new SuiClient({ url: suiTestnetURL });
		const dwallet_client = new DWalletClient({
			transport: new SuiHTTPTransport({
				url: dWalletNodeUrl,
			}),
		});

		const messageSign: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const keyPair = Ed25519Keypair.deriveKeypairFromSeed(
			'witch collapse practice feed shame open despair creek road again ice least',
		);
		// const keyPair2 = Ed25519Keypair.generate();

		const address = keyPair.getPublicKey().toSuiAddress();
		// const address2 = keyPair2.getPublicKey().toSuiAddress();

		console.log('address', address);
		// console.log('address2', address);

		console.log('SUI address', keyPair.toSuiAddress());
		// console.log('SUI address2', keyPair2.toSuiAddress());

		await requestDwltFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		// await requestDwltFromFaucetV0({
		// 	host: 'http://127.0.0.1:9123/gas',
		// 	recipient: keyPair2.getPublicKey().toSuiAddress(),
		// });

		// await requestSuiFromFaucetV0({
		// 	host: 'https://faucet.testnet.sui.io',
		// 	recipient: keyPair.getPublicKey().toSuiAddress(),
		// });

		// sleep for 5 seconds
		await new Promise((resolve) => setTimeout(resolve, 5000));

		console.log('creating dwallet');

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(dwallet_client, keyPair);
		// const encryptionKeysHolder2 = await createActiveEncryptionKeysTable(dwallet_client, keyPair2);

		let activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			keyPair,
			dwallet_client,
			activeEncryptionKeysTableID,
		);

		// let activeEncryptionKeysTableID2 = encryptionKeysHolder2.objectId;
		// let senderEncryptionKeyObj2 = await getOrCreateEncryptionKey(
		// 	keyPair2,
		// 	dwallet_client,
		// 	activeEncryptionKeysTableID2,
		// );

		const createdDwallet1 = await createDWallet(
			keyPair,
			dwallet_client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);
		const createdDwallet2 = await createDWallet(
			keyPair,
			dwallet_client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);

		const createdDwallet3 = await createDWallet(
			keyPair,
			dwallet_client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);

		if (createdDwallet1 == null || createdDwallet2 == null) {
			throw new Error('createDWallet returned null');
		}
		let dwalletCapId1 = createdDwallet1?.dwalletCapID;
		let dWalletId1 = createdDwallet1?.dwalletID;
		// @ts-ignore
		let dwalletCapId2 = createdDwallet2?.dwalletCapID;
		// let dWalletId2 = createdDwallet2?.dwalletID;

		let dwalletCapId3 = createdDwallet3?.dwalletCapID;

		console.log('initialising dwallet cap with ID: ', dwalletCapId1);
		let txb = new TransactionBlockSUI();

		let dWalletCapArg1 = txb.pure(dwalletCapId1);
		let dWalletCapArg2 = txb.pure(dwalletCapId2);
		let dWalletCapArg3 = txb.pure(dwalletCapId3);

		let [cap1] = txb.moveCall({
			target: `${dWalletCapPackageInSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg1],
		});

		let [cap2] = txb.moveCall({
			target: `${dWalletCapPackageInSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg2],
		});
		let [cap3] = txb.moveCall({
			target: `${dWalletCapPackageInSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg3],
		});

		let signMsgArg = txb.pure(bcs.vector(bcs.vector(bcs.u8())).serialize([messageSign]));

		txb.moveCall({
			target: `${dWalletCapPackageInSUI}::dwallet_cap::approve_message`,
			arguments: [cap1, signMsgArg],
		});

		txb.transferObjects([cap1], keyPair.toSuiAddress());
		txb.transferObjects([cap2], keyPair.toSuiAddress());
		txb.transferObjects([cap3], keyPair.toSuiAddress());

		txb.setGasBudget(10000000);

		let res = await sui_client.signAndExecuteTransactionBlock({
			signer: keyPair,
			transactionBlock: txb,
			options: {
				showEffects: true,
			},
		});

		const createCapTxId = res.digest;
		const signTxId = res.digest;
		// const approveMsgTxId = res.digest;

		let first = res.effects?.created?.[0];
		let ref;
		if (first) {
			ref = first.reference.objectId;
			console.log('cap created', ref);
		} else {
			console.log('No objects were created');
		}

		// sleep for 10 seconds
		await new Promise((resolve) => setTimeout(resolve, 10000));

		console.log('address', keyPair.getPublicKey().toSuiAddress());
		console.log('dWalletId1', dwalletCapId1);
		console.log('dWalletId2', dwalletCapId2);
		console.log('dWalletId3', dwalletCapId3);
		let resultFinal1 = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			dwalletCapId1,
			createCapTxId,
			serviceUrl,
			keyPair,
		);
		let resultFinal2 = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			dwalletCapId2,
			createCapTxId,
			serviceUrl,
			keyPair,
		);

		let resultFinal3 = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			// @ts-ignore
			dwalletCapId3,
			createCapTxId,
			serviceUrl,
			keyPair,
		);

		console.log('creation done 1', resultFinal1);
		console.log('creation done 2', resultFinal2);
		console.log('creation done 3', resultFinal3);

		const bytes: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			createdDwallet1?.dwalletID!,
			createdDwallet1?.decentralizedDKGOutput!,
			new Uint8Array(createdDwallet1?.secretKeyShare!),
			[bytes],
			'SHA256',
			keyPair,
			dwallet_client,
		);

		console.log('created signMessages');

		if (signMessagesIdSHA256 == null) {
			throw new Error('createSignMessages returned null');
		}

		if (
			resultFinal1.effects &&
			Array.isArray(resultFinal1.effects.created) &&
			typeof resultFinal1.effects.created[0] === 'object' &&
			'reference' in resultFinal1.effects.created[0]
		) {
			const capWrapperRef = resultFinal1.effects?.created?.[0].reference;

			console.log('A');

			let res = await submitTxStateProof(
				dwallet_client,
				sui_client,
				dWalletId1,
				configObjectId,
				registryObjectId,
				capWrapperRef,
				signMessagesIdSHA256,
				signTxId,
				serviceUrl,
				keyPair,
			);

			console.log('res', res);
			console.log('tx done');
		}
	} catch (error) {
		console.error('Failed to retrieve transaction data:', error);
	}
}

main();
