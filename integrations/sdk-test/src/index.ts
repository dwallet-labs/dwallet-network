// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import {
	createDWallet,
	createSignMessages,
	submitDWalletCreationProof,
	submitTxStateProof,
} from '@dwallet-network/dwallet.js//signature-mpc';
import { SuiClient as dWalletClient } from '@dwallet-network/dwallet.js/client';
import { requestSuiFromFaucetV0 as requestDwltFromFaucetV0 } from '@dwallet-network/dwallet.js/faucet';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
import { SuiClient } from '@mysten/sui.js/client';
import { requestSuiFromFaucetV0 } from '@mysten/sui.js/faucet';
import { TransactionBlock } from '@mysten/sui.js/transactions';

async function getOwnedObject(client: dWalletClient, id: string) {
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

async function main() {
	try {
		// const serviceUrl = 'http://sui-devnet-light-client.devnet.dwallet.cloud/gettxdata';
		const serviceUrl = 'http://localhost:6920/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		// const suiDevnetURL = 'https://fullnode.devnet.sui.io:443';
		const suiTestnetURL = 'http://usw1a-tnt-rpc-0-3a5838e.testnet.sui.io:9000';

		const signTxId = 'BGR7L5NC1DeGvWatYXkEGfQ2o1T8DgLRBW51UFUwFmdv'; // of the dwallet cap id on sui devnet

		const configObjectId = '0x39ddff2aec69fc36d6748581b7afc132c84075fc78919765624d5c86e553b8b4'; // should take this from the light_client.yaml

		const sui_client = new SuiClient({ url: suiTestnetURL });
		const dwallet_client = new dWalletClient({ url: dWalletNodeUrl });

		const keyPair = new Ed25519Keypair();

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

		console.log('creating dwallet');
		const dkg = await createDWallet(keyPair, dwallet_client);

		if (dkg == null) {
			throw new Error('createDWallet returned null');
		}
		let { dwalletCapId } = dkg;

		console.log('initialising dwallet cap');
		let txb = new TransactionBlock();
		let dWalletCap = await getOwnedObject(dwallet_client, dwalletCapId);
		let dWalletCapArg = txb.object(dWalletCap);

		txb.moveCall({
			target: `${dWalletCapPackageSUI}::dwallet_cap::create_cap`,
			arguments: [dWalletCapArg],
		});

		let res = await sui_client.signAndExecuteTransactionBlock({
			signer: keyPair,
			transactionBlock: txb,
			options: {
				showEffects: true,
			},
		});

		const createCapTxId = res.digest;

		console.log('cap created', res);

		// sleep for 5 seconds
		await new Promise((resolve) => setTimeout(resolve, 5000));

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

		const bytes: Uint8Array = new TextEncoder().encode('dWallets are coming... to Sui');

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

			console.log('A');

			let res = await submitTxStateProof(
				dwallet_client,
				sui_client,
				configObjectId,
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
