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
		const serviceUrl = 'http://localhost:6920/gettxdata';

		const dWalletNodeUrl = 'http://127.0.0.1:9000';

		const suiDevnetURL = 'https://fullnode.devnet.sui.io:443';

		const txId = 'DgA1WVxY1qF2e2zAtnicD1RfSdQmmReudniMbm6hP6CP';

		const configObjectId = '0x458097bd140e2d495e36523ee6153eae85656bc09e91397efc9aaac09ef68686';

		const sui_client = new SuiClient({ url: suiDevnetURL });
		const dwallet_client = new SuiClient({ url: dWalletNodeUrl });

		const keyPair = new Ed25519Keypair();

		await requestSuiFromFaucetV0({
			host: 'http://127.0.0.1:9123/gas',
			recipient: keyPair.getPublicKey().toSuiAddress(),
		});

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
			txId,
			serviceUrl,
			keyPair,
		);

		console.log('creation done', resultFinal);

		const bytes: Uint8Array = new Uint8Array([1]);

		const signMessagesIdSHA256 = await createSignMessages(
			dkg?.dwalletId!,
			dkg?.dkgOutput,
			[bytes],
			'SHA256',
			keyPair,
			dwallet_client,
		);

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
				txId,
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
