import { generate_secp_cg_keypair_from_seed } from '@dwallet-network/dwallet-mpc-wasm';
import { SuiClient } from '@mysten/sui/client';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';



import { Config } from './globals';


/**
 * A class groups key pair.
 */
interface ClassGroupsSecpKeyPair {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
}

async function getOrCreateClassGroupsKeyPair(
	conf: Config,
): Promise<ClassGroupsSecpKeyPair> {
	const [expectedEncryptionKey, decryptionKey] = generate_secp_cg_keypair_from_seed(conf.dWalletSeed);
	const activeEncryptionKeyObjID = await this.getActiveEncryptionKeyObjID(
		conf.keypair.getPublicKey(),
		activeEncryptionKeysTableID,
	);
	if (activeEncryptionKeyObjID) {
		const activeEncryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
			this.toConfig(keyPair),
			encryptionKeyMoveType,
			isEncryptionKey,
			activeEncryptionKeyObjID,
		);
		if (isEqual(activeEncryptionKeyObj?.encryption_key, expectedEncryptionKey)) {
			return {
				encryptionKey: expectedEncryptionKey,
				decryptionKey,
				objectID: activeEncryptionKeyObjID,
			};
		}
		throw new Error(
			'encryption key derived from the key pair does not match the one in the active encryption keys table',
		);
	}

	const encryptionKeyCreationEvent: CreatedEncryptionKeyEvent = await this.registerEncryptionKey(
		keyPair,
		expectedEncryptionKey,
		EncryptionKeyScheme.ClassGroups,
	);
	await delay(checkpointCreationTime);

	await this.upsertActiveEncryptionKey(
		keyPair,
		encryptionKeyCreationEvent.encryption_key_id,
		activeEncryptionKeysTableID,
	);
	await delay(checkpointCreationTime);

	return {
		decryptionKey,
		encryptionKey: expectedEncryptionKey,
		objectID: encryptionKeyCreationEvent.encryption_key_id,
	};
}
