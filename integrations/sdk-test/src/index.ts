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
	// Service to get TX data from SUI, a temporary solution.
	lightClientTxDataService: string;
	// The URL of the dWallet node.
	dWalletNodeUrl: string;
	// The dwallet package ID in SUI network where the dWallet cap is defined.
	dWalletCapPackageInSUI: string;
	// The SUI RPC URL (full node).
	suiRPCURL: string;
	// The object ID of the registry in dWallet network.
	dWalletRegistryObjectId: string;
	// The object ID of the config in dWallet network.
	dWalletConfigObjectId: string;
	// The URL of the faucet in dwallet network.
	dWalletFaucetURL: string;
};

function getLocalConf(): NetworkConfig {
	return {
		lightClientTxDataService: 'http://localhost:6920/gettxdata',
		dWalletNodeUrl: 'http://127.0.0.1:9000',
		dWalletFaucetURL: 'http://127.0.0.1:9123/gas',
		dWalletCapPackageInSUI: '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec',
		suiRPCURL: 'https://fullnode.testnet.sui.io:443',
		dWalletRegistryObjectId: '0x4de2a30287ed40600b53c40bfb3eeae7ef4ecf9ba9a90df732c363318612f084',
		dWalletConfigObjectId: '0xcc88a86628098c1472959ba6ad5e1c0fc0c1fd632b7ec21d265fb8efd5d55aea',
	};
}

function getTestNetConf(): NetworkConfig {
	return {
		lightClientTxDataService:
			'https://lightclient-rest-server.alpha.testnet.dwallet.cloud/gettxdata',
		dWalletNodeUrl: 'https://fullnode.alpha.testnet.dwallet.cloud',
		dWalletFaucetURL: 'https://faucet.alpha.testnet.dwallet.cloud/gas',
		dWalletCapPackageInSUI: '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec',
		suiRPCURL: 'https://fullnode.testnet.sui.io:443',
		dWalletRegistryObjectId: '0x4de2a30287ed40600b53c40bfb3eeae7ef4ecf9ba9a90df732c363318612f084',
		dWalletConfigObjectId: '0xcc88a86628098c1472959ba6ad5e1c0fc0c1fd632b7ec21d265fb8efd5d55aea',
	};
}

async function main() {
	try {
		getLocalConf();

		let {
			dWalletConfigObjectId,
			dWalletCapPackageInSUI,
			dWalletNodeUrl,
			lightClientTxDataService,
			dWalletRegistryObjectId,
			suiRPCURL,
			dWalletFaucetURL,
		} = getTestNetConf();

		const suiClient = new SuiClient({ url: suiRPCURL });
		const dwalletClient = new DWalletClient({
			transport: new SuiHTTPTransport({
				url: dWalletNodeUrl,
			}),
		});

		const messageSign: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const keyPair = Ed25519Keypair.deriveKeypairFromSeed(
			'witch collapse practice feed shame open despair creek road again ice least',
		);

		const address = keyPair.getPublicKey().toSuiAddress();
		console.log('Address', address);
		console.log('SUI address', keyPair.toSuiAddress());

		await requestDwltFromFaucetV0({
			host: dWalletFaucetURL,
			recipient: address,
		});

		// Sleep for 5 seconds.
		await new Promise((resolve) => setTimeout(resolve, 5000));

		console.log('Creating dwallet');

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(dwalletClient, keyPair);

		let activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			keyPair,
			dwalletClient,
			activeEncryptionKeysTableID,
		);

		const createdDwallet1 = await createDWallet(
			keyPair,
			dwalletClient,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);
		if (createdDwallet1 == null) {
			throw new Error('createDWallet returned null for createdDwallet1');
		}

		const createdDwallet2 = await createDWallet(
			keyPair,
			dwalletClient,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);
		if (createdDwallet2 == null) {
			throw new Error('createDWallet returned null for createdDwallet2');
		}
		const createdDwallet3 = await createDWallet(
			keyPair,
			dwalletClient,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);
		if (createdDwallet3 == null) {
			throw new Error('createDWallet returned null for createdDwallet3');
		}

		let dwalletCapId1 = createdDwallet1?.dwalletCapID;
		let dwalletCapId2 = createdDwallet2?.dwalletCapID;
		let dwalletCapId3 = createdDwallet3?.dwalletCapID;
		console.log('Initialising dWallet cap with ID: ', dwalletCapId1);
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
		// Approve the message for the given dWallet cap.
		txb.moveCall({
			target: `${dWalletCapPackageInSUI}::dwallet_cap::approve_message`,
			arguments: [cap1, signMsgArg],
		});
		txb.transferObjects([cap1], keyPair.toSuiAddress());
		txb.transferObjects([cap2], keyPair.toSuiAddress());
		txb.transferObjects([cap3], keyPair.toSuiAddress());
		txb.setGasBudget(10000000);
		let res = await suiClient.signAndExecuteTransactionBlock({
			signer: keyPair,
			transactionBlock: txb,
			options: {
				showEffects: true,
			},
		});
		const createCapTxId = res.digest;
		const signTxId = res.digest;

		let first = res.effects?.created?.[0];
		let ref;
		if (first) {
			ref = first.reference.objectId;
			console.log('dwallet cap created in sui:', ref);
		} else {
			console.log('No objects were created');
		}

		await new Promise((resolve) => setTimeout(resolve, 10 * 1000));
		console.log('dwalletCapId1:', dwalletCapId1);
		console.log('dwalletCapId2:', dwalletCapId2);
		console.log('dwalletCapId3:', dwalletCapId3);

		let resultFinal1 = await submitDWalletCreationProof(
			dwalletClient,
			suiClient,
			dWalletConfigObjectId,
			dWalletRegistryObjectId,
			dwalletCapId1,
			createCapTxId,
			lightClientTxDataService,
			keyPair,
		);
		await new Promise((resolve) => setTimeout(resolve, 15000));

		let resultFinal2 = await submitDWalletCreationProof(
			dwalletClient,
			suiClient,
			dWalletConfigObjectId,
			dWalletRegistryObjectId,
			dwalletCapId2,
			createCapTxId,
			lightClientTxDataService,
			keyPair,
		);
		await new Promise((resolve) => setTimeout(resolve, 15000));

		let resultFinal3 = await submitDWalletCreationProof(
			dwalletClient,
			suiClient,
			dWalletConfigObjectId,
			dWalletRegistryObjectId,
			dwalletCapId3,
			createCapTxId,
			lightClientTxDataService,
			keyPair,
		);

		console.log('Creation done 1', resultFinal1);
		console.log('Creation done 2', resultFinal2);
		console.log('Creation done 3', resultFinal3);

		const bytes: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			createdDwallet1?.dwalletID,
			createdDwallet1?.decentralizedDKGOutput,
			new Uint8Array(createdDwallet1?.secretKeyShare),
			[bytes],
			'SHA256',
			keyPair,
			dwalletClient,
		);

		console.log('Created signMessages');

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
				dwalletClient,
				suiClient,
				createdDwallet1?.dwalletID,
				dWalletConfigObjectId,
				dWalletRegistryObjectId,
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
