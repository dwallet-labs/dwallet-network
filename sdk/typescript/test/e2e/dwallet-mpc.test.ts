// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import path from 'path';
import { sample_dwallet_keypair, verify_secp_signature } from '@dwallet-network/dwallet-mpc-wasm';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV2 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, expect, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import {
	checkpointCreationTime,
	Config,
	delay,
	getNetworkDecryptionKeyPublicOutput,
	getObjectWithType,
} from '../../src/dwallet-mpc/globals';
import { createImportedDWallet } from '../../src/dwallet-mpc/import-dwallet';
import { presign } from '../../src/dwallet-mpc/presign';
import {
	isDWalletWithPublicUserSecretKeyShares,
	makeDWalletUserSecretKeySharesPublicRequestEvent,
} from '../../src/dwallet-mpc/publish_secret_share';
import {
	completeFutureSign,
	createUnverifiedPartialUserSignatureCap,
	Hash,
	sign,
	signWithImportedDWallet,
	verifySignWithPartialUserSignatures,
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
		await requestSuiFromFaucetV2({
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
		console.time('Step 1: dWallet Creation');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
		console.timeEnd('Step 1: dWallet Creation');
		await delay(checkpointCreationTime);
		console.log('Running Presign...');
		console.time('Step 2: Presign Phase');
		const completedPresign = await presign(conf, dwallet.dwalletID);
		console.timeEnd('Step 2: Presign Phase');
		console.log(`Step 2: Presign completed | presignID = ${completedPresign.id.id}`);
		await delay(checkpointCreationTime);
		console.log('Running Sign...');
		console.time('Step 3: Sign Phase');
		const signRes = await sign(
			conf,
			completedPresign.id.id,
			dwallet.dwallet_cap_id,
			Buffer.from('hello world'),
			dwallet.secret_share,
			networkDecryptionKeyPublicOutput,
			Hash.KECCAK256,
		);
		console.log(`Sing completed successfully: ${signRes.sign_id}`);
		console.timeEnd('Step 3: Sign Phase');
	});

	it('should create a dwallet and publish its secret share', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		console.log('Creating dWallet...');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);
		console.log('Running publish secret share...');
		await makeDWalletUserSecretKeySharesPublicRequestEvent(
			conf,
			dwallet.dwalletID,
			dwallet.secret_share,
		);
	});

	it('should create a dwallet, publish its secret share and sign with the published share', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		console.log('Creating dWallet...');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);
		console.log('Running publish secret share...');
		await makeDWalletUserSecretKeySharesPublicRequestEvent(
			conf,
			dwallet.dwalletID,
			dwallet.secret_share,
		);
		const dwalletWithSecretShare = await getObjectWithType(
			conf,
			dwallet.dwalletID,
			isDWalletWithPublicUserSecretKeyShares,
		);
		console.log(`secretShare: ${dwalletWithSecretShare}`);
		console.log('Running Presign...');
		const completedPresign = await presign(conf, dwalletWithSecretShare.id.id);
		console.log(`presign has been created successfully: ${completedPresign.id.id}`);
		await delay(checkpointCreationTime);
		console.log('Running Sign...');
		await sign(
			conf,
			completedPresign.id.id,
			dwalletWithSecretShare.dwallet_cap_id,
			Buffer.from('hello world'),
			dwalletWithSecretShare.public_user_secret_key_share,
			networkDecryptionKeyPublicOutput,
			Hash.KECCAK256,
		);
	});

	it('should complete future sign', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);

		console.log('Step 1: dWallet Creation');
		console.time('Step 1: dWallet Creation');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.timeEnd('Step 1: dWallet Creation');
		console.log(`Step 1: dWallet created | dWalletID = ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);

		console.log('Step 2: Presign Phase');
		console.time('Step 2: Presign Phase');
		const completedPresign = await presign(conf, dwallet.dwalletID);
		console.timeEnd('Step 2: Presign Phase');
		console.log(`Step 2: Presign completed | presignID = ${completedPresign.id.id}`);
		await delay(checkpointCreationTime);
		const unverifiedPartialUserSignatureCapID = await createUnverifiedPartialUserSignatureCap(
			conf,
			completedPresign.id.id,
			dwallet.dwallet_cap_id,
			Buffer.from('hello world'),
			dwallet.secret_share,
			networkDecryptionKeyPublicOutput,
			Hash.KECCAK256,
		);
		await delay(checkpointCreationTime);
		const verifiedPartialUserSignatureCapID = await verifySignWithPartialUserSignatures(
			conf,
			unverifiedPartialUserSignatureCapID!,
		);
		await delay(checkpointCreationTime);
		await completeFutureSign(
			conf,
			dwallet.dwallet_cap_id,
			Buffer.from('hello world'),
			Hash.KECCAK256,
			verifiedPartialUserSignatureCapID,
		);
	});

	it('should create an imported dWallet', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		const [secretKey, _publicKey] = sample_dwallet_keypair(networkDecryptionKeyPublicOutput);
		const dwallet = await createImportedDWallet(conf, secretKey);
		console.log({ ...dwallet });
	});

	it('should create an imported dWallet, publish its secret share and sign with it', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		const [secretKey, _publicKey] = sample_dwallet_keypair(networkDecryptionKeyPublicOutput);
		const dwallet = await createImportedDWallet(conf, secretKey);
		await delay(checkpointCreationTime);
		console.log({ ...dwallet });
		console.log('Running publish secret share...');
		await makeDWalletUserSecretKeySharesPublicRequestEvent(
			conf,
			dwallet.dwalletID,
			dwallet.secret_share,
		);
		const dwalletWithSecretShare = await getObjectWithType(
			conf,
			dwallet.dwalletID,
			isDWalletWithPublicUserSecretKeyShares,
		);
		console.log(`secretShare: ${dwalletWithSecretShare}`);
		console.log('Running Presign...');
		const completedPresign = await presign(conf, dwalletWithSecretShare.id.id);
		console.log(`presign has been created successfully: ${completedPresign.id.id}`);
		await delay(checkpointCreationTime);
		console.log('Running Sign...');
		await signWithImportedDWallet(
			conf,
			completedPresign.id.id,
			dwalletWithSecretShare.dwallet_cap_id,
			Buffer.from('hello world'),
			dwalletWithSecretShare.public_user_secret_key_share,
			networkDecryptionKeyPublicOutput,
			Hash.KECCAK256,
		);
	});

	it('should create an imported dWallet, sign with it & verify the signature against the original public key', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		const [secretKey, publicKey] = sample_dwallet_keypair(networkDecryptionKeyPublicOutput);
		const dwallet = await createImportedDWallet(conf, secretKey);
		console.log({ ...dwallet });
		await delay(checkpointCreationTime);
		console.log('Running Presign...');
		const completedPresign = await presign(conf, dwallet.dwalletID);
		console.log(`presign has been created successfully: ${completedPresign.id.id}`);
		await delay(checkpointCreationTime);
		console.log('Running Sign...');
		const signature = await signWithImportedDWallet(
			conf,
			completedPresign.id.id,
			dwallet.dwallet_cap_id,
			Buffer.from('hello world'),
			dwallet.secret_share,
			networkDecryptionKeyPublicOutput,
			Hash.KECCAK256,
		);
		const isValid = verify_secp_signature(
			publicKey,
			signature.state.fields.signature,
			Buffer.from('hello world'),
			networkDecryptionKeyPublicOutput,
			Hash.KECCAK256,
		);
		expect(isValid).toBeTruthy();
	});
});
