import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import type { Config } from './globals.js';
import {
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	getDWalletSecpState,
	getObjectWithType,
	SUI_PACKAGE_ID,
} from './globals.js';

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
	await getObjectWithType(conf, dwallet_id, isDWalletWithPublicUserSecretKeyShares);
}

interface DWalletWithPublicUserSecretKeyShares {
	public_user_secret_key_shares: {
		vec: [];
	};
}

export function isDWalletWithPublicUserSecretKeyShares(
	obj: any,
): obj is DWalletWithPublicUserSecretKeyShares {
	return (
		obj &&
		Array.isArray(obj.public_user_secret_key_share) &&
		obj.public_user_secret_key_shares.length > 0
	);
}
