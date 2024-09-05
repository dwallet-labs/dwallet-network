// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { serialized_pubkeys_from_decentralized_dkg_output } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, expect, it } from 'vitest';

import {
	createActiveEncryptionKeysTable,
	createDWallet,
	EncryptionKeyScheme,
	generate_keypair,
	getActiveEncryptionKeyObjID,
	getEncryptedUserShareByObjectID,
	setActiveEncryptionKey,
	storeEncryptionKey,
} from '../../src/signature-mpc';
import {
	getEncryptedUserShareByObjID,
	getOrCreateEncryptionKey,
	sendUserShareToSuiPubKey,
} from '../../src/signature-mpc/encrypt_user_share';
import { acceptUserShare } from '../../src/signature-mpc/sign';
import { setup, TestToolbox } from './utils/setup';

describe('Secret key share transfer', () => {
	let dwalletSenderToolbox: TestToolbox;
	let dwalletReceiverToolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	beforeAll(async () => {
		dwalletSenderToolbox = await setup();
		dwalletReceiverToolbox = await setup();
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
	});

	it('creates an encryption key & stores it in the active encryption keys table', async () => {
		const [encryptionKey, _] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
		);

		await setActiveEncryptionKey(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
			pubKeyRef?.objectId!,
			activeEncryptionKeysTableID,
		);

		const activeEncryptionKeyAddress = await getActiveEncryptionKeyObjID(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair.toSuiAddress(),
			activeEncryptionKeysTableID,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(pubKeyRef?.objectId!);
	});

	it('full flow — encrypts a secret share to a given Sui public key successfully, and stores it on the chain from the receiving end', async () => {
		// ======================= Create Source DWallet =======================
		let senderEncryptionKeyObj = await getOrCreateEncryptionKey(
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
			activeEncryptionKeysTableID,
		);
		const createdDwallet = await createDWallet(
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
			senderEncryptionKeyObj.encryptionKey,
			senderEncryptionKeyObj.objectID,
		);

		// ======================= Create Destination Paillier Keypair =======================
		let receiverEncryptionKeyObj = await getOrCreateEncryptionKey(
			dwalletReceiverToolbox.keypair,
			dwalletReceiverToolbox.client,
			activeEncryptionKeysTableID,
		);

		// ======================= Send DWallet Secret Share To Destination Keypair  =======================
		let encryptedSecretShare = await getEncryptedUserShareByObjectID(
			dwalletSenderToolbox.client,
			createdDwallet?.encryptedSecretShareObjID!,
		);
		// Verify I signed the dkg output public keys before using it to send the user share.
		let signedDWalletPubKeys = new Uint8Array(encryptedSecretShare?.signedDWalletPubKeys!);
		expect(
			await dwalletSenderToolbox.keypair
				.getPublicKey()
				.verify(
					serialized_pubkeys_from_decentralized_dkg_output(
						new Uint8Array(createdDwallet?.decentralizedDKGOutput!),
					),
					signedDWalletPubKeys,
				),
		).toBeTruthy();

		let objRef = await sendUserShareToSuiPubKey(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
			createdDwallet!,
			dwalletReceiverToolbox.keypair.getPublicKey(),
			activeEncryptionKeysTableID,
			signedDWalletPubKeys,
		);

		// ======================= Verify Received DWallet is Valid =======================
		let encryptedUserShareObjID = objRef?.objectId;
		let encryptedUserShare = await getEncryptedUserShareByObjID(
			dwalletReceiverToolbox.client,
			encryptedUserShareObjID!,
		);

		expect(
			await acceptUserShare(
				encryptedUserShare!,
				dwalletSenderToolbox.keypair.toSuiAddress(),
				receiverEncryptionKeyObj,
				createdDwallet?.dwalletID!,
				activeEncryptionKeysTableID,
				dwalletReceiverToolbox.client,
				dwalletReceiverToolbox.keypair,
			),
		).toBeTruthy();
	});
});
