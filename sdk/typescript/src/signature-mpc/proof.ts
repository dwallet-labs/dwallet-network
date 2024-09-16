//[object Object]
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

const packageId = '0x3';
const dWalletProofMPCModuleName = 'proof';

export async function launchProofMPCEvent(keypair: Keypair, client: DWalletClient) {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${dWalletProofMPCModuleName}::launch_proof_mpc_flow`,
		arguments: [],
	});

	await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});
}
