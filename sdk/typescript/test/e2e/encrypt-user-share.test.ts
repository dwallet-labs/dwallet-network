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
	getEncryptedUserShareByObjectId,
	setActiveEncryptionKey,
	storeEncryptionKey,
} from '../../src/signature-mpc';
import {
	acceptUserShare,
	getEncryptedUserShareByObjID,
	sendUserShareToSuiPubKey,
} from '../../src/signature-mpc/encrypt_user_share';
import { generatePaillierKeyPairFromSuiKeyPair } from '../../src/signature-mpc/utils';
import { setup, TestToolbox } from './utils/setup';

describe('Secret key share transfer', () => {
	let dwalletSenderToolbox: TestToolbox;
	let dwalletReceiverToolbox: TestToolbox;

	beforeAll(async () => {
		dwalletSenderToolbox = await setup();
		dwalletReceiverToolbox = await setup();
	});

	it('creates an encryption key & stores it in the active encryption keys table', async () => {
		const [encryptionKey, _] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
		);
		console.log({ pubKeyRef });

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
		);

		await setActiveEncryptionKey(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
			pubKeyRef?.objectId!,
			encryptionKeysHolder.objectId,
		);

		const activeEncryptionKeyAddress = await getActiveEncryptionKeyObjID(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair.toSuiAddress(),
			encryptionKeysHolder.objectId,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(pubKeyRef?.objectId!);
	});

	it('full flow - encrypts a secret share to a given Sui public key successfully, and store it on chain from the receiving end', async () => {
		// ======================= Create Source DWallet =======================
		// TODO (#202): Create a function that retrieves an encryption key for the given keypair if it exists
		const [walletCreatorEncryptionKey, _] = generatePaillierKeyPairFromSuiKeyPair(
			dwalletSenderToolbox.keypair,
		);

		const pubKeyRef = await storeEncryptionKey(
			walletCreatorEncryptionKey,
			EncryptionKeyScheme.Paillier,
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
		);

		const createdDwallet = await createDWallet(
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
			walletCreatorEncryptionKey,
			pubKeyRef.objectId,
		);

		// ======================= Create Destination Paillier Keypair =======================
		const [walletReceiverEncryptionKey, walletReceiverDecryptionKey] =
			generatePaillierKeyPairFromSuiKeyPair(dwalletReceiverToolbox.keypair);

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			dwalletReceiverToolbox.client,
			dwalletReceiverToolbox.keypair,
		);

		const walletReceiverPubKeyRef = await storeEncryptionKey(
			walletReceiverEncryptionKey,
			EncryptionKeyScheme.Paillier,
			dwalletReceiverToolbox.keypair,
			dwalletReceiverToolbox.client,
		);

		await setActiveEncryptionKey(
			dwalletReceiverToolbox.client,
			dwalletReceiverToolbox.keypair,
			walletReceiverPubKeyRef?.objectId!,
			encryptionKeysHolder.objectId,
		);

		// ======================= Send DWallet Secret Share To Destination Keypair  =======================
		let encryptedSecretShare = await getEncryptedUserShareByObjectId(
			dwalletSenderToolbox.client,
			createdDwallet?.encryptedSecretShareObjId!,
		);
		// Verify I signed the dkg output public keys before using it to send the user share.
		let signedDWalletPubKeys = new Uint8Array(encryptedSecretShare?.signedDWalletPubkeys!);
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

		let txResponse = await sendUserShareToSuiPubKey(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
			createdDwallet!,
			dwalletReceiverToolbox.keypair.getPublicKey(),
			encryptionKeysHolder.objectId,
			signedDWalletPubKeys,
		);

		// ======================= Verify Received DWallet is Valid =======================
		let encryptedUserShareObjID = txResponse.effects?.created![0].reference.objectId;
		let encryptedUserShare = await getEncryptedUserShareByObjID(
			dwalletSenderToolbox.client,
			encryptedUserShareObjID!,
		);
		expect(
			await acceptUserShare(
				encryptedUserShare!,
				dwalletSenderToolbox.keypair.toSuiAddress(),
				walletReceiverEncryptionKey,
				walletReceiverDecryptionKey,
				createdDwallet?.dwalletId!,
				encryptionKeysHolder.objectId,
				dwalletReceiverToolbox.client,
				dwalletReceiverToolbox.keypair,
			),
		).toBeTruthy();
	});
});
