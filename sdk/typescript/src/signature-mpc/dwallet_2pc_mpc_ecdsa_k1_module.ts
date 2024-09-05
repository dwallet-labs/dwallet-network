// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import {
	finalize_dkg,
	finalize_presign,
	initiate_dkg,
	initiate_presign,
	initiate_sign,
	serialized_pubkeys_from_centralized_dkg_output,
} from '@dwallet-network/signature-mpc-wasm';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { hashToNumber, saveEncryptedUserShare } from './dwallet.js';
import { fetchObjectBySessionId } from './utils.js';

export {
	decrypt_user_share,
	generate_keypair,
	generate_proof,
	generate_keypair_from_seed,
} from '@dwallet-network/signature-mpc-wasm';

const packageId = '0x3';
const dWallet2PCMPCECDSAK1ModuleName = 'dwallet_2pc_mpc_ecdsa_k1';

export type CreatedDwallet = {
	dwalletID: string;
	centralizedDKGOutput: number[];
	decentralizedDKGOutput: number[];
	dwalletCapID: string;
	secretKeyShare: number[];
	encryptedSecretShareObjID: string;
};

export async function createDWallet(
	keypair: Keypair,
	client: DWalletClient,
	encryptionKey: Uint8Array,
	encryptionKeyObjId: string,
): Promise<CreatedDwallet | null> {
	const resultDKG = initiate_dkg();

	const commitmentToSecretKeyShare = resultDKG['commitment_to_secret_key_share'];
	const decommitmentRoundPartyState = resultDKG['decommitment_round_party_state'];

	const tx = new TransactionBlock();
	const [cap] = tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_dkg_session`,
		arguments: [tx.pure(commitmentToSecretKeyShare)],
	});
	tx.transferObjects([cap], keypair.toSuiAddress());
	const result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	const sessionRef = result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference!;

	const sessionOutput = await fetchObjectBySessionId(
		sessionRef.objectId,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::DKGSessionOutput`,
		keypair,
		client,
	);
	const sessionOutputFields =
		sessionOutput?.dataType === 'moveObject'
			? (sessionOutput.fields as {
					id: { id: string };
					secret_key_share_encryption_and_proof: number[];
			  })
			: null;

	if (sessionOutputFields) {
		const final = finalize_dkg(
			decommitmentRoundPartyState,
			Uint8Array.from(sessionOutputFields.secret_key_share_encryption_and_proof),
			encryptionKey,
		);
		let serializedPubKeys = serialized_pubkeys_from_centralized_dkg_output(final['dkg_output']);
		const txFinal = new TransactionBlock();
		txFinal.moveCall({
			target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_dwallet`,
			arguments: [
				txFinal.object(sessionOutputFields.id.id),
				txFinal.pure(final['public_key_share_decommitment_and_proof']),
				txFinal.pure(encryptionKeyObjId),
				txFinal.pure(final['encrypted_user_share_and_proof']),
				txFinal.pure([...(await keypair.sign(serializedPubKeys))]),
				txFinal.pure([...keypair.getPublicKey().toRawBytes()]),
			],
		});
		const signResult = await client.signAndExecuteTransactionBlock({
			signer: keypair,
			transactionBlock: txFinal,
			options: {
				showEffects: true,
			},
		});

		let dwalletRef = signResult.effects?.created?.filter((o) => {
			return o.owner === 'Immutable';
		})[0].reference!;
		let encryptedShareRef = signResult.effects?.created?.filter((o) => o.owner === 'Immutable')[1]
			.reference!;

		let dwalletObject = await client.getObject({
			id: dwalletRef.objectId,
			options: { showContent: true },
		});
		let dwalletObjectFields =
			dwalletObject.data?.content?.dataType === 'moveObject'
				? (dwalletObject.data?.content?.fields as {
						dwallet_cap_id: string;
						output: number[];
				  })
				: null;
		if (!dwalletObjectFields?.dwallet_cap_id) {
			// This may happen as the order of the created objects is not guaranteed,
			// and we can't know the object type from the reference.
			let tempRef = dwalletRef;
			dwalletRef = encryptedShareRef;
			encryptedShareRef = tempRef;
			dwalletObject = await client.getObject({
				id: dwalletRef.objectId,
				options: { showContent: true },
			});
			dwalletObjectFields =
				dwalletObject.data?.content?.dataType === 'moveObject'
					? (dwalletObject.data?.content?.fields as {
							dwallet_cap_id: string;
							output: number[];
					  })
					: null;
		}
		await saveEncryptedUserShare(client, keypair, encryptionKeyObjId, encryptedShareRef.objectId);
		return dwalletObjectFields
			? {
					dwalletID: dwalletRef?.objectId,
					centralizedDKGOutput: final['dkg_output'],
					decentralizedDKGOutput: dwalletObjectFields.output,
					dwalletCapID: dwalletObjectFields.dwallet_cap_id,
					secretKeyShare: final['secret_key_share'],
					encryptedSecretShareObjID: encryptedShareRef.objectId,
			  }
			: null;
	}
	return null;
}

export type DWallet = {
	dwalletID: string;
	decentralizedDKGOutput: number[];
	dwalletCapID: string;
};

export async function createPartialUserSignedMessages(
	dwalletID: string,
	decentralizedDKGOutput: number[],
	secretKeyShare: Uint8Array,
	messages: Uint8Array[],
	hash: 'KECCAK256' | 'SHA256',
	keypair: Keypair,
	client: DWalletClient,
) {
	const resultPresign = initiate_presign(
		Uint8Array.of(...decentralizedDKGOutput),
		secretKeyShare,
		messages.length,
	);

	const nonceSharesCommitmentsAndBatchedProof =
		resultPresign['nonce_shares_commitments_and_batched_proof'];
	const signatureNonceSharesAndCommitmentRandomnesses =
		resultPresign['signature_nonce_shares_and_commitment_randomnesses'];

	const hashNum = hashToNumber(hash);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_presign_session`,
		arguments: [
			tx.object(dwalletID),
			tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(messages)),
			tx.pure(nonceSharesCommitmentsAndBatchedProof),
			tx.pure.u8(hashNum),
		],
	});
	const result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: {
			showEffects: true,
		},
	});

	const sessionRef = result.effects?.created?.filter((o) => o.owner === 'Immutable')[0].reference!;

	const sessionOutput = await fetchObjectBySessionId(
		sessionRef.objectId,
		`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::PresignSessionOutput`,
		keypair,
		client,
	);

	const sessionOutputFields =
		sessionOutput?.dataType === 'moveObject'
			? (sessionOutput.fields as {
					id: { id: string };
					output: number[];
			  })
			: null;

	if (sessionOutputFields) {
		const presigns = finalize_presign(
			Uint8Array.of(...decentralizedDKGOutput),
			secretKeyShare,
			signatureNonceSharesAndCommitmentRandomnesses,
			Uint8Array.from(sessionOutputFields.output),
		);

		const presignOutput = await fetchObjectBySessionId(
			sessionRef.objectId,
			`${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::Presign`,
			keypair,
			client,
		);

		const presignOutputFields =
			presignOutput?.dataType === 'moveObject'
				? (presignOutput.fields as {
						id: { id: string };
				  })
				: null;

		if (presignOutputFields) {
			const bcsMessages = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();

			const publicNonceEncryptedPartialSignatureAndProofs = initiate_sign(
				Uint8Array.of(...decentralizedDKGOutput),
				secretKeyShare,
				presigns,
				bcsMessages,
				hashNum,
			);

			const txFinal = new TransactionBlock();
			const [signMessagesObject] = txFinal.moveCall({
				target: `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_partial_user_signed_messages`,
				arguments: [
					txFinal.object(dwalletID),
					txFinal.object(sessionRef.objectId),
					txFinal.object(sessionOutputFields.id.id),
					txFinal.object(presignOutputFields.id.id),
					txFinal.pure(publicNonceEncryptedPartialSignatureAndProofs),
				],
			});
			txFinal.transferObjects([signMessagesObject], keypair.toSuiAddress());
			const resultFinal = await client.signAndExecuteTransactionBlock({
				signer: keypair,
				transactionBlock: txFinal,
				options: {
					showEffects: true,
					showObjectChanges: true,
				},
			});

			return resultFinal.effects?.created?.at(0)?.reference.objectId!;
		}
	}
	return null;
}
