// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Transaction } from '@mysten/sui/transactions';

import {
	createSessionIdentifier,
	DWALLET_COORDINATOR_MOVE_MODULE_NAME,
	getDWalletSecpState,
	getObjectWithType,
	SUI_PACKAGE_ID,
} from './globals.js';
import type { Config } from './globals.ts';

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
	const tx = new Transaction();
	const emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
	const dWalletStateData = await getDWalletSecpState(conf);
	const dwalletStateArg = tx.sharedObjectRef({
		objectId: dWalletStateData.object_id,
		initialSharedVersion: dWalletStateData.initial_shared_version,
		mutable: true,
	});
	const sessionIdentifier = await createSessionIdentifier(
		tx,
		dwalletStateArg,
		conf.ikaConfig.ika_dwallet_2pc_mpc_package_id,
	);
	const presignCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_dwallet_2pc_mpc_package_id}::${DWALLET_COORDINATOR_MOVE_MODULE_NAME}::request_presign`,
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

	tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::destroy_zero`,
		arguments: [emptyIKACoin],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
	return tx;
}

export async function executePresignTransaction(
	conf: Config,
	tx: Transaction,
): Promise<CompletedPresign> {
	console.time(`Presign: ${conf.suiClientKeypair.getPublicKey().toSuiAddress()}`);
	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	const startSessionEvent = result.events?.at(1)?.parsedJson;
	if (!isStartPresignEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}
	const completedPresign = await getObjectWithType(
		conf,
		startSessionEvent.event_data.presign_id,
		isCompletedPresign,
	);
	console.timeEnd(`Presign: ${conf.suiClientKeypair.getPublicKey().toSuiAddress()}`);
	console.log(
		`Presign: ${conf.suiClientKeypair.getPublicKey().toSuiAddress()} - ${startSessionEvent.event_data.presign_id}`,
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
