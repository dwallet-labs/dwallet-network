import { bcs } from '../bcs/index.js';
import { Transaction } from '../transactions/index.js';
import type { Config } from './globals.js';
import { dWalletPackageID } from './globals.js';

export async function launchNetworkDKG(c: Config) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${dWalletPackageID}::dwallet_network_key::start_network_dkg`,
		arguments: [tx.pure(bcs.u8().serialize(1))],
	});

	await c.client.signAndExecuteTransaction({
		signer: c.keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
}
