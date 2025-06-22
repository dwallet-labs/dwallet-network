// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import {
	acceptEncryptedUserShare,
	createDWalletCentralizedParty,
	executeDKGFirstRoundTransaction,
	executeDKGSecondRoundTransaction,
	prepareDKGFirstRoundTransaction,
	prepareDKGSecondRoundTransaction,
} from '../../../src/dwallet-mpc/dkg.js';
import { delay } from '../../../src/dwallet-mpc/globals.js';
import type { Config } from '../../../src/dwallet-mpc/globals.js';
import {
	executePresignTransaction,
	preparePresignTransaction,
} from '../../../src/dwallet-mpc/presign.js';
import {
	executeSignTransaction,
	Hash,
	prepareSignTransaction,
} from '../../../src/dwallet-mpc/sign.js';

/**
 * Helper function for random delays
 */
export function getRandomDelay(maxDelayMs: number): number {
	return Math.floor(Math.random() * maxDelayMs);
}

/**
 * Helper function to create concurrent tasks with random delays
 */
export async function createConcurrentTasks<T>(
	iterations: number,
	configs: Config[],
	taskFactory: (cfg: Config, index: number) => Promise<T>,
	maxDelayMs: number,
): Promise<T[]> {
	const startSignal = Promise.withResolvers<void>();
	const tasks: Promise<T>[] = [];

	for (let i = 0; i < iterations; i++) {
		const cfg = configs[i];
		tasks.push(
			(async () => {
				await startSignal.promise;
				await delay(getRandomDelay(maxDelayMs));
				return taskFactory(cfg, i);
			})(),
		);
	}

	startSignal.resolve();
	return Promise.all(tasks);
}

/**
 * Helper function to create configurations
 */
export async function createConfigurations(
	iterations: number,
	createConf: (dWalletSeed: Uint8Array<ArrayBuffer>, keypairSeed: string | null) => Promise<Config>,
): Promise<Config[]> {
	return Promise.all(
		Array.from({ length: iterations }, () =>
			createConf(crypto.getRandomValues(new Uint8Array(32)), null),
		),
	);
}

/**
 * Helper function to prepare and execute DKG first round
 */
export async function prepareAndExecuteDKGFirstRound(cfg: Config): Promise<any> {
	const tx = await prepareDKGFirstRoundTransaction(cfg);
	return executeDKGFirstRoundTransaction(cfg, tx);
}

/**
 * Helper function to create centralized party
 */
export async function createCentralizedParty(
	cfg: Config,
	networkDecryptionKeyPublicOutput: Uint8Array,
	dkgFirst: any,
): Promise<any> {
	return createDWalletCentralizedParty(cfg, networkDecryptionKeyPublicOutput, dkgFirst);
}

/**
 * Helper function to prepare and execute DKG second round
 */
export async function prepareAndExecuteDKGSecondRound(
	cfg: Config,
	dWalletStateData: any,
	firstDKGRoundOutput: any,
	centralizedPartyOutput: any,
): Promise<[any, Uint8Array]> {
	const tx = await prepareDKGSecondRoundTransaction(
		cfg,
		dWalletStateData,
		firstDKGRoundOutput,
		centralizedPartyOutput.centralizedPublicKeyShareAndProof,
		centralizedPartyOutput.encryptedUserShareAndProof,
		centralizedPartyOutput.centralizedPublicOutput,
	);

	const centralizedSecretKeyShare = centralizedPartyOutput.centralizedSecretKeyShare;
	const secondRoundResponse = await executeDKGSecondRoundTransaction(cfg, firstDKGRoundOutput, tx);

	await acceptEncryptedUserShare(cfg, {
		dwallet_id: secondRoundResponse.dwallet.id.id,
		encrypted_user_secret_key_share_id: secondRoundResponse.encrypted_user_secret_key_share_id,
	});

	return [secondRoundResponse, centralizedSecretKeyShare];
}

/**
 * Helper function to prepare and execute presign
 */
export async function prepareAndExecutePresign(cfg: Config, dwalletId: string): Promise<any> {
	const tx = await preparePresignTransaction(cfg, dwalletId);
	return executePresignTransaction(cfg, tx);
}

/**
 * Helper function to prepare sign transaction
 */
export async function prepareSignTransactionForFlow(
	cfg: Config,
	presignResult: any,
	dkgFirst: any,
	centralizedSecretKeyShare: Uint8Array,
	networkDecryptionKeyPublicOutput: Uint8Array,
): Promise<any> {
	return prepareSignTransaction(
		cfg,
		presignResult.id.id,
		dkgFirst.dwalletCapID,
		new TextEncoder().encode('hello world'),
		centralizedSecretKeyShare,
		networkDecryptionKeyPublicOutput,
		Hash.KECCAK256,
	);
}

/**
 * Helper function to execute sign transaction
 */
export async function executeSignTransactionForFlow(cfg: Config, signTx: any): Promise<any> {
	return executeSignTransaction(signTx, cfg);
}
