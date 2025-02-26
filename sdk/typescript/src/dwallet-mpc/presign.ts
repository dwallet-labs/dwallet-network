// public fun request_ecdsa_presign(
//     self: &mut DWallet2PcMpcSecp256K1,
//     dwallet_id: ID,
//     payment_ika: &mut Coin<IKA>,
//     payment_sui: &mut Coin<SUI>,
//     ctx: &mut TxContext
// ) {

import { Transaction } from '@mysten/sui/transactions';

import {
	Config,
	delay,
	DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	getDWalletSecpState,
	SUI_PACKAGE_ID,
} from './globals.ts';

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
	let startDKGEvent = result.events?.at(0)?.parsedJson;
	if (!isStartSessionEvent(startDKGEvent)) {
		throw new Error('invalid start session event');
	}

	let completedPresignEventType = `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::CompletedECDSAPresignEvent`;

	return await fetchCompletedEvent(
		conf,
		startDKGEvent.session_id,
		completedPresignEventType,
		isCompletedPresignEvent,
	);
}

function isCompletedPresignEvent(event: any): event is CompletedPresignEvent {
	return event.presign_id !== undefined;
}

interface CompletedPresignEvent {
	presign_id: string;
	session_id: string;
}

interface StartSessionEvent {
	session_id: string;
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
	let cursor = null;

	while (Date.now() - startTime <= c.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		await delay(5_000);

		const { data, nextCursor, hasNextPage } = await c.client.queryEvents({
			query: {
				TimeRange: {
					startTime: (Date.now() - c.timeout).toString(),
					endTime: Date.now().toString(),
				},
			},
			cursor,
		});

		const match = data.find(
			(event) =>
				event.type === eventType &&
				isEventFn(event.parsedJson) &&
				event.parsedJson.session_id === sessionID,
		);

		if (match) return match.parsedJson as TEvent;
		if (hasNextPage) cursor = nextCursor;
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch an event of type ${eventType} within ${
			c.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}
