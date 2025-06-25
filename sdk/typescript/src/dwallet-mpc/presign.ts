// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { Transaction } from '@mysten/sui/transactions';

import { DWALLET_COORDINATOR_MOVE_MODULE_NAME, getObjectWithType } from './globals.js';
import type { Config } from './globals.ts';
import {
	createBaseTransaction,
	destroyEmptyIKACoin,
	executeTransactionWithTiming,
} from './transaction-utils.js';

export interface CompletedPresign {
	state: {
		fields: {
			presign: Uint8Array;
		};
	};
	id: { id: string };
}

interface StartPresignEvent {
	event_data: {
		presign_id: string;
	};
}

function isCompletedPresign(event: any): event is CompletedPresign {
	return (
		event.state !== undefined &&
		event.state.fields !== undefined &&
		event.state.fields.presign !== undefined
	);
}

export async function preparePresignTransaction(
	conf: Config,
	dwallet_id: string,
): Promise<Transaction> {
	const { tx, emptyIKACoin, dwalletStateArg, sessionIdentifier } =
		await createBaseTransaction(conf);

	const presignCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_COORDINATOR_MOVE_MODULE_NAME}::request_presign`,
		arguments: [
			dwalletStateArg,
			tx.pure.id(dwallet_id),
			tx.pure.u32(0),
			sessionIdentifier,
			emptyIKACoin,
			tx.gas,
		],
	});

	tx.transferObjects([presignCap], conf.suiClientKeypair.toSuiAddress());
	destroyEmptyIKACoin(tx, emptyIKACoin, conf);

	return tx;
}

export async function executePresignTransaction(
	conf: Config,
	tx: Transaction,
): Promise<CompletedPresign> {
	const extractPresignResult = (result: any) => {
		const startSessionEvent = result.events?.at(1)?.parsedJson;
		if (!isStartPresignEvent(startSessionEvent)) {
			throw new Error('invalid start session event');
		}
		return getObjectWithType(conf, startSessionEvent.event_data.presign_id, isCompletedPresign);
	};

	const completedPresign = await executeTransactionWithTiming(
		conf,
		tx,
		'Presign',
		extractPresignResult,
	);

	console.log(
		`Presign: ${conf.suiClientKeypair.getPublicKey().toSuiAddress()} - ${completedPresign.id.id}`,
	);

	return completedPresign;
}

export async function presign(conf: Config, dwallet_id: string): Promise<CompletedPresign> {
	const tx = await preparePresignTransaction(conf, dwallet_id);
	return executePresignTransaction(conf, tx);
}

function isStartPresignEvent(event: any): event is StartPresignEvent {
	return event.event_data !== undefined && event.event_data.presign_id !== undefined;
}
