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
// import { requestSuiFromFaucetV0 } from '@mysten/sui.js/faucet';
import { TransactionBlock as TransactionBlockSUI } from '@mysten/sui.js/transactions';

async function main() {
	try {
		const serviceUrl = 'http://localhost:6920/gettxdata'; // For local development
		// const serviceUrl = 'http://sui-testnet-light-client.testnet.dwallet.cloud/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';

		const configObjectId = '0x5cad71d9b6289b111d476ab86725c52c1a92adb81cf2c09cfc456f180109f4a8'; // should take this from the light_client.yaml
		const registryObjectId = '0x0be2ba6cd77cfaa4c38d3a1f9f06d8c4bb347af414334b874ef2d6353dd67196';

		const sui_client = new SuiClient({ url: suiTestnetURL });
		const dwallet_client = new DWalletClient({
			transport: new SuiHTTPTransport({
				url: dWalletNodeUrl,

				// websocket: {
				// 	reconnectTimeout: 1000,
				// 	url: dWalletNodeUrl + '/websockets',
				// },
			}),
		});

		// const messageSign = 'dWallets are coming... to Sui';
		const messageSign: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const keyPair = Ed25519Keypair.deriveKeypairFromSeed(
			'witch collapse practice feed shame open despair creek road again ice least',
		);
		const address = keyPair.getPublicKey().toSuiAddress();

		console.log('address', address);

		console.log('SUI address', keyPair.toSuiAddress());

		const dWalletCapPackageSUI =
			'0x0bde775d63aa25d1fb0df56b71acc516d746ea8d79a89a6dc7c9039e0bd41db6';

		await requestDwltFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		// await requestSuiFromFaucetV0({
		// 	host: 'https://faucet.testnet.sui.io',
		// 	recipient: keyPair.getPublicKey().toSuiAddress(),
		// });

		// sleep for 5 seconds
		await new Promise((resolve) => setTimeout(resolve, 5000));

		console.log('creating dwallet');

		let activeEncryptionKeysTableID: string;

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(dwallet_client, keyPair);

		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			keyPair,
			dwallet_client,
			activeEncryptionKeysTableID,
		);

		const createdDwallet = await createDWallet(
			keyPair,
			dwallet_client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);

		if (createdDwallet == null) {
			throw new Error('createDWallet returned null');
		}
		let dwalletCapId = createdDwallet?.dwalletCapID;
		let dWalletId = createdDwallet?.dwalletID;

		console.log('initialising dwallet cap with id: ', dwalletCapId);
		let txb = new TransactionBlockSUI();

		let dWalletCapArg = txb.pure(dwalletCapId);

		let [cap] = txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg],
		});

		let signMsgArg = txb.pure(bcs.vector(bcs.vector(bcs.u8())).serialize([messageSign]));

		txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::approve_message`,
			arguments: [cap, signMsgArg],
		});

		txb.transferObjects([cap], keyPair.toSuiAddress());

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

		let resultFinal = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			dwalletCapId,
			createCapTxId,
			serviceUrl,
			keyPair,
		);

		console.log('creation done', resultFinal);

		const bytes: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			createdDwallet?.dwalletID!,
			createdDwallet?.decentralizedDKGOutput!,
			new Uint8Array(createdDwallet?.secretKeyShare!),
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
			typeof resultFinal.effects?.created == 'object' &&
			'reference' in resultFinal.effects?.created?.[0]
		) {
			const capWrapperRef = resultFinal.effects?.created?.[0].reference;

			console.log('A');

			let res = await submitTxStateProof(
				dwallet_client,
				sui_client,
				dWalletId,
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
