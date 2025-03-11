// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import path from 'path';
import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import { createDWallet, mockCreateDWallet } from '../../src/dwallet-mpc/dkg';
import {
	checkpointCreationTime,
	Config,
	delay,
	mockedProtocolPublicParameters,
	MPCKeyScheme,
} from '../../src/dwallet-mpc/globals';
import { mockCreatePresign, presign } from '../../src/dwallet-mpc/presign';
import { sign } from '../../src/dwallet-mpc/sign';
import { dkgMocks, mockPresign } from './mocks';

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
		const dwallet = await createDWallet(conf, mockedProtocolPublicParameters);
		console.log(`dWallet has been created successfully: ${dwallet.dwallet_id}`);
	});

	it('should mock create dwallet', async () => {
		const result = await mockCreateDWallet(conf, Buffer.from(dkgMocks.dwalletOutput, 'base64'));
		console.log(`dWallet has been created successfully: ${result.dwalletID}`);
	});

	it('should run presign', async () => {
		const dwalletID = (await mockCreateDWallet(conf, Buffer.from(dkgMocks.dwalletOutput, 'base64')))
			.dwalletID;
		console.log(`dWallet has been created successfully: ${dwalletID}`);
		const presignCompletion = await presign(conf, dwalletID);
		console.log(`presign has been created successfully: ${presignCompletion.presign_id}`);
	});

	it('should mock create presign', async () => {
		const dwalletID = (await mockCreateDWallet(conf, Buffer.from(dkgMocks.dwalletOutput, 'base64')))
			.dwalletID;
		const presign = await mockCreatePresign(
			conf,
			Buffer.from(mockPresign.presignBytes, 'base64'),
			dwalletID,
		);
		console.log(`presign has been created successfully: ${presign}`);
	});

	it('should sign', async () => {
		const dkgResult = await mockCreateDWallet(conf, Buffer.from(dkgMocks.dwalletOutput, 'base64'));
		const presign = await mockCreatePresign(
			conf,
			Buffer.from(mockPresign.presignBytes, 'base64'),
			dkgResult.dwalletID,
		);
		await delay(checkpointCreationTime);
		await sign(
			conf,
			presign.presign_id,
			dkgResult.dwalletCapID,
			Buffer.from('hello world'),
			Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
		);
	});

	it('should sign full flow', async () => {
		const dwalletID = await createDWallet(conf, mockedProtocolPublicParameters);
		console.log(`dWallet has been created successfully: ${dwalletID}`);
		await delay(checkpointCreationTime);
		const presignCompletion = await presign(conf, dwalletID.dwallet_id);
		console.log(`presign has been created successfully: ${presignCompletion.presign_id}`);
		await delay(checkpointCreationTime);
		await sign(
			conf,
			presignCompletion.presign_id,
			dwalletID.dwallet_cap_id,
			Buffer.from('hello world'),
			dwalletID.secret_share,
		);
	});
});

describe('Test dWallet MPC - offline', () => {
	it('should run sign centralized part', () => {
		const centralizedSignedMessage = create_sign_centralized_output(
			mockedProtocolPublicParameters,
			MPCKeyScheme.Secp256k1,
			Buffer.from(dkgMocks.dwalletOutput, 'base64'),
			Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
			Buffer.from(mockPresign.presignBytes, 'base64'),
			Buffer.from('hello world'),
			1,
		);
		console.log(
			`centralizedSignedMessage: ${Buffer.from(centralizedSignedMessage).toString('base64')}`,
		);
	});
});
