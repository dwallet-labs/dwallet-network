;
// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Transaction } from '@mysten/sui/transactions';



import { Config, DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME, DWALLET_ECDSAK1_MOVE_MODULE_NAME, fetchCompletedEvent, getDWalletSecpState, isStartSessionEvent, SUI_PACKAGE_ID } from './globals.ts';


interface CompletedPresignEvent {
	presign_id: string;
	session_id: string;
}

export async function presign(conf: Config, dwallet_id: string): Promise<CompletedPresignEvent> {
	const tx = new Transaction();
	let emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
	let dWalletStateData = await getDWalletSecpState(conf);

	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::request_ecdsa_presign`,
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
	let startSessionEvent = result.events?.at(0)?.parsedJson;
	if (!isStartSessionEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}

	let completedPresignEventType = `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::CompletedECDSAPresignEvent`;

	return await fetchCompletedEvent(
		conf,
		startSessionEvent.session_id,
		completedPresignEventType,
		isCompletedPresignEvent,
	);
}

function isCompletedPresignEvent(event: any): event is CompletedPresignEvent {
	return event.presign_id !== undefined;
}
