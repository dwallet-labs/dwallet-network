// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';

const packageId = '0x3';
const dWalletProofMPCModuleName = 'proof';

export async function launchProofMPCEvent(keypair: Keypair, client: PeraClient) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWalletProofMPCModuleName}::launch_proof_mpc_flow`,
		arguments: [],
	});

	await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEffects: true,
		},
	});
}
