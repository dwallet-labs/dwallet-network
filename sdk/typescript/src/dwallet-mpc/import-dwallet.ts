import {
	create_imported_dwallet_centralized_step,
	encrypt_secret_share,
} from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import { acceptEncryptedUserShare } from './dkg.js';
import { getOrCreateClassGroupsKeyPair } from './encrypt-user-share.js';
import type { Config, DWallet, SharedObjectData } from './globals.js';
import {
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	getDwalletSecp256k1ObjID,
	getDWalletSecpState,
	getInitialSharedVersion,
	getNetworkDecryptionKeyID,
	getNetworkDecryptionKeyPublicOutput,
	getObjectWithType,
	isActiveDWallet,
	SUI_PACKAGE_ID,
} from './globals.js';

interface NewImportedKeyDWalletEvent {
	dwallet_id: string;
	dwallet_cap_id: string;
}

interface DWalletImportedKeyVerificationRequestEvent {
	event_data: {
		encrypted_user_secret_key_share_id: string;
	};
}

function isNewImportedKeyDWalletEvent(event: any): event is NewImportedKeyDWalletEvent {
	return event.dwallet_id !== undefined && event.dwallet_cap_id !== undefined;
}

// todo(zeev): refactor for a better API
// https://github.com/dwallet-labs/dwallet-network/pull/1040/files#r2097645823
export async function createImportedDWallet(conf: Config, secretKey: Uint8Array): Promise<DWallet> {
	const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
	const importedDWalletData = await createImportedDWalletMoveCall(conf);

	// The outgoing message and the public output are sent to the network.
	// They include the encrypted network share, encrypted by the network encryption key.
	const [secret_share, public_output, outgoing_message] = create_imported_dwallet_centralized_step(
		networkDecryptionKeyPublicOutput,
		// remove 0x prefix.
		importedDWalletData.dwallet_id.slice(2),
		secretKey,
	);
	const classGroupsSecpKeyPair = await getOrCreateClassGroupsKeyPair(conf);

	const encryptedUserShareAndProof = encrypt_secret_share(
		secret_share,
		classGroupsSecpKeyPair.encryptionKey,
		networkDecryptionKeyPublicOutput,
	);
	const dwalletState = await getDWalletSecpState(conf);
	const encryptedSecretShareID = await verifyImportedDWalletMoveCall(
		conf,
		dwalletState,
		importedDWalletData.dwallet_cap_id,
		outgoing_message,
		encryptedUserShareAndProof,
		public_output,
		importedDWalletData.dwallet_id,
	);
	const dwallet = await getObjectWithType(conf, importedDWalletData.dwallet_id, isActiveDWallet);
	await acceptEncryptedUserShare(conf, {
		dwallet_id: dwallet.id.id,
		encrypted_user_secret_key_share_id: encryptedSecretShareID,
	});
	return {
		dwalletID: importedDWalletData.dwallet_id,
		dwallet_cap_id: importedDWalletData.dwallet_cap_id,
		encrypted_secret_share_id: encryptedSecretShareID,
		secret_share,
		output: dwallet.state.fields.public_output,
	};
}

/**
 * Create an imported dWallet & return the dWallet ID.
 */
export async function createImportedDWalletMoveCall(
	conf: Config,
): Promise<NewImportedKeyDWalletEvent> {
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
	return startSessionEvent.event_data.encrypted_user_secret_key_share_id;
}

function isDWalletImportedKeyVerificationRequestEvent(
	event: any,
): event is DWalletImportedKeyVerificationRequestEvent {
	return event.event_data.encrypted_user_secret_key_share_id !== undefined;
}
