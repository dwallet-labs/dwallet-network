// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SuiClient } from '@mysten/sui.js/client';
import { requestSuiFromFaucetV0 } from '@mysten/sui.js/faucet';
import { Ed25519Keypair } from '@mysten/sui.js/keypairs/ed25519';
import {
	createDWallet,
	createSignMessages,
	submitDwalletCreationProof,
	submitTxStateProof,
} from '@mysten/sui.js/signature-mpc';

async function main() {
	try {
		const serviceUrl = 'http://sui-devnet-light-client.devnet.dwallet.cloud/gettxdata';
		// const serviceUrl = 'http://localhost:6920/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		const suiDevnetURL = 'https://fullnode.devnet.sui.io:443';

		const createTxId = '3jWvhqFifrSCKTvabWVrWG3RmLhUiJWCe2XQKeS6kJqt'; // of the dwallet cap id on sui devnet
		const signTxId = 'BGR7L5NC1DeGvWatYXkEGfQ2o1T8DgLRBW51UFUwFmdv'; // of the dwallet cap id on sui devnet

		const configObjectId = '0x19255b096e19727b63421dbd2fbc26f1c1a5f28017c0726a2ca479b8a9a09d6d'; // should take this from the light_client.yaml

		const sui_client = new SuiClient({ url: suiDevnetURL });
		const dwallet_client = new SuiClient({ url: dWalletNodeUrl });

		const keyPair = new Ed25519Keypair();

		await requestSuiFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		console.log('creating dwallet');
		const dkg = await createDWallet(keyPair, dwallet_client);

		if (dkg == null) {
			throw new Error('createDWallet returned null');
		}
		let { dwalletCapId } = dkg;

		console.log('address', keyPair.getPublicKey().toSuiAddress());

		let resultFinal = await submitDwalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			dwalletCapId,
			createTxId,
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
