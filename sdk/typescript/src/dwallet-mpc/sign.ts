import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import {
	Config,
	DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	getDWalletSecpState,
	SUI_PACKAGE_ID,
} from './globals.ts';

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export async function sign(
	conf: Config,
	presignID: string,
	dwalletID: string,
	dwalletCapID: string,
	message: Uint8Array,
	hash = Hash.KECCAK256,
) {
	// TODO: replace with mock
	let centralizedSignedMessage = new Uint8Array();
	let dWalletStateData = await getDWalletSecpState(conf);
	let tx = new Transaction();
	const messageApproval = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::approve_message`,
		arguments: [
			tx.object(dwalletCapID),
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.pure(bcs.vector(bcs.u8()).serialize(message)),
		],
	});
	let emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});

	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::request_ecdsa_sign`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.pure.id(dwalletID),
			messageApproval,
			tx.pure.id(presignID),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedSignedMessage)),
			emptyIKACoin,
			tx.gas,
		],
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
}
