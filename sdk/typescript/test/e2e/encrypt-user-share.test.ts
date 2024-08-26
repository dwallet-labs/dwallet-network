import { verify_user_share } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, expect, it } from 'vitest';

import {
	createActiveEncryptionKeysTable,
	createDWallet,
	decrypt_user_share,
	EncryptionKeyScheme,
	generate_keypair,
	generate_proof,
	getActiveEncryptionKeyObjID,
	getEncryptionKeyByObjectId,
	setActiveEncryptionKey,
	storeEncryptionKey,
	transferEncryptedUserShare,
} from '../../src/signature-mpc';
import { setup, TestToolbox } from './utils/setup';
import {sendUserShareToSuiPubKey} from "../../src/signature-mpc/encrypt_user_share";

describe('Secret key share transfer', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should encrypt and transfer a dwallet to a newly generated public key', async () => {
		const [encryptionKey, decryptionKey] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);
		const publicKeyID = pubKeyRef?.objectId;
		const recipientData = await getEncryptionKeyByObjectId(toolbox.client, publicKeyID);
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		const dwalletID = dwallet?.dwalletId!;
		const secretShare = dwallet?.secretKeyShare!;
		const encryptedUserShareAndProof = generate_proof(
			new Uint8Array(secretShare),
			recipientData?.encryptionKey!,
		);

		// Verifies that the encryption key has been signed by the desired destination Sui address.
		let isValidEncryptionKey = await toolbox.keypair
			.getPublicKey()
			.verify(
				new Uint8Array(recipientData?.encryptionKey!),
				new Uint8Array(recipientData?.signedEncryptionKey!),
			);
		expect(isValidEncryptionKey).toBeTruthy();

		await transferEncryptedUserShare(
			toolbox.client,
			toolbox.keypair,
			encryptedUserShareAndProof,
			publicKeyID,
			dwalletID,
		);

		const decryptedKeyShare = decrypt_user_share(
			encryptionKey,
			decryptionKey,
			encryptedUserShareAndProof,
		);

		let secretUserShare = new Uint8Array(256);
		secretUserShare.set(secretShare.reverse());
		expect(decryptedKeyShare).toEqual(secretUserShare);

		expect(
			verify_user_share(
				// Take the first 32 bytes, the only ones that are non-zero, and reverse them to convert them
				// from little-endian encoding to big-endian.
				// This is because of BCS and PlaintextSpaceGroupElement serialization.
				// PlaintextSpaceGroupElement is U2048 and has 32LIMBS of 64 bits each.
				new Uint8Array(decryptedKeyShare.slice(0, 32).reverse()),
				new Uint8Array(dwallet?.decentralizedDKGOutput!),
			),
		).toBeTruthy();
	});

	it('creates an encryption key & stores it in the active encryption keys table', async () => {
		const [encryptionKey, _] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);
		console.log({ pubKeyRef });

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			toolbox.client,
			toolbox.keypair,
		);

		await setActiveEncryptionKey(
			toolbox.client,
			toolbox.keypair,
			pubKeyRef?.objectId!,
			encryptionKeysHolder.objectId,
		);

		const activeEncryptionKeyAddress = await getActiveEncryptionKeyObjID(
			toolbox.client,
			toolbox.keypair.toSuiAddress(),
			encryptionKeysHolder.objectId,
		);

		expect(`0x${activeEncryptionKeyAddress}`).toEqual(pubKeyRef?.objectId!);
	});

	it('encrypts a secret share to a given Sui address successfully', async () => {
		const [encryptionKey, decryptionKey] = generate_keypair();
		const pubKeyRef = await storeEncryptionKey(
			encryptionKey,
			EncryptionKeyScheme.Paillier,
			toolbox.keypair,
			toolbox.client,
		);
		console.log({ pubKeyRef });

		const dkg = await createDWallet(toolbox.keypair, toolbox.client);

		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			toolbox.client,
			toolbox.keypair,
		);

		await setActiveEncryptionKey(
			toolbox.client,
			toolbox.keypair,
			pubKeyRef?.objectId!,
			encryptionKeysHolder.objectId,
		);

		const publicKeyID = pubKeyRef?.objectId;
		const recipientData = await getEncryptionKeyByObjectId(toolbox.client, publicKeyID);
		const secretShare = dkg?.secretKeyShare!;
		const encryptedUserShareAndProof = generate_proof(
			new Uint8Array(secretShare),
			recipientData?.encryptionKey!,
		);

		await sendUserShareToSuiPubKey(
			toolbox.client,
			toolbox.keypair,
			dkg!,
			toolbox.keypair.getPublicKey(),
			encryptionKeysHolder.objectId,
		);

		const decryptedKeyShare = decrypt_user_share(
			encryptionKey,
			decryptionKey,
			encryptedUserShareAndProof,
		);

		let secretUserShare = new Uint8Array(256);
		secretUserShare.set(secretShare.reverse());
		expect(decryptedKeyShare).toEqual(secretUserShare);

		expect(
			verify_user_share(
				// Take the first 32 bytes, the only ones that are non-zero, and reverse them to convert them
				// from little-endian encoding to big-endian.
				// This is because of BCS and PlaintextSpaceGroupElement serialization.
				// PlaintextSpaceGroupElement is U2048 and has 32LIMBS of 64 bits each.
				new Uint8Array(decryptedKeyShare.slice(0, 32).reverse()),
				new Uint8Array(dkg?.decentralizedDKGOutput!),
			),
		).toBeTruthy();
	});
});
