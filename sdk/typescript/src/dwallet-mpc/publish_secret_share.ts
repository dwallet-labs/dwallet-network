import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import type { Config } from './globals.js';
import {
	DWALLET_ECDSA_K1_MOVE_MODULE_NAME,
	// getDWalletSecpState, // No longer directly used
	getDWalletStateArg,
	getObjectWithType,
	handleIKACoin,
} from './globals.js';

export async function makeDWalletUserSecretKeySharesPublicRequestEvent(
	conf: Config,
	dwallet_id: string,
	secret_share: Uint8Array,
) {
	const tx = new Transaction();
	const emptyIKACoin = handleIKACoin(tx, conf);
	// const dWalletStateData = await getDWalletSecpState(conf); // No longer needed
	const dwalletStateArg = await getDWalletStateArg(conf, tx, true);

	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSA_K1_MOVE_MODULE_NAME}::request_make_dwallet_user_secret_key_shares_public`,
		arguments: [
			dwalletStateArg,
			tx.pure.id(dwallet_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(secret_share)),
			emptyIKACoin,
			tx.gas,
		],
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
	public_user_secret_key_share: {
		vec: [];
	};
}

export function isDWalletWithPublicUserSecretKeyShares(
	obj: any,
): obj is DWalletWithPublicUserSecretKeyShares {
	return (
		obj &&
		Array.isArray(obj.public_user_secret_key_share) &&
		obj.public_user_secret_key_share.length > 0
	);
}
