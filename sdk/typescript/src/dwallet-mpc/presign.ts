// public fun request_ecdsa_presign(
//     self: &mut DWallet2PcMpcSecp256K1,
//     dwallet_id: ID,
//     payment_ika: &mut Coin<IKA>,
//     payment_sui: &mut Coin<SUI>,
//     ctx: &mut TxContext
// ) {

import {
	Config,
	DWALLET_ECDSAK1_MOVE_MODULE_NAME,
	getDWalletSecpState,
	getInitialSharedVersion,
	SUI_PACKAGE_ID
} from './globals.ts';
import {Transaction} from "@mysten/sui/transactions";

export async function presign(conf: Config, dwallet_id: string) {
	const tx = new Transaction();
	let emptyIKACoin = tx.moveCall({
		target: `${SUI_PACKAGE_ID}::coin::zero`,
		arguments: [],
		typeArguments: [`${c.ikaConfig.ika_package_id}::ika::IKA`],
	});
	let dWalletStateData = await getDWalletSecpState(conf);

	let result = tx.moveCall({
		target: `${conf.ikaConfig.ika_system_package_id}::${DWALLET_ECDSAK1_MOVE_MODULE_NAME}::request_ecdsa_presign`,
		arguments: [
			tx.sharedObjectRef({
				objectId: dWalletStateData.object_id,
				initialSharedVersion: dWalletStateData.initial_shared_version,
				mutable: true,
			}),
			tx.pure.id(dwallet_id),
			emptyIKACoin,
			tx.gas,
		],
	});
}
