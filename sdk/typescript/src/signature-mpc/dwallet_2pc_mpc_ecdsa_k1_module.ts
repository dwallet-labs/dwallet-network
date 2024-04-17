// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import {SuiClient} from "../client";
import {Keypair} from "../cryptography";
import {TransactionBlock} from "../builder";
import {
	finalize_dkg,
	finalize_presign,
	initiate_dkg,
	initiate_presign,
	initiate_sign,
} from '@dwallet-network/signature-mpc-wasm/pkg';
import {bcs} from "../bcs";
import {fetchObjectBySessionId} from "./utils";

const packageId = "0x3";
const dWallet2PCMPCECDSAK1ModuleName = "dwallet_2pc_mpc_ecdsa_k1";

export async function createDWallet(keypair: Keypair, client: SuiClient) {

	const resultDKG = initiate_dkg();

	const commitmentToSecretKeyShare = resultDKG["commitment_to_secret_key_share"];
	const decommitmentRoundPartyState = resultDKG["decommitment_round_party_state"];

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_dkg_session`,
		arguments: [tx.pure(commitmentToSecretKeyShare)],
	});
	const result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	const sessionRef = result.effects?.created?.filter((o) => o.owner == 'Immutable')[0].reference!;

	const sessionOutput = await fetchObjectBySessionId(sessionRef.objectId, `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::DKGSessionOutput`, keypair, client);

	if(sessionOutput?.dataType == "moveObject") {
		// @ts-ignore
		const final = finalize_dkg(decommitmentRoundPartyState, sessionOutput.fields["secret_key_share_encryption_and_proof"]);

		const txFinal = new TransactionBlock();
		// @ts-ignore
		txFinal.moveCall({
			target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_dwallet`,
			// @ts-ignore
			arguments: [txFinal.object(sessionOutput.fields["id"]["id"]), txFinal.pure(final["public_key_share_decommitment_and_proof"])],
		});
		const resultFinal = await client.signAndExecuteTransactionBlock({
			signer: keypair,
			transactionBlock: txFinal,
			options: {
				showEffects: true,
			},
		});

		const dwalletRef = resultFinal.effects?.created?.filter((o) => o.owner == 'Immutable')[0].reference!;

		const dwalletObject = await client.getObject({id: dwalletRef.objectId, options: {showContent: true}});

		if(dwalletObject.data?.content?.dataType == "moveObject") {
			// @ts-ignore
			const dwalletCapId = dwalletObject.data?.content?.fields["dwallet_cap_id"];
			return { dwalletId: dwalletRef?.objectId, dkgOutput: final["dkg_output"], dwalletCapId };

		}

	}
	return null;
}

function hashToNumber(hash: 'KECCAK256' | 'SHA256') {
	if(hash === "KECCAK256") {
		return 0;
	} else {
		return 1;
	}
}

export async function createSignMessages(dwalletId: string, dkgOutput: number[], messages: Uint8Array[], hash: "KECCAK256" | "SHA256", keypair: Keypair, client: SuiClient) {

	const resultPresign = initiate_presign(Uint8Array.of(...dkgOutput), messages.length);

	const nonceSharesCommitmentsAndBatchedProof = resultPresign["nonce_shares_commitments_and_batched_proof"];
	const signatureNonceSharesAndCommitmentRandomnesses = resultPresign["signature_nonce_shares_and_commitment_randomnesses"];

	const hashNum = hashToNumber(hash);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_presign_session`,
		arguments: [tx.object(dwalletId), tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)), tx.pure(nonceSharesCommitmentsAndBatchedProof), tx.pure.u8(hashNum)],
	});
	const result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	const sessionRef = result.effects?.created?.filter((o) => o.owner == 'Immutable')[0].reference!;

	const sessionOutput = await fetchObjectBySessionId(sessionRef.objectId, `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::PresignSessionOutput`, keypair, client);


	if(sessionOutput?.dataType == "moveObject") {
		// @ts-ignore
		const presigns = finalize_presign(Uint8Array.of(...dkgOutput), signatureNonceSharesAndCommitmentRandomnesses, sessionOutput.fields["output"]);

		const presignOutput = await fetchObjectBySessionId(sessionRef.objectId, `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::Presign`, keypair, client);

		if(presignOutput?.dataType == "moveObject") {

			const bcsMessages = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();

			const publicNonceEncryptedPartialSignatureAndProofs = initiate_sign(Uint8Array.of(...dkgOutput), presigns, bcsMessages, hashNum);

			const txFinal = new TransactionBlock();
			const [signMessagesObject] = txFinal.moveCall({
				target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_sign_messages`,
				// @ts-ignore
				arguments: [txFinal.object(dwalletId), txFinal.object(sessionRef.objectId), txFinal.object(sessionOutput.fields["id"]["id"]), txFinal.object(presignOutput.fields["id"]["id"]), txFinal.pure(publicNonceEncryptedPartialSignatureAndProofs)],
			});
			txFinal.transferObjects([signMessagesObject], keypair.toSuiAddress());
			const resultFinal = await client.signAndExecuteTransactionBlock({
				signer: keypair,
				transactionBlock: txFinal,
				options: {
					showEffects: true,
					showObjectChanges: true
				},
			});

			return resultFinal.effects?.created?.at(0)?.reference.objectId!;
		}
	}
	return null;
}

