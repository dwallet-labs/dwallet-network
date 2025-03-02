;
// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';



import { delay, DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME, DWALLET_ECDSAK1_MOVE_MODULE_NAME, getDWalletSecpState, SUI_PACKAGE_ID } from './globals.js';
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
	const startSessionEvent = result.events?.at(0)?.parsedJson;
	if (!isStartSessionEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}

	const completedPresignEventType = `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::CompletedECDSAPresignEvent`;

	return await fetchCompletedEvent(
		conf,
		startSessionEvent.session_id,
		completedPresignEventType,
		isCompletedPresignEvent,
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

export async function fetchCompletedEvent<TEvent extends { session_id: string }>(
	c: Config,
	sessionID: string,
	eventType: string,
	isEventFn: (parsedJson: any) => parsedJson is TEvent,
): Promise<TEvent> {
	const startTime = Date.now();

	while (Date.now() - startTime <= c.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		const interval = 5_000;
		await delay(interval);

		const { data } = await c.client.queryEvents({
			query: {
				TimeRange: {
					startTime: (Date.now() - interval * 2).toString(),
					endTime: Date.now().toString(),
				},
			},
			limit: 1000,
		});

		const match = data.find(
			(event) =>
				event.type === eventType &&
				isEventFn(event.parsedJson) &&
				event.parsedJson.session_id === sessionID,
		);

		if (match) return match.parsedJson as TEvent;
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch an event of type ${eventType} within ${
			c.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

/**
 * Creates a valid mock output of the first DKG blockchain round.
 */
export async function mockCreatePresign(
	conf: Config,
	mockPresign: Uint8Array,
	dwalletID: string,
): Promise<CompletedPresignEvent> {
	const tx = new Transaction();
	const dwalletStateObjData = await getDWalletSecpState(conf);
	const stateArg = tx.sharedObjectRef({
		objectId: dwalletStateObjData.object_id,
		initialSharedVersion: dwalletStateObjData.initial_shared_version,
		mutable: true,
	});

// public fun mock_create_presign(self: &mut DWallet2PcMpcSecp256K1, presign: vector<u8>, dwallet_id: ID, ctx: &mut TxContext) {
	
	
	const firstRoundOutputArg = tx.pure(bcs.vector(bcs.u8()).serialize(mockPresign));
	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	const createdDWalletCap = result?.effects?.created?.find(
		(obj) =>
			isAddressObjectOwner(obj.owner) &&
			obj.owner.AddressOwner === conf.suiClientKeypair.toSuiAddress(),
	);
	if (!dwalletCap || createdDWalletCap === undefined) {
		throw new Error('Unable to create the DWallet cap');
	}
	await delay(checkpointCreationTime);
	const dwalletCapObj = await conf.client.getObject({
		id: createdDWalletCap.reference.objectId,
		options: { showContent: true },
	});
	const dwalletCapObjContent = dwalletCapObj?.data?.content;
	if (!isMoveObject(dwalletCapObjContent)) {
		throw new Error('Invalid DWallet cap object');
	}
	const dwalletCapFields = dwalletCapObjContent.fields;
	if (!isDWalletCap(dwalletCapFields)) {
		throw new Error('Invalid DWallet cap fields');
	}

	return {
		dwalletCapID: createdDWalletCap.reference.objectId,
		dwalletID: dwalletCapFields.dwallet_id,
		sessionID: '',
		output: mockPresign,
	};
}
