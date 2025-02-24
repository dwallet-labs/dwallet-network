// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import {
	createDKGFirstRoundOutputMock,
	createDWallet,
	dkgSecondRoundMoveCall,
} from '../../src/dwallet-mpc/dkg';
import {
	checkpointCreationTime,
	Config,
	delay,
	getDWalletSecpState,
	mockedProtocolPublicParameters,
} from '../../src/dwallet-mpc/globals';
import { dkgFirstRoundMock } from '../../src/dwallet-mpc/mocks';
import {getOrCreateClassGroupsKeyPair} from "../../src/dwallet-mpc/encrypt-user-share.ts";

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
			ikaConfig: require('../../../../ika_config.json'),
			dWalletSeed,
			encryptedSecretShareSigningKeypair,
		};
		await delay(2000);
	});

	it('should create a dWallet (DKG)', async () => {
		await createDWallet(conf, mockedProtocolPublicParameters);
	});

	it('should create the encryption key', async () => {
		let cgKeyPair = await getOrCreateClassGroupsKeyPair(conf);
		// log the encryption key base 64
		console.log(Buffer.from(cgKeyPair.encryptionKey).toString('base64'));
	});

	it('should run the DKG second round', async () => {
		let cgKeyPair = await getOrCreateClassGroupsKeyPair(conf);
		await delay(checkpointCreationTime);
		// log the encryption key base 64
		console.log(Buffer.from(cgKeyPair.encryptionKey).toString('base64'));

		let dwalletState = await getDWalletSecpState(conf);
		let dwalletCapID = await createDKGFirstRoundOutputMock(
			conf,
			Buffer.from(dkgFirstRoundMock.firstRoundOutput, 'base64'),
		);
		await delay(checkpointCreationTime);
		await dkgSecondRoundMoveCall(
			conf,
			dwalletState,
			{
				sessionID: dkgFirstRoundMock.sessionID,
				dwalletCapID: dwalletCapID,
				output: Buffer.from(dkgFirstRoundMock.firstRoundOutput, 'base64'),
			},
			Buffer.from(dkgFirstRoundMock.centralizedPublicKeyShareAndProof, 'base64'),
			Buffer.from(dkgFirstRoundMock.encryptedSecretShareAndProof, 'base64'),
			Buffer.from(dkgFirstRoundMock.centralizedPublicOutput, 'base64'),
		);
	});
});
