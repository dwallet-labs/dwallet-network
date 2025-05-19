import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import {
	Config,
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	getDwalletSecp256k1ObjID,
	getInitialSharedVersion,
	getNetworkDecryptionKeyID,
	getObjectWithType,
	isActiveDWallet,
	SUI_PACKAGE_ID,
	type SharedObjectData,
} from './globals';

interface NewImportedKeyDWalletEvent {
	dwallet_id: string;
	dwallet_cap_id: string;
}

function isNewImportedKeyDWalletEvent(event: any): event is NewImportedKeyDWalletEvent {
	return event.dwallet_id !== undefined && event.dwallet_cap_id !== undefined;
}

/**
 * Create an imported dWallet & return the dWallet ID.
 */
export async function createImportedDWallet(conf: Config): Promise<NewImportedKeyDWalletEvent> {
	const tx = new Transaction();
	const networkDecryptionKeyID = await getNetworkDecryptionKeyID(conf);
	const dwalletSecp256k1ID = await getDwalletSecp256k1ObjID(conf);
	const dwalletCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::new_imported_key_dwallet`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dwalletSecp256k1ID,
				initialSharedVersion: await getInitialSharedVersion(conf, dwalletSecp256k1ID),
				mutable: true,
			}),
			tx.pure.id(networkDecryptionKeyID),
			tx.pure.u32(0),
		],
	});
	tx.transferObjects([dwalletCap], conf.suiClientKeypair.toSuiAddress());
	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	const creationEvent = result.events?.at(0)?.parsedJson;
	if (!isNewImportedKeyDWalletEvent(creationEvent)) {
		throw new Error('Failed to create imported dWallet');
	}
	return creationEvent;
}

export async function verifyImportedDWalletMoveCall(
	conf: Config,
	dWalletStateData: SharedObjectData,
	dwalletCapID: string,
	centralized_party_message: Uint8Array,
	encrypted_centralized_secret_share_and_proof: Uint8Array,
	user_public_output: Uint8Array,
	dwalletID: string,
): Promise<string> {
	const tx = new Transaction();
	const dwalletStateArg = tx.sharedObjectRef({
		objectId: dWalletStateData.object_id,
		initialSharedVersion: dWalletStateData.initial_shared_version,
		mutable: true,
	});
	const dwalletCapArg = tx.object(dwalletCapID);
	const centralizedPublicOutputArg = tx.pure(bcs.vector(bcs.u8()).serialize(user_public_output));
	const encryptedCentralizedSecretShareAndProofArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(encrypted_centralized_secret_share_and_proof),
	);
	const encryptionKeyAddressArg = tx.pure.id(
		conf.encryptedSecretShareSigningKeypair.toSuiAddress(),
	);
	const centralizedPartyMessageArg = tx.pure(
		bcs.vector(bcs.u8()).serialize(centralized_party_message),
	);
	const signerPublicKeyArg = tx.pure(
		bcs
			.vector(bcs.u8())
			.serialize(conf.encryptedSecretShareSigningKeypair.getPublicKey().toRawBytes()),
	);

	const emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_imported_key_dwallet_verification`,
		arguments: [
			dwalletStateArg,
			dwalletCapArg,
			centralizedPartyMessageArg,
			encryptedCentralizedSecretShareAndProofArg,
			encryptionKeyAddressArg,
			centralizedPublicOutputArg,
			signerPublicKeyArg,
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
	if (result.errors !== undefined) {
		throw new Error(`DKG second round failed with errors ${result.errors}`);
	}
	const startSessionEvent = result.events?.at(0)?.parsedJson;
	if (!isDWalletImportedKeyVerificationRequestEvent(startSessionEvent)) {
		throw new Error('invalid start session event');
	}
	await getObjectWithType(conf, dwalletID, isActiveDWallet);
	return startSessionEvent.encrypted_user_secret_key_share_id;
}

interface DWalletImportedKeyVerificationRequestEvent {
	encrypted_user_secret_key_share_id: string;
}

function isDWalletImportedKeyVerificationRequestEvent(
	event: any,
): event is DWalletImportedKeyVerificationRequestEvent {
	return event.encrypted_user_secret_key_share_id !== undefined;
}
