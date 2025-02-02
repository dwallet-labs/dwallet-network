// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { resolve } from 'path';
import { fromHex, toBase64 } from '@mysten/bcs';
import { beforeAll, describe, expect, it } from 'vitest';

import { bcs } from '../../src/bcs';
import { Ed25519Keypair } from '../../src/keypairs/ed25519';
import { Transaction } from '../../src/transactions';
import { coinWithBalance } from '../../src/transactions/intents/CoinWithBalance';
import { normalizeIkaAddress } from '../../src/utils';
import { setup, TestToolbox } from './utils/setup';

describe('coinWithBalance', () => {
	let toolbox: TestToolbox;
	let publishToolbox: TestToolbox;
	let packageId: string;
	let testType: string;
	let testTypeZero: string;

	beforeAll(async () => {
		[toolbox, publishToolbox] = await Promise.all([setup(), setup()]);
		const packagePath = resolve(__dirname, './data/coin_metadata');
		packageId = await publishToolbox.getPackage(packagePath);
		testType = normalizeIkaAddress(packageId) + '::test::TEST';
		testTypeZero = normalizeIkaAddress(packageId) + '::test_zero::TEST_ZERO';
	});

	it('works with ika', async () => {
		const tx = new Transaction();
		const receiver = new Ed25519Keypair();

		tx.transferObjects(
			[
				coinWithBalance({
					type: 'gas',
					balance: 12345n,
				}),
			],
			receiver.toIkaAddress(),
		);
		tx.setSender(publishToolbox.keypair.toIkaAddress());

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: ['CoinWithBalance'],
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					$Intent: {
						data: {
							balance: '12345',
							type: 'gas',
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					TransferObjects: {
						objects: [
							{
								Result: 0,
							},
						],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: [],
					client: toolbox.client,
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
				{
					Pure: {
						bytes: toBase64(bcs.u64().serialize(12345).toBytes()),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					SplitCoins: {
						coin: {
							GasCoin: true,
						},
						amounts: [
							{
								Input: 1,
							},
						],
					},
				},
				{
					TransferObjects: {
						objects: [
							{
								NestedResult: [0, 0],
							},
						],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		const { digest } = await toolbox.client.signAndExecuteTransaction({
			transaction: tx,
			signer: publishToolbox.keypair,
		});

		const result = await toolbox.client.waitForTransaction({
			digest,
			options: { showEffects: true, showBalanceChanges: true },
		});

		expect(result.effects?.status.status).toBe('success');
		expect(
			result.balanceChanges?.find(
				(change) =>
					typeof change.owner === 'object' &&
					'AddressOwner' in change.owner &&
					change.owner.AddressOwner === receiver.toIkaAddress(),
			),
		).toEqual({
			amount: '12345',
			coinType: '0x2::ika::IKA',
			owner: {
				AddressOwner: receiver.toIkaAddress(),
			},
		});
	});

	it('works with custom coin', async () => {
		const tx = new Transaction();
		const receiver = new Ed25519Keypair();

		tx.transferObjects(
			[
				coinWithBalance({
					type: testType,
					balance: 1n,
				}),
			],
			receiver.toIkaAddress(),
		);
		tx.setSender(publishToolbox.keypair.toIkaAddress());

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: ['CoinWithBalance'],
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					$Intent: {
						data: {
							balance: '1',
							type: testType,
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					TransferObjects: {
						objects: [
							{
								Result: 0,
							},
						],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: [],
					client: publishToolbox.client,
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
				{
					Object: {
						ImmOrOwnedObject: expect.anything(),
					},
				},
				{
					Pure: {
						bytes: toBase64(bcs.u64().serialize(1).toBytes()),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					SplitCoins: {
						coin: {
							Input: 1,
						},
						amounts: [
							{
								Input: 2,
							},
						],
					},
				},
				{
					TransferObjects: {
						objects: [{ NestedResult: [0, 0] }],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		const { digest } = await toolbox.client.signAndExecuteTransaction({
			transaction: tx,
			signer: publishToolbox.keypair,
		});

		const result = await toolbox.client.waitForTransaction({
			digest,
			options: { showEffects: true, showBalanceChanges: true },
		});

		expect(result.effects?.status.status).toBe('success');
		expect(
			result.balanceChanges?.find(
				(change) =>
					typeof change.owner === 'object' &&
					'AddressOwner' in change.owner &&
					change.owner.AddressOwner === receiver.toIkaAddress(),
			),
		).toEqual({
			amount: '1',
			coinType: testType,
			owner: {
				AddressOwner: receiver.toIkaAddress(),
			},
		});
	});

	it('works with zero balance coin', async () => {
		const tx = new Transaction();
		const receiver = new Ed25519Keypair();

		tx.transferObjects(
			[
				coinWithBalance({
					type: testTypeZero,
					balance: 0n,
				}),
			],
			receiver.toIkaAddress(),
		);
		tx.setSender(publishToolbox.keypair.toIkaAddress());

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: ['CoinWithBalance'],
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					$Intent: {
						data: {
							balance: '0',
							type: testTypeZero,
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					TransferObjects: {
						objects: [
							{
								Result: 0,
							},
						],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: [],
					client: publishToolbox.client,
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					MoveCall: {
						arguments: [],
						function: 'zero',
						module: 'coin',
						package: '0x0000000000000000000000000000000000000000000000000000000000000002',
						typeArguments: [testTypeZero],
					},
				},
				{
					TransferObjects: {
						objects: [{ Result: 0 }],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		const { digest } = await toolbox.client.signAndExecuteTransaction({
			transaction: tx,
			signer: publishToolbox.keypair,
		});

		const result = await toolbox.client.waitForTransaction({
			digest,
			options: { showEffects: true, showBalanceChanges: true, showObjectChanges: true },
		});

		expect(result.effects?.status.status).toBe('success');
		expect(
			result.objectChanges?.filter((change) => {
				if (change.type !== 'created') return false;
				if (typeof change.owner !== 'object' || !('AddressOwner' in change.owner)) return false;

				return (
					change.objectType === `0x2::coin::Coin<${testTypeZero}>` &&
					change.owner.AddressOwner === receiver.toIkaAddress()
				);
			}).length,
		).toEqual(1);
	});

	it('works with multiple coins', async () => {
		const tx = new Transaction();
		const receiver = new Ed25519Keypair();

		tx.transferObjects(
			[
				coinWithBalance({ type: testType, balance: 1n }),
				coinWithBalance({ type: testType, balance: 2n }),
				coinWithBalance({ type: 'gas', balance: 3n }),
				coinWithBalance({ type: 'gas', balance: 4n }),
				coinWithBalance({ type: testTypeZero, balance: 0n }),
			],
			receiver.toIkaAddress(),
		);

		tx.setSender(publishToolbox.keypair.toIkaAddress());

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: ['CoinWithBalance'],
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					$Intent: {
						data: {
							balance: '1',
							type: testType,
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					$Intent: {
						data: {
							balance: '2',
							type: testType,
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					$Intent: {
						data: {
							balance: '3',
							type: 'gas',
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					$Intent: {
						data: {
							balance: '4',
							type: 'gas',
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					$Intent: {
						data: {
							balance: '0',
							type: testTypeZero,
						},
						inputs: {},
						name: 'CoinWithBalance',
					},
				},
				{
					TransferObjects: {
						objects: [
							{
								Result: 0,
							},
							{
								Result: 1,
							},
							{
								Result: 2,
							},
							{
								Result: 3,
							},
							{
								Result: 4,
							},
						],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		expect(
			JSON.parse(
				await tx.toJSON({
					supportedIntents: [],
					client: publishToolbox.client,
				}),
			),
		).toEqual({
			expiration: null,
			gasData: {
				budget: null,
				owner: null,
				payment: null,
				price: null,
			},
			inputs: [
				{
					Pure: {
						bytes: toBase64(fromHex(receiver.toIkaAddress())),
					},
				},
				{
					Object: {
						ImmOrOwnedObject: expect.anything(),
					},
				},
				{
					Pure: {
						bytes: toBase64(bcs.u64().serialize(1).toBytes()),
					},
				},
				{
					Pure: {
						bytes: toBase64(bcs.u64().serialize(2).toBytes()),
					},
				},
				{
					Pure: {
						bytes: toBase64(bcs.u64().serialize(3).toBytes()),
					},
				},
				{
					Pure: {
						bytes: toBase64(bcs.u64().serialize(4).toBytes()),
					},
				},
			],
			sender: publishToolbox.keypair.toIkaAddress(),
			commands: [
				{
					SplitCoins: {
						coin: {
							Input: 1,
						},
						amounts: [
							{
								Input: 2,
							},
						],
					},
				},
				{
					SplitCoins: {
						coin: {
							Input: 1,
						},
						amounts: [
							{
								Input: 3,
							},
						],
					},
				},
				{
					SplitCoins: {
						coin: {
							GasCoin: true,
						},
						amounts: [
							{
								Input: 4,
							},
						],
					},
				},
				{
					SplitCoins: {
						coin: {
							GasCoin: true,
						},
						amounts: [
							{
								Input: 5,
							},
						],
					},
				},
				{
					MoveCall: {
						arguments: [],
						function: 'zero',
						module: 'coin',
						package: '0x0000000000000000000000000000000000000000000000000000000000000002',
						typeArguments: [testTypeZero],
					},
				},
				{
					TransferObjects: {
						objects: [
							{ NestedResult: [0, 0] },
							{ NestedResult: [1, 0] },
							{ NestedResult: [2, 0] },
							{ NestedResult: [3, 0] },
							{ Result: 4 },
						],
						address: {
							Input: 0,
						},
					},
				},
			],
			version: 2,
		});

		const { digest } = await toolbox.client.signAndExecuteTransaction({
			transaction: tx,
			signer: publishToolbox.keypair,
		});

		const result = await toolbox.client.waitForTransaction({
			digest,
			options: { showEffects: true, showBalanceChanges: true, showObjectChanges: true },
		});

		expect(result.effects?.status.status).toBe('success');
		expect(
			result.balanceChanges?.filter(
				(change) =>
					typeof change.owner === 'object' &&
					'AddressOwner' in change.owner &&
					change.owner.AddressOwner === receiver.toIkaAddress(),
			),
		).toEqual([
			{
				amount: '7',
				coinType: '0x2::ika::IKA',
				owner: {
					AddressOwner: receiver.toIkaAddress(),
				},
			},
			{
				amount: '3',
				coinType: testType,
				owner: {
					AddressOwner: receiver.toIkaAddress(),
				},
			},
		]);
		expect(
			result.objectChanges?.filter((change) => {
				if (change.type !== 'created') return false;
				if (typeof change.owner !== 'object' || !('AddressOwner' in change.owner)) return false;

				return (
					change.objectType === `0x2::coin::Coin<${testTypeZero}>` &&
					change.owner.AddressOwner === receiver.toIkaAddress()
				);
			}).length,
		).toEqual(1);
	});
});
