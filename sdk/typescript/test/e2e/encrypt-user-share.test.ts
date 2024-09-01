import {
	serialized_pubkeys_from_decentralized_dkg_output,
	verify_user_share,
} from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, expect, it } from 'vitest';

import {
	createActiveEncryptionKeysTable,
	createDWallet,
	decrypt_user_share,
	EncryptionKeyScheme,
	generate_keypair,
	generate_keypair_from_seed,
	generate_proof,
	getActiveEncryptionKeyObjID,
	getEncryptedUserShareByObjectId,
	getEncryptionKeyByObjectId,
	setActiveEncryptionKey,
	storeEncryptionKey,
	transferEncryptedUserShare,
} from '../../src/signature-mpc';
import {
	getEncryptedUserShareByObjID,
	sendUserShareToSuiPubKey,
	verifyEncryptedSecretShare,
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

	it('should encrypt and transfer a dwallet to a newly generated public key', async () => {
		const [encryptionKey, decryptionKey] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
		);
		const publicKeyID = pubKeyRef?.objectId;
		const recipientData = await getEncryptionKeyByObjectId(
			dwalletSenderToolbox.client,
			publicKeyID,
		);
		const dwallet = await createDWallet(
			dwalletSenderToolbox.keypair,
			dwalletSenderToolbox.client,
			encryptionKey,
			publicKeyID,
		);
		const secretShare = dwallet?.secretKeyShare!;
		const encryptedUserShareAndProof = generate_proof(
			new Uint8Array(secretShare),
			recipientData?.encryptionKey!,
		);

		// Verifies that the encryption key has been signed by the desired destination Sui address.
		let isValidEncryptionKey = await dwalletSenderToolbox.keypair
			.getPublicKey()
			.verify(
				new Uint8Array(recipientData?.encryptionKey!),
				new Uint8Array(recipientData?.signedEncryptionKey!),
			);
		expect(isValidEncryptionKey).toBeTruthy();
		await transferEncryptedUserShare(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
			encryptedUserShareAndProof,
			publicKeyID,
			dwallet!,
		);

		const decryptedKeyShare = decrypt_user_share(
			encryptionKey,
			decryptionKey,
			encryptedUserShareAndProof,
		);
		expect(decryptedKeyShare).toEqual(new Uint8Array(secretShare));

		expect(
			verify_user_share(
				new Uint8Array(decryptedKeyShare),
				new Uint8Array(dwallet?.decentralizedDKGOutput!),
			),
		).toBeTruthy();
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
		const [walletCreatorEncryptionKey, walletCreatorDecryptionKey] =
			generatePaillierKeyPairFromSuiKeyPair(dwalletSenderToolbox.keypair);

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
		expect(
			await dwalletSenderToolbox.keypair
				.getPublicKey()
				.verify(
					serialized_pubkeys_from_decentralized_dkg_output(
						new Uint8Array(createdDwallet?.decentralizedDKGOutput!),
					),
					new Uint8Array(encryptedSecretShare?.signedDWalletPubkeys!),
				),
		).toBeTruthy();

		let txResponse = await sendUserShareToSuiPubKey(
			dwalletSenderToolbox.client,
			dwalletSenderToolbox.keypair,
			createdDwallet!,
			dwalletReceiverToolbox.keypair.getPublicKey(),
			encryptionKeysHolder.objectId,
		);

		// ======================= Verify Received DWallet is Valid =======================
		let encryptedUserShareObjID = txResponse.effects?.created![0].reference.objectId;
		let encryptedUserShare = await getEncryptedUserShareByObjID(
			dwalletSenderToolbox.client,
			encryptedUserShareObjID!,
		);
		expect(
			await verifyEncryptedSecretShare(
				encryptedUserShare!,
				dwalletSenderToolbox.keypair.toSuiAddress(),
				walletReceiverEncryptionKey,
				walletReceiverDecryptionKey,
				createdDwallet?.dwalletId!,
				dwalletReceiverToolbox.client,
			),
		).toBeTruthy();

		// ======================= Receiver Encrypts Secret Share To Himself =======================
		const decryptedKeyShare = decrypt_user_share(
			walletReceiverEncryptionKey,
			walletReceiverDecryptionKey,
			new Uint8Array(encryptedUserShare?.encryptedUserShareAndProof!),
		);
		let dwalletToSend = {
			dwalletId: createdDwallet!.dwalletId,
			secretKeyShare: Array.from(decryptedKeyShare),
			decentralizedDKGOutput: createdDwallet!.decentralizedDKGOutput,
		};

		// It is now safe to sign the dwallet public keys, as we verified in verifyEncryptedSecretShare that they have been signed by the desired
		// Sui source address.
		await sendUserShareToSuiPubKey(
			dwalletReceiverToolbox.client,
			dwalletReceiverToolbox.keypair,
			dwalletToSend,
			dwalletReceiverToolbox.keypair.getPublicKey(),
			encryptionKeysHolder.objectId,
		);
	});
});
