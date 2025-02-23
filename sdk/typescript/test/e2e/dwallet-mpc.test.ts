// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import { createDWallet, launchDKGSecondRound } from '../../src/dwallet-mpc/dkg';
import { getOrCreateClassGroupsKeyPair } from '../../src/dwallet-mpc/encrypt-user-share';
import { Config, delay, mockedProtocolPublicParameters } from '../../src/dwallet-mpc/globals';
import { dkgFirstRoundMock } from '../../src/dwallet-mpc/mocks';

const fiveMinutes = 5 * 60 * 1000;
describe('Test dWallet MPC', () => {
	let conf: Config;

	beforeEach(async () => {
		const keypair = Ed25519Keypair.deriveKeypairFromSeed('0x1');
		const address = keypair.getPublicKey().toSuiAddress();
		const suiClient = new SuiClient({ url: getFullnodeUrl('localnet') });
		await requestSuiFromFaucetV1({
			host: getFaucetHost('localnet'),
			recipient: address,
		});
		const dWalletSeed = new Uint8Array(32).fill(8);

		conf = {
			keypair,
			client: suiClient,
			timeout: fiveMinutes,
			ikaConfig: require('../../../../ika_config.json'),
			dWalletSeed,
		};
		await delay(2000);
	});

	it('should create a dWallet (DKG)', async () => {
		await createDWallet(conf, mockedProtocolPublicParameters);
	});

	it('should run DKG second round', async () => {
		await launchDKGSecondRound(
			conf,
			{
				sessionID: dkgFirstRoundMock.sessionID,
				dwalletCapID: dkgFirstRoundMock.dwalletCapID,
				output: Buffer.from(dkgFirstRoundMock.firstRoundOutput, 'base64'),
			},
			mockedProtocolPublicParameters,
		);
	});

	it('should get or create an encryption key', async () => {
		let classGroupsSecpKeyPair = await getOrCreateClassGroupsKeyPair(conf);
		console.log({ classGroupsSecpKeyPair });
	});
});
