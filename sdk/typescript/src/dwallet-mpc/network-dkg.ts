import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { DWallet } from './dkg.js';
import { dWalletMoveType, isDWallet } from './dkg.js';
import type { Config } from './globals.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletPackageID,
	fetchObjectFromEvent,
} from './globals.js';

interface CompletedDKGSecondRoundEvent {
	session_id: string;
	initiator: string;
	dwallet_cap_id: string;
	dwallet_id: string;
	value: number[];
}

export async function launchNetworkDKG(c: Config) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::dwallet_network_key::emit_start_network_decryption_key_share_generation`,
		arguments: [tx.pure(bcs.u8().serialize(1))],
	});

	await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	return await dWalletFromEvent(c);
}

async function dWalletFromEvent(conf: Config): Promise<DWallet> {
	function isCompletedDKGSecondRoundEvent(event: any): event is CompletedDKGSecondRoundEvent {
		return (
			event &&
			event.session_id &&
			event.initiator &&
			event.dwallet_cap_id &&
			event.dwallet_id &&
			Array.isArray(event.value)
		);
	}

	return fetchObjectFromEvent<CompletedDKGSecondRoundEvent, DWallet>({
		conf,
		eventType: `${dWalletPackageID}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedDKGSecondRoundEvent`,
		objectType: dWalletMoveType,
		isEvent: isCompletedDKGSecondRoundEvent,
		isObject: isDWallet,
		filterEvent: (_) => true,
		getObjectId: (event) => event.dwallet_id,
	});
}
