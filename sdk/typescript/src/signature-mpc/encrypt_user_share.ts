// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { serialized_pubkeys_from_decentralized_dkg_output } from '@dwallet-network/signature-mpc-wasm';
import _ from 'lodash';

import type { DWalletClient } from '../client/index.js';
import type { Keypair, PublicKey } from '../cryptography/index.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import { Ed25519PublicKey } from '../keypairs/ed25519/index.js';
import { generate_proof, getDwalletByObjID } from './dwallet_2pc_mpc_ecdsa_k1_module.js';
import {
	EncryptionKeyScheme,
	getActiveEncryptionKeyObjID,
	getEncryptionKeyByObjectId,
	saveEncryptedUserShare,
	setActiveEncryptionKey,
	storeEncryptionKey,
	transferEncryptedUserShare,
} from './dwallet.js';
import { decryptAndVerifyUserShare } from './sign';
import { generatePaillierKeyPairFromSuiKeyPair } from './utils';

export type DWalletToTransfer = {
	secretKeyShare: number[];
	decentralizedDKGOutput: number[];
	dwalletId: string;
};

/**
 * Encrypts and sends the given secret user share to the given destination public key.
 *
 * @param keypair The Sui keypair that was being used to sign the signedDWalletPubKeys.
 * @param dwallet The DWallet that we want to send the secret user share of.
 * @param destinationPublicKey The ed2551 public key of the destination Sui address.
 * @param activeEncryptionKeysTableID The ID of the table that holds the active encryption keys.
 * @param signedDWalletPubKeys The signed DWallet public keys.
 */
export const sendUserShareToSuiPubKey = async (
	client: DWalletClient,
	keypair: Keypair,
	dwallet: DWalletToTransfer,
	destinationPublicKey: PublicKey,
	activeEncryptionKeysTableID: string,
	signedDWalletPubKeys: Uint8Array,
) => {
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		client,
		destinationPublicKey.toSuiAddress(),
		activeEncryptionKeysTableID,
	);

	const recipientData = await getEncryptionKeyByObjectId(client, activeEncryptionKeyObjID);
	let isValidEncryptionKey = await destinationPublicKey.verify(
		new Uint8Array(recipientData?.encryptionKey!),
		new Uint8Array(recipientData?.signedEncryptionKey!),
	);
	if (!isValidEncryptionKey) {
		throw new Error(
			'The destination public key has not been signed by the desired destination Sui address',
		);
	}
	const encryptedUserShareAndProof = generate_proof(
		new Uint8Array(dwallet.secretKeyShare),
		recipientData?.encryptionKey!,
	);

	return await transferEncryptedUserShare(
		client,
		keypair,
		encryptedUserShareAndProof,
		activeEncryptionKeyObjID,
		dwallet,
		signedDWalletPubKeys,
	);
};

export type EncryptedUserShare = {
	dwalletId: string;
	encryptedUserShareAndProof: number[];
	encryptionKeyObjID: string;
	signedDWalletPubkeys: number[];
	senderPubKey: number[];
};

export const getEncryptedUserShareByObjID = async (
	client: DWalletClient,
	objID: string,
): Promise<EncryptedUserShare | null> => {
	const response = await client.getObject({
		id: objID,
		options: { showContent: true },
	});

	const objectFields =
		response.data?.content?.dataType === 'moveObject'
			? (response.data?.content?.fields as unknown as {
					dwallet_id: string;
					encrypted_secret_share_and_proof: number[];
					encryption_key_id: string;
					signed_dwallet_pubkeys: number[];
					sender_pubkey: number[];
			  })
			: null;

	return objectFields
		? {
				dwalletId: objectFields.dwallet_id,
				encryptedUserShareAndProof: objectFields.encrypted_secret_share_and_proof,
				encryptionKeyObjID: objectFields.encryption_key_id,
				signedDWalletPubkeys: objectFields.signed_dwallet_pubkeys,
				senderPubKey: objectFields.sender_pubkey,
		  }
		: null;
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
		dwalletId: dwalletID,
		secretKeyShare: Array.from(decryptedKeyShare),
		decentralizedDKGOutput: dwallet!.decentralizedDKGOutput,
	};
	let serializedPubkeys = serialized_pubkeys_from_decentralized_dkg_output(
		new Uint8Array(dwallet?.decentralizedDKGOutput!),
	);
	// Encrypt it to self, so that in the future we'd know that we already verified everything and only need to verify our signature.
	const encryptedUserShareRef = await sendUserShareToSuiPubKey(
		client,
		keypair,
		dwalletToSend,
		keypair.getPublicKey(),
		encryptionKeysHolderObjID,
		await keypair.sign(serializedPubkeys),
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

export type EncryptionKeyPair = {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
};

export const getOrCreateEncryptionKey = async (
	keypair: Ed25519Keypair,
	client: DWalletClient,
	activeEncryptionKeysTableID: string,
): Promise<EncryptionKeyPair> => {
	let [encryptionKey, decryptionKey] = generatePaillierKeyPairFromSuiKeyPair(keypair);
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		client,
		keypair.toSuiAddress(),
		activeEncryptionKeysTableID,
	);
	if (activeEncryptionKeyObjID) {
		let encryptionKeyObj = await getEncryptionKeyByObjectId(client, activeEncryptionKeyObjID);
		if (_.isEqual(encryptionKeyObj?.encryptionKey, encryptionKey)) {
			return {
				encryptionKey,
				decryptionKey,
				objectID: activeEncryptionKeyObjID,
			};
		} else {
			throw new Error(
				'Encryption key derived from Sui secret does not match the one in the active encryption keys table',
			);
		}
	}
	const encryptionKeyRef = await storeEncryptionKey(
		encryptionKey,
		EncryptionKeyScheme.Paillier,
		keypair,
		client,
	);
	await setActiveEncryptionKey(
		client,
		keypair,
		encryptionKeyRef?.objectId!,
		activeEncryptionKeysTableID,
	);
	return {
		decryptionKey,
		encryptionKey,
		objectID: encryptionKeyRef.objectId,
	};
};
