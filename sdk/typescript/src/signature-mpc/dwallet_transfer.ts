import type { DWalletClient } from '../client/index.js';
import type { Keypair, PublicKey } from '../cryptography/index.js';
import {
	getActiveEncryptionKeyObjID,
	getEncryptionKeyByObjectId,
	transferEncryptedUserShare,
} from './dwallet';
import { Dwallet, generate_proof } from './dwallet_2pc_mpc_ecdsa_k1_module';

export const sendUserShareToAddress = async (
	client: DWalletClient,
	keypair: Keypair,
	dwallet: Dwallet,
	destinationPublicKey: PublicKey,
	activeEncryptionKeysTableID: string,
) => {
	const activeEncryptionKeyObjID = await getActiveEncryptionKeyObjID(
		client,
		keypair,
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
		dwallet.dwalletId,
	);
};
