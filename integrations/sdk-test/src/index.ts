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

type NetworkConfig = {
	lightClientTxDataService: string;
	dWalletNodeUrl: string;
	dWalletCapPackageInSUI: string;
	suiRPCURL: string;
	registryObjectId: string;
	configObjectId: string;
	faucetURL: string;
};

function getLocalConf(): NetworkConfig {
	return {
		lightClientTxDataService: 'http://localhost:6920/gettxdata',
		dWalletNodeUrl: 'http://127.0.0.1:9000',
		faucetURL: 'http://127.0.0.1:9123/gas',
		dWalletCapPackageInSUI: '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec',
		suiRPCURL: 'https://fullnode.testnet.sui.io:443',
		registryObjectId: '0x4de2a30287ed40600b53c40bfb3eeae7ef4ecf9ba9a90df732c363318612f084',
		configObjectId: '0xcc88a86628098c1472959ba6ad5e1c0fc0c1fd632b7ec21d265fb8efd5d55aea',
	};
}

function getTestNetConf(): NetworkConfig {
	return {
		lightClientTxDataService:
			'https://lightclient-rest-server.alpha.testnet.dwallet.cloud/gettxdata',
		dWalletNodeUrl: 'https://fullnode.alpha.testnet.dwallet.cloud',
		faucetURL: 'https://faucet.alpha.testnet.dwallet.cloud/gas',
		dWalletCapPackageInSUI: '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec',
		suiRPCURL: 'https://fullnode.testnet.sui.io:443',
		registryObjectId: '0x4de2a30287ed40600b53c40bfb3eeae7ef4ecf9ba9a90df732c363318612f084',
		configObjectId: '0xcc88a86628098c1472959ba6ad5e1c0fc0c1fd632b7ec21d265fb8efd5d55aea',
	};
}

async function main() {
	try {
		getLocalConf();

		let {
			configObjectId,
			dWalletCapPackageInSUI,
			dWalletNodeUrl,
			lightClientTxDataService,
			registryObjectId,
			suiRPCURL,
		} = getTestNetConf();

		const sui_client = new SuiClient({ url: suiRPCURL });
		const dwallet_client = new DWalletClient({
			transport: new SuiHTTPTransport({
				url: dWalletNodeUrl,
			}),
		});

		const messageSign: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const keyPair = Ed25519Keypair.deriveKeypairFromSeed(
			'witch collapse practice feed shame open despair creek road again ice least',
		);

		const address = keyPair.getPublicKey().toSuiAddress();

		console.log('address', address);
		console.log('SUI address', keyPair.toSuiAddress());

		await requestDwltFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		// Sleep for 5 seconds.
		await new Promise((resolve) => setTimeout(resolve, 5000));

		console.log('creating dwallet');

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(dwallet_client, keyPair);

		let activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			keyPair,
			dwallet_client,
			activeEncryptionKeysTableID,
		);

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
			lightClientTxDataService,
			keyPair,
		);
		let resultFinal2 = await submitDWalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			registryObjectId,
			dwalletCapId2,
			createCapTxId,
			lightClientTxDataService,
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
			lightClientTxDataService,
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
				lightClientTxDataService,
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
