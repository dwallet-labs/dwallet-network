import { bcs } from '@mysten/bcs';
import { Transaction } from '@mysten/sui/transactions';

import { Config, DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME } from './globals.ts';

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export async function sign(
	conf: Config,
	presignID: string,
	dwalletID: string,
	dwalletCapID: string,
	message: Uint8Array,
	hash = Hash.KECCAK256,
) {
	// TODO: replace with mock
	let centralizedSignedMessage = new Uint8Array();
	let tx = new Transaction();
	const [messageApprovals] = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME}::approve_messages`,
		arguments: [
			tx.object(dwalletCapID),
			tx.pure(bcs.u8().serialize(hash.valueOf())),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize([message])),
		],
	});
}
