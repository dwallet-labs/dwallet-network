// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { DWalletClient } from '@dwallet-network/dwallet.js/client';
import { requestSuiFromFaucetV0 as requestDwltFromFaucetV0 } from '@dwallet-network/dwallet.js/faucet';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
import {
	createDWallet,
	createSignMessages,
	submitDWalletCreationProof,
	submitTxStateProof,
} from '@dwallet-network/dwallet.js/signature-mpc';
import { SuiClient } from '@mysten/sui.js/client';
import { requestSuiFromFaucetV0 } from '@mysten/sui.js/faucet';
import { TransactionBlock as TransactionBlockSUI } from '@mysten/sui.js/transactions';

async function main() {
	try {
		// const serviceUrl = 'http://sui-devnet-light-client.devnet.dwallet.cloud/gettxdata';
		const serviceUrl = 'http://localhost:6920/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		// const suiDevnetURL = 'https://fullnode.devnet.sui.io:443';
		const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';

		const configObjectId = '0xd3fc444d4d546eb6f1617294a1b4fc814a7f868558b1cb86954a1a7e13d7b92e'; // should take this from the light_client.yaml

		const sui_client = new SuiClient({ url: suiTestnetURL });
		const dwallet_client = new DWalletClient({ url: dWalletNodeUrl });

		const messageSign = 'dWallets are coming... to Sui';

		const keyPair = Ed25519Keypair.deriveKeypairFromSeed(
			'witch collapse practice feed shame open despair creek road again ice least',
		);
		console.log('SUI address', keyPair.toSuiAddress());

		const dWalletCapPackageSUI =
			'0xda072e51bf74040f2f99909595ef1db40fdc75071b92438bb9864f6c744c6736';

		await requestDwltFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		await requestSuiFromFaucetV0({
			host: 'https://faucet.testnet.sui.io',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		// sleep for 5 seconds
		await new Promise((resolve) => setTimeout(resolve, 5000));

		console.log('creating dwallet');
		const dkg = await createDWallet(keyPair, dwallet_client);

		if (dkg == null) {
			throw new Error('createDWallet returned null');
		}
		let { dwalletCapId } = dkg;

		console.log('initialising dwallet cap with id: ', dwalletCapId);
		let txb = new TransactionBlockSUI();

		let dWalletCapArg = txb.pure(dwalletCapId);

		let [cap] = txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg],
		});

		let signMsgArg = txb.pure(messageSign);
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
		const approveMsgTxId = res.digest;

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
			dwalletCapId,
			createCapTxId,
			serviceUrl,
			keyPair,
		);

		console.log('creation done', resultFinal);

		const bytes: Uint8Array = new TextEncoder().encode(messageSign);

		const signMessagesIdSHA256 = await createSignMessages(
			dkg?.dwalletId!,
			dkg?.dkgOutput,
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
			let res = await submitTxStateProof(
				dwallet_client,
				sui_client,
				configObjectId,
				capWrapperRef,
				signMessagesIdSHA256,
				approveMsgTxId,
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
