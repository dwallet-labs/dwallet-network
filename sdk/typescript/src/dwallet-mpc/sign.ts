import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import {
	DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	fetchCompletedEvent,
	fetchObjectWithType,
	getDWalletSecpState,
	getObjectWithType,
	isActiveDWallet,
	isDWalletCap,
	isPresign,
	isStartSessionEvent,
	MPCKeyScheme,
	SUI_PACKAGE_ID,
} from './globals.js';
import type { Config } from './globals.ts';

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

interface CompletedFutureSignEvent {
	session_id: string;
	dwallet_id: string;
	partial_centralized_signed_message_id: string;
}

function isCompletedSignEvent(obj: any): obj is CompletedSignEvent {
	return (
		obj && 'session_id' in obj && 'sign_id' in obj && 'signature' in obj && 'is_future_sign' in obj
	);
}

function isCompletedFutureSignEvent(obj: any): obj is CompletedFutureSignEvent {
	return (
		obj &&
		'session_id' in obj &&
		'dwallet_id' in obj &&
		'partial_centralized_signed_message_id' in obj
	);
}

export async function sign(
	conf: Config,
	presignID: string,
	dwalletCapID: string,
	message: Uint8Array,
	secretKey: Uint8Array,
	networkDecryptionKeyPublicOutput: Uint8Array,
	hash = Hash.KECCAK256,
): Promise<ReadySignObject> {
	const dwalletCap = await getObjectWithType(conf, dwalletCapID, isDWalletCap);
	const dwalletID = dwalletCap.dwallet_id;
	const activeDWallet = await getObjectWithType(conf, dwalletID, isActiveDWallet);
	const presign = await getObjectWithType(conf, presignID, isPresign);

	const centralizedSignedMessage = create_sign_centralized_output(
		networkDecryptionKeyPublicOutput,
		MPCKeyScheme.Secp256k1,
		activeDWallet.state.fields.public_output,
		secretKey,
		presign.state.fields.presign,
		message,
		hash,
	);
	const dWalletStateData = await getDWalletSecpState(conf);
	const tx = new Transaction();
	const messageApproval = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME}::approve_message`,
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
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_ecdsa_sign`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.object(presign.cap_id),
			messageApproval,
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedSignedMessage)),
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
	if (!isStartSignEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}
	return await getObjectWithType(conf, startSessionEvent.event_data.sign_id, isReadySignObject);
}

interface ReadySignObject {
	state: {
		fields: {
			signature: Uint8Array;
		};
	};
}

function isReadySignObject(obj: any): obj is ReadySignObject {
	return (
		obj?.state !== undefined &&
		obj.state.fields !== undefined &&
		obj.state.fields.signature !== undefined
	);
}

interface StartSignEvent {
	event_data: {
		sign_id: string;
	};
}

function isStartSignEvent(event: any): event is StartSignEvent {
	return event.event_data !== undefined && event.event_data.sign_id !== undefined;
}

interface StartFutureSignEvent {
	event_data: {
		partial_centralized_signed_message_id: string;
	};
}

function isStartFutureSignEvent(event: any): event is StartFutureSignEvent {
	return (
		event.event_data !== undefined &&
		event.event_data.partial_centralized_signed_message_id !== undefined
	);
}

export async function createUnverifiedECDSAPartialUserSignatureCap(
	conf: Config,
	presignID: string,
	dwalletCapID: string,
	message: Uint8Array,
	secretKey: Uint8Array,
	networkDecryptionKeyPublicOutput: Uint8Array,
	hash = Hash.KECCAK256,
): Promise<string> {
	const dwalletCap = await getObjectWithType(conf, dwalletCapID, isDWalletCap);
	const dwalletID = dwalletCap.dwallet_id;
	const activeDWallet = await getObjectWithType(conf, dwalletID, isActiveDWallet);
	const presign = await getObjectWithType(conf, presignID, isPresign);
	const dWalletStateData = await getDWalletSecpState(conf);
	const tx = new Transaction();

	const centralizedSignedMessage = create_sign_centralized_output(
		networkDecryptionKeyPublicOutput,
		MPCKeyScheme.Secp256k1,
		activeDWallet.state.fields.public_output,
		secretKey,
		presign.state.fields.presign,
		message,
		hash,
	);

	const emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});

	const [unverifiedECDSAPartialUserSignatureCap] = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_ecdsa_future_sign`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.object(presign.cap_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(message)),
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedSignedMessage)),
			emptyIKACoin,
			tx.gas,
		],
	});
	tx.transferObjects(
		[unverifiedECDSAPartialUserSignatureCap],
		conf.suiClientKeypair.toSuiAddress(),
	);
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
			showObjectChanges: true,
		},
	});
	const startSessionEvent = result.events?.at(0)?.parsedJson;
	if (!isStartFutureSignEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}

	const partialSignature = await getObjectWithType(
		conf,
		startSessionEvent.event_data.partial_centralized_signed_message_id,
		isVerifiedECDSAPartialUserSignature,
	);
	return partialSignature.cap_id;
}

interface VerifiedECDSAPartialUserSignature {
	state: {
		variant: 'NetworkVerificationCompleted';
	};
	cap_id: string;
}

function isVerifiedECDSAPartialUserSignature(obj: any): obj is VerifiedECDSAPartialUserSignature {
	return (
		obj &&
		'state' in obj &&
		'variant' in obj.state &&
		obj.state.variant === 'NetworkVerificationCompleted'
	);
}

export async function verifyECFSASignWithPartialUserSignatures(
	conf: Config,
	unverifiedECDSAPartialUserSignatureCapID: string,
): Promise<string> {
	const dWalletStateData = await getDWalletSecpState(conf);
	const tx = new Transaction();

	const [verifiedECDSAPartialUserSignatureCap] = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::verify_ecdsa_partial_user_signature_cap`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.object(unverifiedECDSAPartialUserSignatureCapID),
		],
	});
	tx.transferObjects([verifiedECDSAPartialUserSignatureCap], conf.suiClientKeypair.toSuiAddress());

	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
			showObjectChanges: true,
		},
	});
	const objects = result.objectChanges!;
	if (!objects) {
		throw new Error('no objects created during verify_ecdsa_partial_user_signature_cap call');
	}
	for (const obj of objects) {
		if (
			obj &&
			'objectType' in obj &&
			obj.objectType! ===
				`${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME}::VerifiedECDSAPartialUserSignatureCap`
		) {
			return obj.objectId;
		}
	}
	throw new Error('no verified object created');
}

export async function completeFutureSign(
	conf: Config,
	dwalletCapID: string,
	message: Uint8Array,
	hash = Hash.KECCAK256,
	verifyECDSAPartialUserSignatureCapID: string,
): Promise<ReadySignObject> {
	const dWalletStateData = await getDWalletSecpState(conf);
	const tx = new Transaction();

	const messageApproval = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME}::approve_message`,
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
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_ecdsa_sign_with_partial_user_signatures`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.object(verifyECDSAPartialUserSignatureCapID),
			messageApproval,
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
	if (!isStartSignEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}
	return await getObjectWithType(conf, startSessionEvent.event_data.sign_id, isReadySignObject);
}
