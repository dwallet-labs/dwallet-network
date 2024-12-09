import { bcs } from '../bcs/index.js';
import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';
import { dWallet2PCMPCECDSAK1ModuleName, fetchObjectBySessionId, packageId } from './globals.js';

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export async function signMockCall(
	keypair: Keypair,
	client: PeraClient,
	hashedMessage: Uint8Array,
	presignFirstRound: Uint8Array,
	presignSecondRound: Uint8Array,
	dkgOutput: Uint8Array,
	centralizedSignedMessage: Uint8Array,
	presignFirstRoundSessionId: string,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::mock_sign`,
		arguments: [
			tx.pure(bcs.vector(bcs.u8()).serialize(hashedMessage)),
			tx.pure(bcs.vector(bcs.u8()).serialize(presignFirstRound)),
			tx.pure(bcs.vector(bcs.u8()).serialize(presignSecondRound)),
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
	return await fetchSignObjects(keypair, client, eventData.session_id);
}

export async function signMessageTransactionCall(
	keypair: Keypair,
	client: PeraClient,
	dwalletCapId: string,
	hashedMessage: Uint8Array,
	dwalletId: string,
	presignId: string,
	centralizedSignedMessage: Uint8Array,
	presignSessionId: string,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`,
		arguments: [
			tx.object(dwalletCapId),
			tx.pure(bcs.vector(bcs.u8()).serialize(hashedMessage)),
			tx.object(dwalletId),
			tx.object(presignId),
			tx.pure(bcs.vector(bcs.u8()).serialize(centralizedSignedMessage)),
			tx.object(presignSessionId),
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
	return await fetchSignObjects(keypair, client, eventData.session_id);
}
export async function fetchSignObjects(keypair: Keypair, client: PeraClient, session_id: string) {
	let signOutput = await fetchObjectBySessionId(
		session_id,
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
