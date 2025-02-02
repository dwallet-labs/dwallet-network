// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { IkaObjectRef } from '@ika-io/ika/client';
import { getFaucetHost, requestIkaFromFaucetV1 } from '@ika-io/ika/faucet';
import { Ed25519Keypair } from '@ika-io/ika/keypairs/ed25519';
import { Transaction } from '@ika-io/ika/transactions';

import { client } from './rpc';

// This simulates what a server would do to sponsor a transaction
export async function sponsorTransaction(sender: string, transactionKindBytes: Uint8Array) {
	// Rather than do gas pool management, we just spin out a new keypair to sponsor the transaction with:
	const keypair = new Ed25519Keypair();
	const address = keypair.getPublicKey().toIkaAddress();
	console.log(`Sponsor address: ${address}`);

	await requestIkaFromFaucetV1({ recipient: address, host: getFaucetHost('testnet') });

	let payment: IkaObjectRef[] = [];
	let retires = 50;
	while (retires !== 0) {
		const coins = await client.getCoins({ owner: address, limit: 1 });
		if (coins.data.length > 0) {
			payment = coins.data.map((coin) => ({
				objectId: coin.coinObjectId,
				version: coin.version,
				digest: coin.digest,
			}));
			break;
		}
		await new Promise((resolve) => setTimeout(resolve, 200)); // Sleep for 200ms
		retires -= 1;
	}

	const tx = Transaction.fromKind(transactionKindBytes);
	tx.setSender(sender);
	tx.setGasOwner(address);
	tx.setGasPayment(payment);

	return keypair.signTransaction(await tx.build({ client }));
}
