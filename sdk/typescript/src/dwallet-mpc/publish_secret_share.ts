import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/dist/esm/transactions';

import {
	Config,
	DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME,
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	fetchCompletedEvent,
	getDWalletSecpState,
	SUI_PACKAGE_ID,
} from './globals';

export async function makeDWalletUserSecretKeySharesPublicRequestEvent(
	conf: Config,
	dwallet_id: string,
	secret_share: Uint8Array,
) {
	const tx = new Transaction();
	const emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});
	const dWalletStateData = await getDWalletSecpState(conf);

	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_make_dwallet_user_secret_key_shares_public`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.pure.id(dwallet_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(secret_share)),
			emptyIKACoin,
			tx.gas,
		],
	});

	tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::destroy_zero`,
		arguments: [emptyIKACoin],
		typeArguments: [`${conf.ikaConfig.ika_package_id}::ika::IKA`],
	});

	await conf.client.signAndExecuteTransaction({
		signer: conf.suiClientKeypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
		},
	});
}
