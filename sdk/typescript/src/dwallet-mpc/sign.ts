import { bcs } from '../bcs/index.js';
import type { PeraClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { Transaction } from '../transactions/index.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	getEventByTypeAndSessionId,
	packageId,
} from './globals.js';

export enum Hash {
	KECCAK256 = 0,
	SHA256 = 1,
}

export async function signMockCall(
	keypair: Keypair,
	client: PeraClient,
	hashedMessages: Uint8Array[],
	presignFirstRound: Uint8Array,
	presignSecondRound: Uint8Array,
	dkgOutput: Uint8Array,
	centralizedSignedMessages: Uint8Array[],
	presignFirstRoundSessionId: string,
) {
	const tx = new Transaction();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::mock_sign`,
		arguments: [
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
			tx.pure(bcs.vector(bcs.u8()).serialize(presignFirstRound)),
			tx.pure(bcs.vector(bcs.u8()).serialize(presignSecondRound)),
			tx.pure(bcs.vector(bcs.u8()).serialize(dkgOutput)),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(centralizedSignedMessages)),
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

	console.log({ res });

	const eventData = res.events?.find(
		(event) =>
			event.type === `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::StartBatchedSignEvent`,
	)?.parsedJson as {
		session_id: string;
	};
	let completionEvent = await getEventByTypeAndSessionId(
		client,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedSignEvent`,
		eventData.session_id,
	);
	return (completionEvent as { signed_messages: Uint8Array[] }).signed_messages;
}

export async function signMessageTransactionCall(
	keypair: Keypair,
	client: PeraClient,
	dwalletCapId: string,
	hashedMessages: Uint8Array[],
	dwalletId: string,
	presignId: string,
	centralizedSignedMessages: Uint8Array[],
	presignSessionId: string,
) {
	const tx = new Transaction();

	const [messageApprovals] = tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::approve_messages`,
		arguments: [
			tx.object(dwalletCapId),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
		],
	});

	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::sign`,
		arguments: [
			tx.pure.id(dwalletCapId),
			messageApprovals,
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(hashedMessages)),
			tx.object(presignId),
			tx.object(dwalletId),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(centralizedSignedMessages)),
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
	const eventData = res.events?.find(
		(event) =>
			event.type === `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::StartBatchedSignEvent`,
	)?.parsedJson as {
		session_id: string;
	};
	let completionEvent = await getEventByTypeAndSessionId(
		client,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::CompletedSignEvent`,
		eventData.session_id,
	);
	return (completionEvent as { signed_messages: Uint8Array[] }).signed_messages;
}
