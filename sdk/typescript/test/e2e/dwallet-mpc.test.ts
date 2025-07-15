// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import path from 'path';
import { sample_dwallet_keypair, verify_secp_signature } from '@dwallet-network/dwallet-mpc-wasm';
import { getFullnodeUrl, SuiClient } from '@mysten/sui/client';
import { getFaucetHost, requestSuiFromFaucetV2 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, expect, it } from 'vitest';

import {
	acceptEncryptedUserShare,
	createDWallet,
	createDWalletCentralizedParty,
	executeDKGFirstRoundTransaction,
	executeDKGSecondRoundTransaction,
	launchDKGFirstRound,
	prepareDKGFirstRoundTransaction,
	prepareDKGSecondRoundTransaction,
} from '../../src/dwallet-mpc/dkg';
import {
	checkpointCreationTime,
	Config,
	delay,
	getDWalletSecpState,
	getNetworkPublicParameters,
	getObjectWithType,
} from '../../src/dwallet-mpc/globals';
import { createImportedDWallet } from '../../src/dwallet-mpc/import-dwallet';
import {
	executePresignTransaction,
	preparePresignTransaction,
	presign,
} from '../../src/dwallet-mpc/presign';
import {
	isDWalletWithPublicUserSecretKeyShares,
	makeDWalletUserSecretKeySharesPublicRequestEvent,
} from '../../src/dwallet-mpc/publish_secret_share';
import {
	completeFutureSign,
	createUnverifiedPartialUserSignatureCap,
	executeSignTransaction,
	Hash,
	prepareSignTransaction,
	sign,
	signWithImportedDWallet,
	verifySignWithPartialUserSignatures,
} from '../../src/dwallet-mpc/sign';

async function createConf(
	dWalletSeed: Uint8Array<ArrayBuffer>,
	keypairSeed: string | null,
): Promise<Config> {
	const keypair =
		keypairSeed == null
			? Ed25519Keypair.generate()
			: Ed25519Keypair.deriveKeypairFromSeed(keypairSeed);
	const encryptedSecretShareSigningKeypair = Ed25519Keypair.deriveKeypairFromSeed(
		Buffer.from(dWalletSeed).toString('hex'),
	);
	const address = keypair.getPublicKey().toSuiAddress();
	console.log(`Address: ${address}`);
	const suiClient = new SuiClient({ url: getFullnodeUrl('localnet') });
	// const suiClient = new SuiClient({ url: 'https://fullnode.sui.beta.devnet.ika-network.net' });
	await requestSuiFromFaucetV2({
		host: getFaucetHost('localnet'),
		// host: 'https://faucet.sui.beta.devnet.ika-network.net',
		recipient: address,
	});

	return {
		suiClientKeypair: keypair,
		client: suiClient,
		timeout: fiveMinutes,
		// todo(zeev): fix this, bad parsing, bad path, needs to be localized.
		ikaConfig: require(path.resolve(process.cwd(), '../../ika_config.json')),
		dWalletSeed,
		encryptedSecretShareSigningKeypair,
	};
}

const fiveMinutes = 100 * 60 * 1000;

// Helper function for random delays
function getRandomDelay(maxDelayMs: number): number {
	return Math.floor(Math.random() * maxDelayMs);
}

