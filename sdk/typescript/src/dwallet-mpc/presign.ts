// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Transaction } from '@mysten/sui/transactions';

import {
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	// getDWalletSecpState, // No longer directly used
	executeTransactionAndGetMainEvent,
	getDWalletStateArg,
	getObjectWithType,
	handleIKACoin,
} from './globals.js';
import type { Config } from './globals.ts';

interface CompletedPresign {
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

export async function presign(conf: Config, dwallet_id: string): Promise<CompletedPresign> {
	const tx = new Transaction();
	const emptyIKACoin = handleIKACoin(tx, conf);
	// const dWalletStateData = await getDWalletSecpState(conf); // No longer needed
	const dwalletStateArg = await getDWalletStateArg(conf, tx, true);

	const presignCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_presign`,
		arguments: [
			dwalletStateArg,
			tx.pure.id(dwallet_id),
			tx.pure.u32(0),
			emptyIKACoin,
			tx.gas,
		],
	});

	tx.transferObjects([presignCap], conf.suiClientKeypair.toSuiAddress());

	const startSessionEvent = await executeTransactionAndGetMainEvent<StartPresignEvent>(
		conf,
		tx,
		isStartPresignEvent,
		'Presign failed',
	);

	return await getObjectWithType(conf, startSessionEvent.event_data.presign_id, isCompletedPresign);
}

function isStartPresignEvent(event: any): event is StartPresignEvent {
	return event.event_data !== undefined && event.event_data.presign_id !== undefined;
}
