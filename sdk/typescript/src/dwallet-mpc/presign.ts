// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Transaction } from '@mysten/sui/transactions';

import {
	DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	fetchCompletedEvent,
	getDWalletSecpState,
	SUI_PACKAGE_ID,
} from './globals.js';
import type { Config } from './globals.ts';

interface CompletedPresignEvent {
	presign_id: string;
	session_id: string;
	presign: Uint8Array;
}

interface StartSessionEvent {
	session_id: string;
}

export async function presign(conf: Config, dwallet_id: string): Promise<CompletedPresignEvent> {
	const tx = new Transaction();
	const emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
	const dWalletStateData = await getDWalletSecpState(conf);

	const presignCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_ecdsa_presign`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.pure.id(dwallet_id),
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

	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	const startSessionEvent = result.events?.at(0)?.parsedJson;
	if (!isStartSessionEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}

	const completedPresignEventType = `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME}::CompletedECDSAPresignEvent`;

	return await fetchCompletedEvent(
		conf,
		startSessionEvent.session_id,
		isCompletedPresignEvent,
		completedPresignEventType,
	);
}

function isCompletedPresignEvent(event: any): event is CompletedPresignEvent {
	return (
		event.presign_id !== undefined && event.presign !== undefined && event.session_id !== undefined
	);
}

function isStartSessionEvent(event: any): event is StartSessionEvent {
	return event.session_id !== undefined;
}
