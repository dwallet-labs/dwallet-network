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
	getNetworkDecryptionKeyPublicOutput,
	mockedNetworkDecryptionKeyPublicOutput,
	MPCKeyScheme,
} from '../../src/dwallet-mpc/globals';
import { mockCreatePresign, presign } from '../../src/dwallet-mpc/presign';
import {
	completeFutureSign,
	createUnverifiedECDSAPartialUserSignatureCap,
	Hash,
	sign,
	verifyECFSASignWithPartialUserSignatures,
} from '../../src/dwallet-mpc/sign';
import { dkgMocks, mockPresign } from './mocks';

const fiveMinutes = 100 * 60 * 1000;
describe('Test dWallet MPC', () => {
	let conf: Config;

	beforeEach(async () => {
		// todo(zeev): Think key is probably incorrect, check it.
		const keypair = Ed25519Keypair.deriveKeypairFromSeed('0x2');
		const dWalletSeed = new Uint8Array(32).fill(1);
		const encryptedSecretShareSigningKeypair = Ed25519Keypair.deriveKeypairFromSeed(
			Buffer.from(dWalletSeed).toString('hex'),
		);
		const address = keypair.getPublicKey().toSuiAddress();
		console.log(`Address: ${address}`);
		const suiClient = new SuiClient({ url: getFullnodeUrl('localnet') });
		// const suiClient = new SuiClient({ url: 'https://fullnode.sui.beta.devnet.ika-network.net' });
		await requestSuiFromFaucetV1({
			host: getFaucetHost('localnet'),
			// host: 'https://faucet.sui.beta.devnet.ika-network.net',
			recipient: address,
		});

		conf = {
			suiClientKeypair: keypair,
			client: suiClient,
			timeout: fiveMinutes,
			// todo(zeev): fix this, bad parsing, bad path, needs to be localized.
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
		const dwallet = await createDWallet(conf, mockedNetworkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet}`);
	});

	it('should mock create dwallet', async () => {
		const result = await mockCreateDWallet(
			conf,
			Buffer.from(dkgMocks.dwalletOutput, 'base64'),
			Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
		);
		console.log(`dWallet has been created successfully: ${result.dwalletID}`);
	});

	it('should run presign', async () => {
		const dwalletID = (
			await mockCreateDWallet(
				conf,
				Buffer.from(dkgMocks.dwalletOutput, 'base64'),
				Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
			)
		).dwalletID;
		console.log(`dWallet has been created successfully: ${dwalletID}`);
		const presignCompletion = await presign(conf, dwalletID);
		console.log(`presign has been created successfully: ${presignCompletion.presign_id}`);
	});

	it('should mock create presign', async () => {
		const dwalletID = (
			await mockCreateDWallet(
				conf,
				Buffer.from(dkgMocks.dwalletOutput, 'base64'),
				Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
			)
		).dwalletID;
		const presign = await mockCreatePresign(
			conf,
			Buffer.from(mockPresign.presignBytes, 'base64'),
			dwalletID,
		);
		console.log(`presign has been created successfully: ${presign}`);
	});

	it('should sign', async () => {
		const dkgResult = await mockCreateDWallet(
			conf,
			Buffer.from(dkgMocks.dwalletOutput, 'base64'),
			Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
		);
		await delay(checkpointCreationTime);
		const presign = await mockCreatePresign(
			conf,
			Buffer.from(mockPresign.presignBytes, 'base64'),
			dkgResult.dwalletID,
		);
		await delay(checkpointCreationTime);
		await sign(
			conf,
			presign.presign_id,
			dkgResult.dwallet_cap_id,
			Buffer.from('hello world'),
			Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
		);
	});

	it('should sign full flow', async () => {
		console.log('Creating dWallet...');
		const dwalletID = await createDWallet(conf, mockedNetworkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwalletID}`);
		await delay(checkpointCreationTime);
		console.log('Running Presign...');
		const presignCompletion = await presign(conf, dwalletID.dwalletID);
		console.log(`presign has been created successfully: ${presignCompletion.presign_id}`);
		await delay(checkpointCreationTime);
		console.log('Running Sign...');
		await sign(
			conf,
			presignCompletion.presign_id,
			dwalletID.dwallet_cap_id,
			Buffer.from('hello world'),
			dwalletID.secret_share,
			Hash.KECCAK256,
			mockedNetworkDecryptionKeyPublicOutput,
		);
	});

	it('should complete future sign', async () => {
		const dkgResult = await mockCreateDWallet(
			conf,
			Buffer.from(dkgMocks.dwalletOutput, 'base64'),
			Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
		);
		const presign = await mockCreatePresign(
			conf,
			Buffer.from(mockPresign.presignBytes, 'base64'),
			dkgResult.dwalletID,
		);
		await delay(checkpointCreationTime);
		const unverifiedECDSAPartialUserSignatureCapID =
			await createUnverifiedECDSAPartialUserSignatureCap(
				conf,
				presign.presign_id,
				dkgResult.dwallet_cap_id,
				Buffer.from('hello world'),
				Buffer.from(dkgMocks.centralizedSecretKeyShare, 'base64'),
				Hash.KECCAK256,
				mockedNetworkDecryptionKeyPublicOutput,
			);
		await delay(checkpointCreationTime);
		const verifiedECDSAPartialUserSignatureCapID = await verifyECFSASignWithPartialUserSignatures(
			conf,
			unverifiedECDSAPartialUserSignatureCapID!,
		);
		await delay(checkpointCreationTime);
		await completeFutureSign(
			conf,
			dkgResult.dwallet_cap_id,
			Buffer.from('hello world'),
			Hash.KECCAK256,
			verifiedECDSAPartialUserSignatureCapID,
		);
	});

	it('should sign full flow with on-chain network DKG output', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		console.log('Creating dWallet...');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);
		console.log('Starting Presign...');
		const presignCompletion = await presign(conf, dwallet.dwalletID);
		console.log(`presign has been created successfully: ${presignCompletion.presign_id}`);
		await delay(checkpointCreationTime);
		console.log('Running Sign...');
		await sign(
			conf,
			presignCompletion.presign_id,
			dwallet.dwallet_cap_id,
			Buffer.from('hello world'),
			dwallet.secret_share,
			Hash.KECCAK256,
			networkDecryptionKeyPublicOutput,
		);
	});
});

describe('Test dWallet MPC - offline', () => {
	it('should run sign centralized party', () => {
		const centralizedSignedMessage = create_sign_centralized_output(
			mockedNetworkDecryptionKeyPublicOutput,
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