describe('Test dWallet MPC', () => {
	let conf: Config;

	beforeEach(async () => {
		const dWalletSeed = new Uint8Array(32).fill(9);
		conf = await createConf(dWalletSeed, '0x2');
		await delay(2000);
	});

	it(
		'run multiple full flows simultaneously',
		async () => {
			const iterations = 2;
			const maxDelayBeforeMPCRequestSec = 1000 * 5 * 0;
			const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);

			// Create a new configuration for each iteration
			const configs = await Promise.all(
				Array.from({ length: iterations }, () =>
					createConf(crypto.getRandomValues(new Uint8Array(32)), null),
				),
			);

			// -----------------------------
			// Phase 1: DKG Initialization
			// -----------------------------
			const dkgFirstStartSignal = Promise.withResolvers();
			const dkgFirstTasks = [];

			for (let i = 0; i < iterations; i++) {
				const cfg = configs[i];
				const tx = await prepareDKGFirstRoundTransaction(cfg);
				dkgFirstTasks.push(
					(async () => {
						await dkgFirstStartSignal.promise;
						await delay(getRandomDelay(maxDelayBeforeMPCRequestSec));
						console.time(`DKG first round: ${cfg.suiClientKeypair.getPublicKey().toSuiAddress()}`);
						const dkgFirstRoundOutput = await executeDKGFirstRoundTransaction(cfg, tx);
						console.timeEnd(
							`DKG first round: ${cfg.suiClientKeypair.getPublicKey().toSuiAddress()}`,
						);
						console.log(
							`DKG first round: ${cfg.suiClientKeypair.getPublicKey().toSuiAddress()}, session ID : ${dkgFirstRoundOutput.sessionIdentifier}`,
						);
						return dkgFirstRoundOutput;
					})(),
				);
			}

			dkgFirstStartSignal.resolve();

			const dkgFirsts = await Promise.all(dkgFirstTasks);

			const centralizedSecretKeySharesTsks = [];
			for (let i = 0; i < iterations; i++) {
				const cfg = configs[i];
				const dkgFirst = dkgFirsts[i];
				centralizedSecretKeySharesTsks.push(
					(async () => {
						return createDWalletCentralizedParty(cfg, networkDecryptionKeyPublicOutput, dkgFirst);
					})(),
				);
			}

			const centralizedPartyOutputs = await Promise.all(centralizedSecretKeySharesTsks);
			const dWalletStateData = await getDWalletSecpState(conf);

			const dkgSeconsStartSignal = Promise.withResolvers();
			const dkgSecondTasks = [];
			for (let i = 0; i < iterations; i++) {
				const cfg = configs[i];
				const firstDKGRoundOutput = dkgFirsts[i];
				const centralizedPartyOutput = centralizedPartyOutputs[i];
				const tx = await prepareDKGSecondRoundTransaction(
					cfg,
					dWalletStateData,
					firstDKGRoundOutput,
					centralizedPartyOutput.centralizedPublicKeyShareAndProof,
					centralizedPartyOutput.encryptedUserShareAndProof,
					centralizedPartyOutput.centralizedPublicOutput,
				);
				dkgSecondTasks.push(
					(async () => {
						await dkgSeconsStartSignal.promise;
						const centralizedSecretKeyShare = centralizedPartyOutputs[i].centralizedSecretKeyShare;
						await delay(getRandomDelay(maxDelayBeforeMPCRequestSec));
						console.time(`DKG second round: ${cfg.suiClientKeypair.getPublicKey().toSuiAddress()}`);
						const secondRoundResponse = await executeDKGSecondRoundTransaction(
							cfg,
							firstDKGRoundOutput,
							tx,
						);
						console.timeEnd(
							`DKG second round: ${cfg.suiClientKeypair.getPublicKey().toSuiAddress()}`,
						);
						console.log(
							`DKG second round: ${cfg.suiClientKeypair.getPublicKey().toSuiAddress()}, session ID : ${secondRoundResponse.dwallet.id.id}`,
						);
						await acceptEncryptedUserShare(cfg, {
							dwallet_id: secondRoundResponse.dwallet.id.id,
							encrypted_user_secret_key_share_id:
								secondRoundResponse.encrypted_user_secret_key_share_id,
						});
						return [secondRoundResponse, centralizedSecretKeyShare];
					})(),
				);
			}

			dkgSeconsStartSignal.resolve();
			const dwallets = await Promise.all(dkgSecondTasks);

			await delay(checkpointCreationTime);

			// -----------------------------
			// Phase 2: Presign
			// -----------------------------
			const presignStartSignal = Promise.withResolvers();
			const presignTasks = [];

			for (let i = 0; i < iterations; i++) {
				const cfg = configs[i];
				const [dwallet, _] = dwallets[i];
				const tx = await preparePresignTransaction(cfg, dwallet.dwallet.id.id);
				presignTasks.push(
					(async () => {
						await presignStartSignal.promise;
						await delay(getRandomDelay(maxDelayBeforeMPCRequestSec));
						return executePresignTransaction(cfg, tx);
					})(),
				);
			}

			presignStartSignal.resolve();
			const presignResults = await Promise.all(presignTasks);

			// -----------------------------
			// Phase 3: Sign and Send
			// -----------------------------
			const startSignal = Promise.withResolvers();
			const signAndSendTasks: Promise<any>[] = [];

			await delay(checkpointCreationTime);

			for (let i = 0; i < iterations; i++) {
				const cfg = configs[i];
				const [_, centralizedSecretKeyShare] = dwallets[i];
				const dkgFirst = dkgFirsts[i];
				const presignResult = presignResults[i];
				signAndSendTasks.push(
					(async () => {
						return prepareSignTransaction(
							cfg,
							presignResult.id.id,
							dkgFirst.dwalletCapID,
							Buffer.from('hello world'),
							centralizedSecretKeyShare,
							networkDecryptionKeyPublicOutput,
							Hash.KECCAK256,
						);
					})(),
				);
			}

			const signTxs = await Promise.all(signAndSendTasks);

			for (let i = 0; i < iterations; i++) {
				const cfg = configs[i];
				const signTx = signTxs[i];
				signAndSendTasks.push(
					(async () => {
						await startSignal.promise;
						await delay(getRandomDelay(maxDelayBeforeMPCRequestSec));
						console.time(`Sign: ${cfg.suiClientKeypair.toSuiAddress()}`);
						const signRes = await executeSignTransaction(signTx, cfg);
						console.timeEnd(`Sign: ${conf.suiClientKeypair.toSuiAddress()}`);
						console.log(`Sign: ${cfg.suiClientKeypair.toSuiAddress()} - ${signRes.id.id}`);
						return signRes;
					})(),
				);
			}

			startSignal.resolve();
			await Promise.all(signAndSendTasks);
		},
		70 * 1000 * 60,
	);

	it('read the network decryption key', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
		console.log(`networkDecryptionKeyPublicOutput: ${networkDecryptionKeyPublicOutput}`);
	});

	it('should create a dWallet (DKG)', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
	});

	it('should run presign', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.log(`dWallet has been created successfully: ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);
		const completedPresign = await presign(conf, dwallet.dwalletID);
		console.log(`presign has been created successfully: ${completedPresign.id.id}`);
	});

	it('should sign full flow', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
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
		console.log(`Sing completed successfully: ${signRes.id.id}`);
		console.timeEnd('Step 3: Sign Phase');
	});

	it('should create a dwallet and publish its secret share', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
		console.log('Step 1: dWallet Creation');
		console.time('Step 1: dWallet Creation');
		const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
		console.timeEnd('Step 1: dWallet Creation');
		console.log(`Step 1: dWallet created | dWalletID = ${dwallet.dwalletID}`);
		await delay(checkpointCreationTime);
		console.log('Running publish secret share...');
		await makeDWalletUserSecretKeySharesPublicRequestEvent(
			conf,
			dwallet.dwalletID,
			dwallet.secret_share,
		);
	});

	it('should create a dwallet, publish its secret share and sign with the published share', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
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
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);

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
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
		const [secretKey, _publicKey] = sample_dwallet_keypair(networkDecryptionKeyPublicOutput);
		const dwallet = await createImportedDWallet(conf, secretKey);
		console.log({ ...dwallet });
	});

	it('should create an imported dWallet, publish its secret share and sign with it', async () => {
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
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
		const networkDecryptionKeyPublicOutput = await getNetworkPublicParameters(conf);
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
