// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { SuiTransactionBlockResponse } from '@mysten/sui/client';
import { Transaction } from '@mysten/sui/transactions';

import {
	Config,
	createSessionIdentifier,
	DWALLET_COORDINATOR_INNER_MOVE_MODULE_NAME,
	getDWalletSecpState,
	SharedObjectData,
	SUI_PACKAGE_ID,
} from './globals.js';

/**
 * Creates an empty IKA coin for transaction gas
 */
export function createEmptyIKACoin(tx: Transaction, conf: Config) {
	return tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
}

/**
 * Destroys an empty IKA coin after use
 */
export function destroyEmptyIKACoin(tx: Transaction, emptyIKACoin: any, conf: Config) {
	return tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::destroy_zero`,
		arguments: [emptyIKACoin],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
}

/**
 * Creates a shared object reference argument for dwallet state
 */
export async function createDWalletStateArg(tx: Transaction, conf: Config, mutable = true) {
	const dWalletStateData = await getDWalletSecpState(conf);

	return tx.sharedObjectRef({
		objectId: dWalletStateData.object_id,
		initialSharedVersion: dWalletStateData.initial_shared_version,
		mutable,
	});
}

/**
 * Generic transaction execution with timing and logging
 */
export async function executeTransactionWithTiming<T>(
	conf: Config,
	tx: Transaction,
	operationName: string,
	extractResult?: (result: SuiTransactionBlockResponse) => T,
): Promise<T> {
	const address = conf.suiClientKeypair.toSuiAddress();
	console.time(`${operationName}: ${address}`);

	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});

	if (result.errors !== undefined) {
		throw new Error(`${operationName} failed with errors: ${result.errors}`);
	}

	console.timeEnd(`${operationName}: ${address}`);

	if (extractResult) {
		return extractResult(result);
	}

	return result as T;
}

/**
 * Generic transaction execution with retry logic
 */
export async function executeTransactionWithRetry<T>(
	conf: Config,
	tx: Transaction,
	operationName: string,
	extractResult?: (result: SuiTransactionBlockResponse) => T,
): Promise<T> {
	const { delay } = await import('./globals.js');
	const startTime = Date.now();

	while (Date.now() - startTime <= conf.timeout) {
		try {
			return await executeTransactionWithTiming(conf, tx, operationName, extractResult);
		} catch (error) {
			// If we're still within timeout, wait a bit and retry
			if (Date.now() - startTime <= conf.timeout) {
				await delay(5_000); // Wait 5 seconds before retrying
				continue;
			}
			throw error; // If we've exceeded timeout, throw the error
		}
	}

	throw new Error(
		`Failed to execute ${operationName} within ${conf.timeout / (60 * 1000)} minutes`,
	);
}

/**
 * Common pattern for creating a transaction with empty IKA coin and session identifier
 */
export async function createBaseTransaction(conf: Config): Promise<{
	tx: Transaction;
	emptyIKACoin: any;
	dwalletStateArg: any;
	sessionIdentifier: any;
}> {
	const tx = new Transaction();
	const emptyIKACoin = createEmptyIKACoin(tx, conf);
	const dwalletStateArg = await createDWalletStateArg(tx, conf);
	const sessionIdentifier = await createSessionIdentifier(
		tx,
		dwalletStateArg,
		conf.ikaConfig.ika_system_package_id,
	);

	return { tx, emptyIKACoin, dwalletStateArg, sessionIdentifier };
}

/**
 * Type guard utility for checking if an object has a specific property
 */
export function hasProperty<T extends string>(obj: any, prop: T): obj is Record<T, any> {
	return obj && typeof obj === 'object' && prop in obj;
}

/**
 * Type guard utility for checking if an object has nested properties
 */
export function hasNestedProperty(obj: any, ...props: string[]): boolean {
	let current = obj;
	for (const prop of props) {
		if (!current || typeof current !== 'object' || !(prop in current)) {
			return false;
		}
		current = current[prop];
	}
	return true;
}

/**
 * Type guard utility for checking if an object has an event_data property with specific fields
 */
export function hasEventData(obj: any, requiredFields: string[]): boolean {
	return (
		hasProperty(obj, 'event_data') &&
		requiredFields.every((field) => hasProperty(obj.event_data, field))
	);
}
