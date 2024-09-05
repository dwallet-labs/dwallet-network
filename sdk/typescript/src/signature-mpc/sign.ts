// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import {
	serialized_pubkeys_from_decentralized_dkg_output,
	verify_user_share,
} from '@dwallet-network/signature-mpc-wasm';

import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import { Ed25519PublicKey } from '../keypairs/ed25519/index.js';
import {
	createPartialUserSignedMessages,
	decrypt_user_share,
} from './dwallet_2pc_mpc_ecdsa_k1_module.js';
import {
	getActiveEncryptionKeyObjID,
	getDwalletByObjID,
	getEncryptedUserShare,
	getEncryptedUserShareByObjectID,
	saveEncryptedUserShare,
} from './dwallet.js';
import type { EncryptedUserShare, EncryptionKeyPair } from './encrypt_user_share.js';
import { getOrCreateEncryptionKey, sendUserShareToSuiPubKey } from './encrypt_user_share.js';

export const decryptAndVerifyUserShare = async (
	sourcePublicKey: Ed25519PublicKey,
	expectedSourceSuiAddress: string,
	dkgOutput: number[],
	encryptedUserShareObj: EncryptedUserShare,
	encryptionKeyObj: EncryptionKeyPair,
): Promise<Uint8Array> => {
	if (sourcePublicKey.toSuiAddress() !== expectedSourceSuiAddress) {
		throw new Error('The source public key does not match the expected Sui address');
	}
	let serializedPubKeys = serialized_pubkeys_from_decentralized_dkg_output(
		new Uint8Array(dkgOutput),
	);
	if (
		!(await sourcePublicKey.verify(
			serializedPubKeys,
			new Uint8Array(encryptedUserShareObj?.signedDWalletPubKeys!),
		))
	) {
		throw new Error('the DWallet public keys have not been signed by the desired Sui address');
	}
	const decryptedKeyShare = decrypt_user_share(
		encryptionKeyObj.encryptionKey,
		encryptionKeyObj.decryptionKey,
		new Uint8Array(encryptedUserShareObj?.encryptedUserShareAndProof!),
	);
	if (!verify_user_share(decryptedKeyShare, new Uint8Array(dkgOutput!))) {
		throw new Error("the decrypted key share doesn't match the dwallet's public key share");
	}
	return decryptedKeyShare;
};

export const acceptUserShare = async (
	encryptedUserShare: EncryptedUserShare,
	expectedSourceSuiAddress: string,
	encryptionKeyObj: EncryptionKeyPair,
	dwalletID: string,
	encryptionKeysHolderObjID: string,
	client: DWalletClient,
	keypair: Keypair,
): Promise<boolean> => {
	let dwallet = await getDwalletByObjID(client, dwalletID);
	// This function also verifies that the dkg output has been signed by the source public key.
	const decryptedKeyShare = await decryptAndVerifyUserShare(
		new Ed25519PublicKey(encryptedUserShare?.senderPubKey!),
		expectedSourceSuiAddress,
		dwallet?.decentralizedDKGOutput!,
		encryptedUserShare,
		encryptionKeyObj,
	);
	let dwalletToSend = {
		dwalletID,
		secretKeyShare: Array.from(decryptedKeyShare),
		decentralizedDKGOutput: dwallet!.decentralizedDKGOutput,
	};
	let serializedPubKeys = serialized_pubkeys_from_decentralized_dkg_output(
		new Uint8Array(dwallet?.decentralizedDKGOutput!),
	);
	// Encrypt it to self, so that in the future we'd know that we already
	// verified everything and only need to verify our signature.
	const encryptedUserShareRef = await sendUserShareToSuiPubKey(
		client,
		keypair,
		dwalletToSend,
		keypair.getPublicKey(),
		encryptionKeysHolderObjID,
		await keypair.sign(serializedPubKeys),
	);
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		client,
		keypair.toSuiAddress(),
		encryptionKeysHolderObjID,
	);
	await saveEncryptedUserShare(
		client,
		keypair,
		activeEncryptionKeyObjID,
		encryptedUserShareRef?.objectId!,
	);
	return true;
};

/**
 * Pre-signs the given message with the given DWallet ID.
 *
 * @param client
 * @param keypair The Sui keypair that encrypted the given dwallet to itself in the past. This keypair is
 * either the one who created the dwallet with the {@link createDWallet} function, or the one who accepted
 * it with the {@link acceptUserShare} function.
 * @param dwalletID
 * @param message
 * @param hash
 * @param activeEncryptionKeysTableID
 */
export const presignWithDWalletID = async (
	client: DWalletClient,
	keypair: Ed25519Keypair,
	dwalletID: string,
	message: Uint8Array,
	hash: 'KECCAK256' | 'SHA256',
	activeEncryptionKeysTableID: string,
): Promise<string | null> => {
	let encryptionKeyObj = await getOrCreateEncryptionKey(
		keypair,
		client,
		activeEncryptionKeysTableID,
	);

	let encryptedUserShareObjId = await getEncryptedUserShare(client, keypair, dwalletID);
	let encryptedUserShareObj = await getEncryptedUserShareByObjectID(
		client,
		encryptedUserShareObjId!,
	);
	let dwallet = await getDwalletByObjID(client, dwalletID);
	const decryptedKeyShare = await decryptAndVerifyUserShare(
		keypair.getPublicKey(),
		keypair.toSuiAddress(),
		dwallet?.decentralizedDKGOutput!,
		encryptedUserShareObj!,
		encryptionKeyObj,
	);
	return await createPartialUserSignedMessages(
		dwalletID,
		dwallet?.decentralizedDKGOutput!,
		decryptedKeyShare,
		[message],
		hash,
		keypair,
		client,
	);
};
