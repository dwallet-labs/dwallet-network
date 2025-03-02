import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import {Config, mockedProtocolPublicParameters, MPCKeyScheme} from './globals.ts';
import {
	DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	fetchCompletedEvent,
	getDWalletSecpState,
	isStartSessionEvent,
	SUI_PACKAGE_ID,
} from './globals.ts';

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

interface CompletedSignEvent {
	session_id: string;
	sign_id: string;
	signature: Uint8Array;
	is_future_sign: boolean;
}

function isCompletedSignEvent(obj: any): obj is CompletedSignEvent {
	return (
		obj && 'session_id' in obj && 'sign_id' in obj && 'signature' in obj && 'is_future_sign' in obj
	);
}

export async function sign(
	conf: Config,
	presignID: string,
	dwalletCapID: string,
	message: Uint8Array,
	hash = Hash.KECCAK256,
	protocolPublicParameters: Uint8Array = mockedProtocolPublicParameters,
): Promise<CompletedSignEvent> {
	let partialSignatures = create_sign_centralized_output(
		protocolPublicParameters,
		MPCKeyScheme.Secp256k1,


	);
	// TODO: replace with mock
	const centralizedSignedMessage = new Uint8Array();
	const dWalletStateData = await getDWalletSecpState(conf);
	const tx = new Transaction();
	const messageApproval = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::approve_message`,
		arguments: [
			tx.object(dwalletCapID),
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.pure(bcs.vector(bcs.u8()).serialize(message)),
		],
	});
	const emptyIKACoin = tx.moveCall({
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
	const startSessionEvent = result.events?.at(0)?.parsedJson;
	if (!isStartSessionEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}
	const completedSignEventType = `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::CompletedECDSASignEvent`;
	return await fetchCompletedEvent(
		conf,
		startSessionEvent.session_id,
		completedSignEventType,
		isCompletedSignEvent,
	);
}
