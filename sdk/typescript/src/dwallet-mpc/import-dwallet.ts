import { Transaction } from '@mysten/sui/transactions';

import {
	Config,
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	getDwalletSecp256k1ObjID,
	getInitialSharedVersion,
	getNetworkDecryptionKeyID,
	SUI_PACKAGE_ID,
} from './globals';

/**
 * Create an imported dWallet & return the dWallet ID.
 */
export async function createImportedDWallet(conf: Config): Promise<string> {
	const tx = new Transaction();
	const networkDecryptionKeyID = await getNetworkDecryptionKeyID(conf);
	const dwalletSecp256k1ID = await getDwalletSecp256k1ObjID(conf);
	const dwalletCap = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::new_imported_key_dwallet`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dwalletSecp256k1ID,
				initialSharedVersion: await getInitialSharedVersion(conf, dwalletSecp256k1ID),
				mutable: true,
			}),
			tx.pure.id(networkDecryptionKeyID),
			tx.pure.u32(0),
		],
	});
	tx.transferObjects([dwalletCap], conf.suiClientKeypair.toSuiAddress());
	const result = await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
	return '';
}
