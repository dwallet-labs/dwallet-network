// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SuiClient } from '@mysten/sui.js/client';
import { requestSuiFromFaucetV0 } from '@mysten/sui.js/faucet';
import { Ed25519Keypair } from '@mysten/sui.js/keypairs/ed25519';
import {
	createDWallet,
	submitDwalletCreationProof,
	submitTxStateProof,
} from '@mysten/sui.js/signature-mpc';

async function main() {
	try {
		const serviceUrl = 'http://localhost:6920/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		const suiDevnetURL = 'https://fullnode.devnet.sui.io:443';

		const txId = 'DgA1WVxY1qF2e2zAtnicD1RfSdQmmReudniMbm6hP6CP';

		const configObjectId = '0xcdd8c5ebc06a405b4ee5898998141f86b41cabe0fef3841882c70e9f8a9dee9d';

		const sui_client = new SuiClient({ url: suiDevnetURL });
		const dwallet_client = new SuiClient({ url: dWalletNodeUrl });

		const keyPair = new Ed25519Keypair();

		await requestSuiFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

		let result = await createDWallet(keyPair, dwallet_client);

		if (result == null) {
			throw new Error('createDWallet returned null');
		}
		let { dwalletCapId } = result;

		console.log('address', keyPair.getPublicKey().toSuiAddress());

		let resultFinal = await submitDwalletCreationProof(
			dwallet_client,
			sui_client,
			configObjectId,
			dwalletCapId,
			txId,
			serviceUrl,
			keyPair,
		);

		console.log('resultFinal');
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
				txId,
				serviceUrl,
				keyPair,
			);

			console.log('res', res);
			console.log('tx done');
		}

		// Additional processing can be done here if necessary
	} catch (error) {
		console.error('Failed to retrieve transaction data:', error);
	}
}

main();
