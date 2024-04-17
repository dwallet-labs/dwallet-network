// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '../bcs';
import { TransactionBlock } from '../builder';
import { SuiClient } from '../client';
import { Keypair } from '../cryptography';
import { fetchObjectBySessionId } from './utils';

const packageId = '0x3';
const dWalletModuleName = 'dwallet';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export async function approveAndSign(dwalletCapId: string, signMessagesId: string, messages: Uint8Array[], keypair: Keypair, client: SuiClient) {

	const tx = new TransactionBlock();
	const [messageApprovals] = tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::approve_messages`,
		arguments: [
			tx.object(dwalletCapId),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
		],
	});
	tx.moveCall({
		target: `${packageId}::${dWalletModuleName}::sign_messages`,
		typeArguments: [`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignData`],
		arguments: [tx.object(signMessagesId), messageApprovals],
	});
	const result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	const signSessionRef = result.effects?.created?.filter((o) => o.owner == 'Immutable')[0].reference!;

	const signOutput = await fetchObjectBySessionId(
		signSessionRef.objectId,
		`${packageId}::${dWalletModuleName}::SignOutput`,
		keypair,
		client,
	);

	if (signOutput?.dataType === 'moveObject') {
		// @ts-ignore
		return { signOutputId: signOutput.fields["id"]["id"], signatures: signOutput.fields["signatures"] };
	}

	return;
}