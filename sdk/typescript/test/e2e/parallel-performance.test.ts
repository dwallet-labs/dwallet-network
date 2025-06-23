// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import path from 'path';
import { SuiClient } from '@mysten/sui/client';
import { requestSuiFromFaucetV2 } from '@mysten/sui/faucet';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import { beforeEach, describe, expect, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import {
	checkpointCreationTime,
	Config,
	delay,
	getNetworkDecryptionKeyPublicOutput,
} from '../../src/dwallet-mpc/globals';
import { presign } from '../../src/dwallet-mpc/presign';
import { Hash, sign } from '../../src/dwallet-mpc/sign';

// Configuration for parallel tests
interface ParallelTestConfig {
	parallelCount: number;
	timeout: number;
}

// Default configuration - 8 parallel operations
const DEFAULT_PARALLEL_CONFIG: ParallelTestConfig = {
	parallelCount: parseInt(process.env.PARALLEL_COUNT || '8'),
	timeout: parseInt(process.env.TEST_TIMEOUT || '600000'), // 10 minutes default
};

// Performance metrics tracking
interface PerformanceMetrics {
	operation: string;
	index: number;
	startTime: number;
	endTime: number;
	duration: number;
	success: boolean;
	error?: string;
}

// Helper function to measure execution time
async function measureOperation<T>(
	operation: string,
	index: number,
	fn: () => Promise<T>,
): Promise<{ result: T; metrics: PerformanceMetrics }> {
	const startTime = Date.now();
	let success = true;
	let error: string | undefined;
	let result: T;

	try {
		result = await fn();
	} catch (e) {
		success = false;
		error = e instanceof Error ? e.message : String(e);
		throw e;
	} finally {
		const endTime = Date.now();
		const metrics: PerformanceMetrics = {
			operation,
			index,
			startTime,
			endTime,
			duration: endTime - startTime,
			success,
			error,
		};
		console.log(
			`[${operation}] #${index + 1}: ${success ? 'SUCCESS' : 'FAILED'} - ${metrics.duration}ms${
				error ? ` (${error})` : ''
			}`,
		);
	}

	return { result: result!, metrics };
}

// Helper function to print performance summary
function printPerformanceSummary(metrics: PerformanceMetrics[], operation: string) {
	const successful = metrics.filter((m) => m.success);
	const failed = metrics.filter((m) => !m.success);
	const durations = successful.map((m) => m.duration);

	if (durations.length === 0) {
		console.log(`\n📊 ${operation} Performance Summary: ALL FAILED`);
		return;
	}

	const avgDuration = durations.reduce((a, b) => a + b, 0) / durations.length;
	const minDuration = Math.min(...durations);
	const maxDuration = Math.max(...durations);
	const totalDuration =
		Math.max(...metrics.map((m) => m.endTime)) - Math.min(...metrics.map((m) => m.startTime));

	console.log(`\n📊 ${operation} Performance Summary:`);
	console.log(`   Total Operations: ${metrics.length}`);
	console.log(`   Successful: ${successful.length}`);
	console.log(`   Failed: ${failed.length}`);
	console.log(`   Success Rate: ${((successful.length / metrics.length) * 100).toFixed(1)}%`);
	console.log(`   Average Duration: ${avgDuration.toFixed(0)}ms`);
	console.log(`   Min Duration: ${minDuration}ms`);
	console.log(`   Max Duration: ${maxDuration}ms`);
	console.log(`   Total Wall Time: ${totalDuration}ms`);
	console.log(`   Throughput: ${(successful.length / (totalDuration / 1000)).toFixed(2)} ops/sec`);

	if (failed.length > 0) {
		console.log(`\n❌ Failed Operations:`);
		failed.forEach((f) => console.log(`   #${f.index + 1}: ${f.error}`));
	}
}

const fiveMinutes = 100 * 60 * 1000;
describe('Parallel Performance Tests', () => {
	let conf: Config;
	let networkDecryptionKeyPublicOutput: string;

	beforeEach(async () => {
		const keypair = Ed25519Keypair.deriveKeypairFromSeed('0x2');
		const dWalletSeed = new Uint8Array(32).fill(9);
		const encryptedSecretShareSigningKeypair = Ed25519Keypair.deriveKeypairFromSeed(
			Buffer.from(dWalletSeed).toString('hex'),
		);
		const address = keypair.getPublicKey().toSuiAddress();
		console.log(`Address: ${address}`);

		const suiClient = new SuiClient({ url: 'https://fullnode.sui.beta.devnet.ika-network.net' });
		await requestSuiFromFaucetV2({
			host: 'https://faucet.sui.beta.devnet.ika-network.net',
			recipient: address,
		});

		conf = {
			suiClientKeypair: keypair,
			client: suiClient,
			timeout: DEFAULT_PARALLEL_CONFIG.timeout,
			ikaConfig: require(path.resolve(process.cwd(), '../../ika_config.json')),
			dWalletSeed,
			encryptedSecretShareSigningKeypair,
		};

		// Get network decryption key once for all tests
		networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
		await delay(2000);
	});

	it(
		`should run DKG ${DEFAULT_PARALLEL_CONFIG.parallelCount} times in parallel`,
		async () => {
			console.log(
				`\n🚀 Starting ${DEFAULT_PARALLEL_CONFIG.parallelCount} parallel DKG operations...`,
			);

			const promises = Array.from({ length: DEFAULT_PARALLEL_CONFIG.parallelCount }, (_, index) =>
				measureOperation(`DKG`, index, async () => {
					const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
					return dwallet;
				}),
			);

			const results = await Promise.allSettled(promises);
			const metrics = results.map((result, index) => {
				if (result.status === 'fulfilled') {
					return result.value.metrics;
				} else {
					return {
						operation: 'DKG',
						index,
						startTime: Date.now(),
						endTime: Date.now(),
						duration: 0,
						success: false,
						error: result.reason?.message || 'Unknown error',
					} as PerformanceMetrics;
				}
			});

			printPerformanceSummary(metrics, 'DKG');

			// Ensure at least some operations succeeded
			const successCount = metrics.filter((m) => m.success).length;
			expect(successCount).toBeGreaterThan(0);
		},
		DEFAULT_PARALLEL_CONFIG.timeout,
	);

	it(
		`should run DKG once and Presign ${DEFAULT_PARALLEL_CONFIG.parallelCount} times in parallel`,
		async () => {
			console.log(
				`\n🚀 Creating single dWallet for ${DEFAULT_PARALLEL_CONFIG.parallelCount} parallel Presign operations...`,
			);

			// Create one dWallet first
			console.time('DKG Setup');
			const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
			console.timeEnd('DKG Setup');
			console.log(`dWallet created: ${dwallet.dwalletID}`);

			await delay(checkpointCreationTime);

			console.log(
				`\n🚀 Starting ${DEFAULT_PARALLEL_CONFIG.parallelCount} parallel Presign operations...`,
			);

			const promises = Array.from({ length: DEFAULT_PARALLEL_CONFIG.parallelCount }, (_, index) =>
				measureOperation(`Presign`, index, async () => {
					const completedPresign = await presign(conf, dwallet.dwalletID);
					return completedPresign;
				}),
			);

			const results = await Promise.allSettled(promises);
			const metrics = results.map((result, index) => {
				if (result.status === 'fulfilled') {
					return result.value.metrics;
				} else {
					return {
						operation: 'Presign',
						index,
						startTime: Date.now(),
						endTime: Date.now(),
						duration: 0,
						success: false,
						error: result.reason?.message || 'Unknown error',
					} as PerformanceMetrics;
				}
			});

			printPerformanceSummary(metrics, 'Presign (Single DKG)');

			const successCount = metrics.filter((m) => m.success).length;
			expect(successCount).toBeGreaterThan(0);
		},
		DEFAULT_PARALLEL_CONFIG.timeout,
	);

	it(
		`should run DKG once, Presign once, and Sign ${DEFAULT_PARALLEL_CONFIG.parallelCount} times in parallel`,
		async () => {
			console.log(
				`\n🚀 Setting up DKG and Presign for ${DEFAULT_PARALLEL_CONFIG.parallelCount} parallel Sign operations...`,
			);

			// Create one dWallet first
			console.time('DKG Setup');
			const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
			console.timeEnd('DKG Setup');
			console.log(`dWallet created: ${dwallet.dwalletID}`);

			await delay(checkpointCreationTime);

			// Create one Presign
			console.time('Presign Setup');
			const completedPresign = await presign(conf, dwallet.dwalletID);
			console.timeEnd('Presign Setup');
			console.log(`Presign created: ${completedPresign.id.id}`);

			await delay(checkpointCreationTime);

			console.log(
				`\n🚀 Starting ${DEFAULT_PARALLEL_CONFIG.parallelCount} parallel Sign operations...`,
			);

			const promises = Array.from({ length: DEFAULT_PARALLEL_CONFIG.parallelCount }, (_, index) =>
				measureOperation(`Sign`, index, async () => {
					const message = Buffer.from(`hello world ${index}`); // Unique message per sign
					const signRes = await sign(
						conf,
						completedPresign.id.id,
						dwallet.dwallet_cap_id,
						message,
						dwallet.secret_share,
						networkDecryptionKeyPublicOutput,
						Hash.KECCAK256,
					);
					return signRes;
				}),
			);

			const results = await Promise.allSettled(promises);
			const metrics = results.map((result, index) => {
				if (result.status === 'fulfilled') {
					return result.value.metrics;
				} else {
					return {
						operation: 'Sign',
						index,
						startTime: Date.now(),
						endTime: Date.now(),
						duration: 0,
						success: false,
						error: result.reason?.message || 'Unknown error',
					} as PerformanceMetrics;
				}
			});

			printPerformanceSummary(metrics, 'Sign (Single DKG + Presign)');

			const successCount = metrics.filter((m) => m.success).length;
			expect(successCount).toBeGreaterThan(0);
		},
		DEFAULT_PARALLEL_CONFIG.timeout,
	);

	it(
		`should run full flow (DKG + Presign + Sign) ${DEFAULT_PARALLEL_CONFIG.parallelCount} times in parallel`,
		async () => {
			console.log(
				`\n🚀 Starting ${DEFAULT_PARALLEL_CONFIG.parallelCount} parallel full flow operations...`,
			);

			const promises = Array.from({ length: DEFAULT_PARALLEL_CONFIG.parallelCount }, (_, index) =>
				measureOperation(`Full Flow`, index, async () => {
					// DKG
					const dwallet = await createDWallet(conf, networkDecryptionKeyPublicOutput);
					await delay(checkpointCreationTime);

					// Presign
					const completedPresign = await presign(conf, dwallet.dwalletID);
					await delay(checkpointCreationTime);

					// Sign
					const message = Buffer.from(`hello world ${index}`);
					const signRes = await sign(
						conf,
						completedPresign.id.id,
						dwallet.dwallet_cap_id,
						message,
						dwallet.secret_share,
						networkDecryptionKeyPublicOutput,
						Hash.KECCAK256,
					);

					return {
						dwallet,
						presign: completedPresign,
						sign: signRes,
					};
				}),
			);

			const results = await Promise.allSettled(promises);
			const metrics = results.map((result, index) => {
				if (result.status === 'fulfilled') {
					return result.value.metrics;
				} else {
					return {
						operation: 'Full Flow',
						index,
						startTime: Date.now(),
						endTime: Date.now(),
						duration: 0,
						success: false,
						error: result.reason?.message || 'Unknown error',
					} as PerformanceMetrics;
				}
			});

			printPerformanceSummary(metrics, 'Full Flow (DKG + Presign + Sign)');

			const successCount = metrics.filter((m) => m.success).length;
			expect(successCount).toBeGreaterThan(0);
		},
		DEFAULT_PARALLEL_CONFIG.timeout * 2,
	); // Double timeout for full flow

	it(
		'should run comprehensive performance comparison',
		async () => {
			console.log(
				`\n🚀 Running comprehensive performance comparison with ${Math.min(4, DEFAULT_PARALLEL_CONFIG.parallelCount)} operations each...`,
			);

			const testCount = Math.min(4, DEFAULT_PARALLEL_CONFIG.parallelCount);
			const allMetrics: PerformanceMetrics[] = [];

			// Test 1: DKG only
			console.log(`\n1️⃣ Testing ${testCount} DKG operations...`);
			const dkgPromises = Array.from({ length: testCount }, (_, index) =>
				measureOperation(`DKG-Comparison`, index, async () => {
					return await createDWallet(conf, networkDecryptionKeyPublicOutput);
				}),
			);
			const dkgResults = await Promise.allSettled(dkgPromises);
			const dkgMetrics = dkgResults
				.map((result, index) => {
					if (result.status === 'fulfilled') {
						allMetrics.push(result.value.metrics);
						return result.value;
					}
					throw new Error(`DKG ${index} failed`);
				})
				.filter(Boolean);

			// Test 2: Presign (using first successful DKG)
			if (dkgMetrics.length > 0) {
				console.log(`\n2️⃣ Testing ${testCount} Presign operations...`);
				await delay(checkpointCreationTime);
				const dwallet = dkgMetrics[0].result;

				const presignPromises = Array.from({ length: testCount }, (_, index) =>
					measureOperation(`Presign-Comparison`, index, async () => {
						return await presign(conf, dwallet.dwalletID);
					}),
				);
				const presignResults = await Promise.allSettled(presignPromises);
				const presignMetrics = presignResults
					.map((result, index) => {
						if (result.status === 'fulfilled') {
							allMetrics.push(result.value.metrics);
							return result.value;
						}
						return null;
					})
					.filter(Boolean);

				// Test 3: Sign (using first successful Presign)
				if (presignMetrics.length > 0) {
					console.log(`\n3️⃣ Testing ${testCount} Sign operations...`);
					await delay(checkpointCreationTime);
					const presign = presignMetrics[0]!.result;

					const signPromises = Array.from({ length: testCount }, (_, index) =>
						measureOperation(`Sign-Comparison`, index, async () => {
							const message = Buffer.from(`comparison test ${index}`);
							return await sign(
								conf,
								presign.id.id,
								dwallet.dwallet_cap_id,
								message,
								dwallet.secret_share,
								networkDecryptionKeyPublicOutput,
								Hash.KECCAK256,
							);
						}),
					);
					const signResults = await Promise.allSettled(signPromises);
					signResults.forEach((result, index) => {
						if (result.status === 'fulfilled') {
							allMetrics.push(result.value.metrics);
						}
					});
				}
			}

			// Print comparison summary
			console.log(`\n📊 COMPREHENSIVE PERFORMANCE COMPARISON:`);
			console.log(`=====================================`);

			const operationTypes = ['DKG-Comparison', 'Presign-Comparison', 'Sign-Comparison'];
			operationTypes.forEach((opType) => {
				const opMetrics = allMetrics.filter((m) => m.operation === opType && m.success);
				if (opMetrics.length > 0) {
					const avgDuration = opMetrics.reduce((a, b) => a + b.duration, 0) / opMetrics.length;
					const minDuration = Math.min(...opMetrics.map((m) => m.duration));
					const maxDuration = Math.max(...opMetrics.map((m) => m.duration));
					console.log(
						`${opType.replace('-Comparison', '').padEnd(8)}: avg=${avgDuration.toFixed(0)}ms, min=${minDuration}ms, max=${maxDuration}ms`,
					);
				}
			});

			expect(allMetrics.length).toBeGreaterThan(0);
		},
		DEFAULT_PARALLEL_CONFIG.timeout,
	);
});
