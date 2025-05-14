// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import path from 'path';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV1 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import {
	checkpointCreationTime,
	Config,
	delay,
	getNetworkDecryptionKeyPublicOutput,
} from '../../src/dwallet-mpc/globals';
import { presign } from '../../src/dwallet-mpc/presign';
import {
	completeFutureSign,
	createUnverifiedECDSAPartialUserSignatureCap,
	Hash,
	sign,
	verifyECFSASignWithPartialUserSignatures,
} from '../../src/dwallet-mpc/sign';

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
		console.log(`Address: ${address}`);
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

	it('read the network decryption key', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		console.log(`networkDecryptionKeyPublicOutput: ${networkDecryptionKeyPublicOutput}`);
	});

	it('should create a dWallet (DKG)', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet}`);
	});

	it('should run presign', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet}`);
		const completedPresign = await presign(conf, dwallet.dwalletID);
		console.log(`presign has been created successfully: ${completedPresign.id.id}`);
	});

	it('should sign full flow', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		console.log('Creating dWallet...');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);
		console.log('Running Presign...');
		const completedPresign = await presign(conf, dwallet.dwalletID);
		console.log(`presign has been created successfully: ${completedPresign.id.id}`);
		await delay(checkpointCreationTime);
		console.log('Running Sign...');
		await sign(
			conf,
			completedPresign.id.id,
			dwallet.dwallet_cap_id,
			Buffer.from('hello world'),
			dwallet.secret_share,
			networkDecryptionKeyPublicOutput,
			Hash.KECCAK256,
		);
	});

	it('should complete future sign', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		console.log('Creating dWallet...');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);
		console.log('Starting Presign...');
		const completedPresign = await presign(conf, dwallet.dwalletID);
		console.log(`presign has been created successfully: ${completedPresign.id.id}`);
		await delay(checkpointCreationTime);
		const unverifiedECDSAPartialUserSignatureCapID =
			await createUnverifiedECDSAPartialUserSignatureCap(
				conf,
				completedPresign.id.id,
				dwallet.dwallet_cap_id,
				Buffer.from('hello world'),
				dwallet.secret_share,
				networkDecryptionKeyPublicOutput,
				Hash.KECCAK256,
			);
		await delay(checkpointCreationTime);
		const verifiedECDSAPartialUserSignatureCapID = await verifyECFSASignWithPartialUserSignatures(
			conf,
			unverifiedECDSAPartialUserSignatureCapID!,
		);
		await delay(checkpointCreationTime);
		await completeFutureSign(
			conf,
			dwallet.dwallet_cap_id,
			Buffer.from('hello world'),
			Hash.KECCAK256,
			verifiedECDSAPartialUserSignatureCapID,
		);
	});
});
