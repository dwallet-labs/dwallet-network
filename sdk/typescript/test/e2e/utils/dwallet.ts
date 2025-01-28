// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
// noinspection ES6PreferShortImport

import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { expect } from 'vitest';

import { bcs } from '../../../src/bcs/index.js';
import { createDWallet } from '../../../src/dwallet-mpc/dkg.js';
import type { Config } from '../../../src/dwallet-mpc/globals.js';
import { MPCKeyScheme } from '../../../src/dwallet-mpc/globals.js';
import { presign } from '../../../src/dwallet-mpc/presign.js';
import {
	createSignDataMoveArgs,
	createSignDataMoveFunc,
	dWalletCurveMoveType,
	signDataMoveType,
} from '../../../src/dwallet-mpc/sign_with_ecdsa_k1.js';

/**
 * Run the Full MPC User Sessions
 */
export async function fullMPCUserSessionsECDSAK1(
	c: Config,
	protocolPublicParameters: Uint8Array,
	activeEncryptionKeysTableID: string,
) {
	const dWallet = await createDWallet(c, protocolPublicParameters, activeEncryptionKeysTableID);
	console.log({ dWallet });
	expect(dWallet).toBeDefined();
	const presignCompletionEvent = await presign(c, dWallet.id.id, 2);
	console.log({ presignCompletionEvent });
	expect(presignCompletionEvent).toBeDefined();
	const messages = [Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])];
	let serializedMsgs = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
	let serializedPresigns = bcs
		.vector(bcs.vector(bcs.u8()))
		.serialize(presignCompletionEvent.presigns)
		.toBytes();
	let serializedPresignFirstRoundSessionIds = bcs
		.vector(bcs.string())
		.serialize(
			presignCompletionEvent.first_round_session_ids.map((session_id) => session_id.slice(2)),
		)
		.toBytes();
	const [centralizedSignedMsg, _] = create_sign_centralized_output(
		protocolPublicParameters,
		MPCKeyScheme.Secp256k1,
		Uint8Array.from(dWallet.centralized_public_output),
		Uint8Array.from(dWallet.centralizedSecretKeyShare),
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

	console.log('Signing message');
	let signOutput = await signMessageTransactionCall(
		c,
		dWallet,
		messages,
		Hash.SHA256,
		signDataArgs,
		createSignDataMoveFunc,
		dWalletCurveMoveType,
		signDataMoveType,
	);

	expect(signOutput).toBeDefined();
	console.log({ signOutput });
}
