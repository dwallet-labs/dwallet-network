// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import path from 'path';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import {
	createDKGFirstRoundOutputMock,
	createDWallet,
	dkgSecondRoundMoveCall,
} from '../../src/dwallet-mpc/dkg';
import { getOrCreateClassGroupsKeyPair } from '../../src/dwallet-mpc/encrypt-user-share';
import {
	checkpointCreationTime,
	Config,
	delay,
	getDWalletSecpState,
	mockedProtocolPublicParameters,
} from '../../src/dwallet-mpc/globals';
import { dkgFirstRoundMock } from './mocks';
import { presign } from '../../src/dwallet-mpc/presign';

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
		await createDWallet(conf, mockedProtocolPublicParameters);
	});

	it('should run the DKG second round', async () => {
		await getOrCreateClassGroupsKeyPair(conf);
		await delay(checkpointCreationTime);

		let dwalletState = await getDWalletSecpState(conf);
		let firstRoundOutputResult = await createDKGFirstRoundOutputMock(
			conf,
			Buffer.from(dkgFirstRoundMock.firstRoundOutput, 'base64'),
		);
		await delay(checkpointCreationTime);
		await dkgSecondRoundMoveCall(
			conf,
			dwalletState,
			{
				sessionID: dkgFirstRoundMock.sessionID,
				dwalletCapID: firstRoundOutputResult.dwalletCapID,
				output: Buffer.from(dkgFirstRoundMock.firstRoundOutput, 'base64'),
				dwalletID: firstRoundOutputResult.dwalletID,
			},
			Buffer.from(dkgFirstRoundMock.centralizedPublicKeyShareAndProof, 'base64'),
			Buffer.from(dkgFirstRoundMock.encryptedSecretShareAndProof, 'base64'),
			Buffer.from(dkgFirstRoundMock.centralizedPublicOutput, 'base64'),
		);
		console.log(`dWallet has been created successfully: ${firstRoundOutputResult.dwalletID}`);
	});

	it('should run presign', async () => {
		await getOrCreateClassGroupsKeyPair(conf);
		await delay(checkpointCreationTime);

		let dwalletState = await getDWalletSecpState(conf);
		let firstRoundOutputResult = await createDKGFirstRoundOutputMock(
			conf,
			Buffer.from(dkgFirstRoundMock.firstRoundOutput, 'base64'),
		);
		await delay(checkpointCreationTime);
		await dkgSecondRoundMoveCall(
			conf,
			dwalletState,
			{
				sessionID: dkgFirstRoundMock.sessionID,
				dwalletCapID: firstRoundOutputResult.dwalletCapID,
				output: Buffer.from(dkgFirstRoundMock.firstRoundOutput, 'base64'),
				dwalletID: firstRoundOutputResult.dwalletID,
			},
			Buffer.from(dkgFirstRoundMock.centralizedPublicKeyShareAndProof, 'base64'),
			Buffer.from(dkgFirstRoundMock.encryptedSecretShareAndProof, 'base64'),
			Buffer.from(dkgFirstRoundMock.centralizedPublicOutput, 'base64'),
		);
		console.log(`dWallet has been created successfully: ${firstRoundOutputResult.dwalletID}`);
		let presignCompletion = await presign(conf, firstRoundOutputResult.dwalletID);
		console.log(`presign has been created successfully: ${presignCompletion.presign_id}`);
	});
});
