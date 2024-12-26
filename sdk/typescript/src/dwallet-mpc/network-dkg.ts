import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWalletPackageID } from './globals.js';

export async function launchNetworkDKG(c: Config) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::pera_system::decryption_key_shares`,
		arguments: [tx.pure(bcs.u8().serialize(1)), tx.object('0x5')],
	});

	const res = await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
	console.log(res);
}
