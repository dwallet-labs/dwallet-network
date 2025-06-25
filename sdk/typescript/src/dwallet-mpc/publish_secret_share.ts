import { bcs } from '@mysten/bcs';

import type { Config } from './globals.js';
import { DWALLET_COORDINATOR_MOVE_MODULE_NAME, getObjectWithType } from './globals.js';
import {
	createBaseTransaction,
	destroyEmptyIKACoin,
	executeTransactionWithTiming,
} from './transaction-utils.js';

export async function makeDWalletUserSecretKeySharesPublicRequestEvent(
	conf: Config,
	dwallet_id: string,
	secret_share: Uint8Array,
) {
	const { tx, emptyIKACoin, dwalletStateArg, sessionIdentifier } =
		await createBaseTransaction(conf);

	tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_COORDINATOR_MOVE_MODULE_NAME}::request_make_dwallet_user_secret_key_shares_public`,
		arguments: [
			dwalletStateArg,
			tx.pure.id(dwallet_id),
			tx.pure(bcs.vector(bcs.u8()).serialize(secret_share)),
			sessionIdentifier,
			emptyIKACoin,
			tx.gas,
		],
	});

	destroyEmptyIKACoin(tx, emptyIKACoin, conf);

	await executeTransactionWithTiming(conf, tx, 'Make DWallet User Secret Key Shares Public');
	await getObjectWithType(conf, dwallet_id, isDWalletWithPublicUserSecretKeyShares);
}

interface DWalletWithPublicUserSecretKeyShares {
	public_user_secret_key_share: Uint8Array;
	id: { id: string };
	dwallet_cap_id: string;
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
