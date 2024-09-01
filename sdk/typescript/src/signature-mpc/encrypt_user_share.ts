// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import {
	serialized_pubkeys_from_decentralized_dkg_output,
	verify_user_share,
} from '@dwallet-network/signature-mpc-wasm';

import type { DWalletClient } from '../client/index.js';
import type { Keypair, PublicKey } from '../cryptography/index.js';
import { Ed25519PublicKey } from '../keypairs/ed25519/index.js';
import {
	decrypt_user_share,
	generate_proof,
	getDwalletByObjID,
} from './dwallet_2pc_mpc_ecdsa_k1_module.js';
import {
	getActiveEncryptionKeyObjID,
	getEncryptionKeyByObjectId,
	transferEncryptedUserShare,
} from './dwallet.js';

export type DWalletToTransfer = {
	secretKeyShare: number[];
	decentralizedDKGOutput: number[];
	dwalletId: string;
};

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
	encryptionKey: Uint8Array,
	decryptionKey: Uint8Array,
	dwalletID: string,
	encryptionKeysHolderObjID: string,
	client: DWalletClient,
	keypair: Keypair,
): Promise<boolean> => {
	let dwallet = await getDwalletByObjID(client, dwalletID);
	let publicKey = new Ed25519PublicKey(encryptedUserShare?.senderPubKey!);
	let serializedPubkeys = serialized_pubkeys_from_decentralized_dkg_output(
		new Uint8Array(dwallet?.decentralizedDKGOutput!),
	);
	if (
		!(await publicKey.verify(
			serializedPubkeys,
			new Uint8Array(encryptedUserShare?.signedDWalletPubkeys!),
		))
	) {
		return false;
	}
	if (publicKey.toSuiAddress() !== expectedSourceSuiAddress) {
		return false;
	}

	const decryptedKeyShare = decrypt_user_share(
		encryptionKey,
		decryptionKey,
		new Uint8Array(encryptedUserShare?.encryptedUserShareAndProof!),
	);

	if (!verify_user_share(decryptedKeyShare, new Uint8Array(dwallet?.decentralizedDKGOutput!))) {
		return false;
	}
	let dwalletToSend = {
		dwalletId: dwalletID,
		secretKeyShare: Array.from(decryptedKeyShare),
		decentralizedDKGOutput: dwallet!.decentralizedDKGOutput,
	};

	await sendUserShareToSuiPubKey(
		client,
		keypair,
		dwalletToSend,
		keypair.getPublicKey(),
		encryptionKeysHolderObjID,
	);
	return true;
};
