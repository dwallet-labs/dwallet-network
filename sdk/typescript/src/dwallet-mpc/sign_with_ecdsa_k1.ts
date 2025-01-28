import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import type { SerializedBcs } from '@mysten/bcs';

import { bcs } from '../bcs/index.js';
import type { TransactionArgument } from '../transactions/index.js';
import { Transaction } from '../transactions/index.js';
import { EncryptedUserShare, fetchEncryptedUserSecretShare } from './encrypt-user-share.js';
import {
	dWallet2PCMPCECDSAK1ModuleName,
	dWalletMoveType,
	fetchObjectWithType,
	isDWallet,
	mockedProtocolPublicParameters,
	MPCKeyScheme,
	packageId,
} from './globals.js';
import type { Config, DWallet, DWalletWithSecretKeyShare } from './globals.js';
import { fetchProtocolPublicParameters } from './network-dkg.js';
import { presign } from './presign.js';
import type { CompletedSignEvent } from './sign.js';
import { Hash, signMessageTransactionCall } from './sign.js';

export const dWalletCurveMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::Secp256K1`;
export const signDataMoveType = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::AlgorithmSpecificData`;
export const createSignDataMoveFunc = `${packageId}::${dWallet2PCMPCECDSAK1ModuleName}::create_signing_algorithm_data`;

export function createSignDataMoveArgs(
	presignIDs: string[],
	centralizedSignedMessages: Uint8Array[],
	dWallet: DWallet | DWalletWithSecretKeyShare,
): (TransactionArgument | SerializedBcs<any>)[] {
	const tx = new Transaction();
	return [
		tx.makeMoveVec({ elements: presignIDs.map((presignID) => tx.object(presignID)) }),
		tx.pure(bcs.vector(bcs.vector(bcs.u8())).serialize(centralizedSignedMessages)),
		tx.object(dWallet.id.id),
	];
}

/**
 * Presigns and Signs a message with the dWallets' on-chain encrypted secret share.
 * Can be called with any dWallet, as the encrypted secret share is automatically created
 * upon dWallet creation.
 *
 * @param conf The Pera config to run the TXs with.
 * @param dwalletID The ID of the dWallet to sign with.
 * @param activeEncryptionKeysTableID The ID of the active encryption keys table that holds the client encryption key.
 * @param messages The messages to sign.
 * @param mockNetworkKey A boolean indicating whether to use a mocked chain MPC network key
 * for testing purposes or to use the real one.
 * defaults to false, a.k.a. to use the real one.
 */
export async function signWithEncryptedDWallet(
	conf: Config,
	dwalletID: string,
	activeEncryptionKeysTableID: string,
	messages: Uint8Array[],
	mockNetworkKey: boolean = false,
): Promise<CompletedSignEvent> {
	const dWallet = await fetchObjectWithType<DWallet>(conf, dWalletMoveType, isDWallet, dwalletID);
	const encryptedSecretShare = await fetchEncryptedUserSecretShare(conf, dwalletID);
	const userShare = EncryptedUserShare.fromConfig(conf);
	// The share is encrypted to myself, this is why the source and dest are the same.
	const decryptedShare = await userShare.decryptAndVerifyUserShare(
		activeEncryptionKeysTableID,
		encryptedSecretShare,
		dWallet,
		conf.keypair.toPeraAddress(),
		conf.keypair,
	);

	const presignCompletionEvent = await presign(conf, dWallet.id.id, messages.length);
	const serializedMsgs = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
	const serializedPresigns = bcs
		.vector(bcs.vector(bcs.u8()))
		.serialize(presignCompletionEvent.presigns)
		.toBytes();
	const serializedPresignFirstRoundSessionIds = bcs
		.vector(bcs.string())
		.serialize(
			presignCompletionEvent.first_round_session_ids.map((session_id) => session_id.slice(2)),
		)
		.toBytes();
	const protocolPublicParameters = mockNetworkKey
		? mockedProtocolPublicParameters
		: await fetchProtocolPublicParameters(
				conf,
				MPCKeyScheme.Secp256k1,
				dWallet.dwallet_mpc_network_decryption_key_version,
			);
	const [centralizedSignedMsg] = create_sign_centralized_output(
		protocolPublicParameters,
		MPCKeyScheme.Secp256k1,
		Uint8Array.from(dWallet.centralized_public_output),
		decryptedShare,
		serializedPresigns,
		serializedMsgs,
		Hash.SHA256,
		serializedPresignFirstRoundSessionIds,
	);

	let signDataArgs = createSignDataMoveArgs(
		presignCompletionEvent.presign_ids,
		centralizedSignedMsg,
		dWallet,
	);

	return await signMessageTransactionCall(
		conf,
		dWallet,
		messages,
		Hash.SHA256,
		signDataArgs,
		createSignDataMoveFunc,
		dWalletCurveMoveType,
		signDataMoveType,
	);
}
