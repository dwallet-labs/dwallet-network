// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { Config, delay, mockedProtocolPublicParameters } from '../../src/dwallet-mpc/globals';

const fiveMinutes = 5 * 60 * 1000;
describe('Test dWallet MPC', () => {
	let conf: Config;

	beforeEach(async () => {
		const keypair = Ed25519Keypair.generate();
		const address = keypair.getPublicKey().toSuiAddress();
		const suiClient = new SuiClient({ url: getFullnodeUrl('localnet') });
		await requestSuiFromFaucetV1({
			host: getFaucetHost('localnet'),
			recipient: address,
		});
		conf = {
			keypair,
			client: suiClient,
			timeout: fiveMinutes,
			ikaConfig: require('../../../../ika_config.json'),
		};
		await delay(2000);
	});

	it('should create a dWallet (DKG)', async () => {
		await createDWallet(conf, mockedProtocolPublicParameters);
	});
});
