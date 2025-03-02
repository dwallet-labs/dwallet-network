// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import path from 'path';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import { createDWallet, mockCreateDWallet } from '../../src/dwallet-mpc/dkg';
import { Config, delay, mockedProtocolPublicParameters } from '../../src/dwallet-mpc/globals';
import { presign } from '../../src/dwallet-mpc/presign';
import { dkgMocks } from './mocks';

const fiveMinutes = 5 * 60 * 1000;
describe('Test dWallet MPC', () => {
	let conf: Config;

	beforeEach(async () => {
		const keypair = Ed25519Keypair.deriveKeypairFromSeed('0x1');
		const dWalletSeed = new Uint8Array(32).fill(8);
		const encryptedSecretShareSigningKeypair = Ed25519Keypair.deriveKeypairFromSeed(
			Buffer.from(dWalletSeed).toString('hex'),
		);
		const address = keypair.getPublicKey().toSuiAddress();
		const suiClient = new SuiClient({ url: getFullnodeUrl('localnet') });
		await requestSuiFromFaucetV1({
			host: getFaucetHost('localnet'),
			recipient: address,
		});

		conf = {
			suiClientKeypair: keypair,
			client: suiClient,
			timeout: fiveMinutes,
			ikaConfig: require(path.resolve(process.cwd(), '../../ika_config.json')),
			dWalletSeed,
			encryptedSecretShareSigningKeypair,
		};
		await delay(2000);
	});

	it('should create a dWallet (DKG)', async () => {
		const dwalletID = await createDWallet(conf, mockedProtocolPublicParameters);
		console.log(`dWallet has been created successfully: ${dwalletID}`);
	});

	it('should mock create dwallet', async () => {
		await mockCreateDWallet(conf, Buffer.from(dkgMocks.dwalletOutput, 'base64'));
	});

	it('should run presign', async () => {
		const dwalletID = await createDWallet(conf, mockedProtocolPublicParameters);
		console.log(`dWallet has been created successfully: ${dwalletID}`);
		const presignCompletion = await presign(conf, dwalletID);
		console.log(`presign has been created successfully: ${presignCompletion.presign_id}`);
	});
});
