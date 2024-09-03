// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { DWalletClient } from '../client/index.js';
import type { Ed25519Keypair } from '../keypairs/ed25519/index.js';
import {
	createPartialUserSignedMessages,
	decrypt_user_share,
	getDwalletByObjID,
} from './dwallet_2pc_mpc_ecdsa_k1_module.js';
import {
	approveAndSign,
	getEncryptedUserShare,
	getEncryptedUserShareByObjectId,
} from './dwallet.js';
import { generatePaillierKeyPairFromSuiKeyPair } from './utils.js';

/**
 * Signs the given message with the given DWallet ID.
 *
 * @param keypair The Sui keypair that encrypted the given dwallet to itself in the past. This keypair is
 * either the one who created the dwallet with the {@link createDWallet} function, or the one who accepted
 * it with the {@link acceptUserShare} function.
 */
export const signWithDWalletID = async (
	client: DWalletClient,
	keypair: Ed25519Keypair,
	dwalletID: string,
	message: Uint8Array,
) => {
	let [encryptionKey, decryptionKey] = generatePaillierKeyPairFromSuiKeyPair(keypair);

	let encryptedUserShareObjId = await getEncryptedUserShare(client, keypair, dwalletID);
	let encryptedUserShareObj = await getEncryptedUserShareByObjectId(
		client,
		encryptedUserShareObjId!,
	);
	const decryptedKeyShare = decrypt_user_share(
		encryptionKey,
		decryptionKey,
		new Uint8Array(encryptedUserShareObj?.encryptedUserShareAndProof!),
	);
	let dwallet = await getDwalletByObjID(client, dwalletID);
	const signMessagesIdSHA256 = await createPartialUserSignedMessages(
		dwalletID,
		dwallet?.decentralizedDKGOutput!,
		decryptedKeyShare,
		[message],
		'SHA256',
		keypair,
		client,
	);
	return await approveAndSign(
		dwallet?.dwalletCapId!,
		signMessagesIdSHA256!,
		[message],
		keypair,
		client,
	);
};
