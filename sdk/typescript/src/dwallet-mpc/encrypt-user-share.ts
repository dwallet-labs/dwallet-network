import { generate_secp_cg_keypair_from_seed } from '@dwallet-network/dwallet-mpc-wasm';
import { SuiClient } from '@mysten/sui/client';
import { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';
import {toHex} from "@mysten/bcs"
import {
	Config, DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	fetchObjectWithType,
	getDWalletSecpState,
	getInitialSharedVersion
} from './globals';
import {Transaction} from "@mysten/sui/transactions";

/**
 * A class groups key pair.
 */
interface ClassGroupsSecpKeyPair {
	encryptionKey: Uint8Array;
	decryptionKey: Uint8Array;
	objectID: string;
}

/**
 * Retrieves the active encryption key object ID
 * from the active encryption keys table *activeEncryptionKeysTableID*
 * for the given address — derived from the public key.
 * Throws an error otherwise.
 */
async function getActiveEncryptionKeyObjID(conf: Config, address: string): Promise<string> {
	const tx = new Transaction();
	let dwalletState = await getDWalletSecpState(conf);
	tx.moveCall({
		target: `${conf.ikaConfig.ika_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::get_active_encryption_key`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dwalletState.object_id,
				initialSharedVersion: dwalletState.initial_shared_version,
				mutable: false,
			}),
			tx.pure.address(address)],
	});

	// Safe to use this function as it has been used here:
	// https://github.com/dwallet-labs/dwallet-network/blob/29929ded135f05578b6ce33b52e6ff5e894d0487/sdk/deepbook-v3/src/client.ts#L84
	// in late 2024 (can be seen with git blame).
	// Note that regular `getObject()` is not working because of dynamic fields.
	const res = await conf.client.devInspectTransactionBlock({
		sender: address,
		transactionBlock: tx,
	});

	const objIDArray = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0) as number[]);
	return toHex(objIDArray);
}

/**
 * A class groups Move encryption key object.
 */
interface EncryptionKey {
	encryption_key: Uint8Array;
	key_owner_address: string;
	encryption_key_signature: Uint8Array;
}

async function getOrCreateClassGroupsKeyPair(conf: Config): Promise<ClassGroupsSecpKeyPair> {
	const [expectedEncryptionKey, decryptionKey] = generate_secp_cg_keypair_from_seed(
		conf.dWalletSeed,
	);
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		conf,
		conf.keypair.toSuiAddress(),
	);
	const encryptionKeyMoveType = `${conf.ikaConfig.ika_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::EncryptionKey`;

	if (activeEncryptionKeyObjID) {
		const activeEncryptionKeyObj = await fetchObjectWithType<EncryptionKey>(
			conf,
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
