import { bcs } from '../bcs/index.js';
import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';
import { dWallet2PCMPCECDSAK1ModuleName, fetchObjectBySessionId, packageId } from './globals.js';

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export async function signMessageTransactionCall(
	keypair: Keypair,
	client: PeraClient,
	hashedMessage: Uint8Array,
	presign: Uint8Array,
	dkgOutput: Uint8Array,
	centralizedSignedMessage: Uint8Array,
	presignFirstRoundSessionId: string,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`,
		arguments: [
			tx.pure(bcs.vector(bcs.u8()).serialize(hashedMessage)),
			tx.pure(bcs.vector(bcs.u8()).serialize(presign)),
			tx.pure(bcs.vector(bcs.u8()).serialize(dkgOutput)),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedSignedMessage)),
			tx.pure.id(presignFirstRoundSessionId),
		],
	});

	let res = await client.signAndExecuteTransaction({
		signer: keypair,
		transaction: tx,
		options: {
			showEvents: true,
		},
	});

	const eventData = res.events?.at(0)?.parsedJson as { session_id: string };

	let signOutput = await fetchObjectBySessionId(
		eventData.session_id,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::SignOutput`,
		keypair,
		client,
	);

	let output =
		signOutput?.dataType === 'moveObject'
			? (signOutput.fields as {
					id: { id: string };
					output: number[];
				})
			: null;

	return {
		id: output?.id.id,
		signOutput: output?.output,
	};
}
