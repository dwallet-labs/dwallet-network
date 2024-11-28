// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { create_sign_centralized_output } from '@dwallet-network/dwallet-mpc-wasm';
import { bcs } from '@mysten/bcs';
import { beforeAll, describe, it } from 'vitest';

import { createDWallet } from '../../src/dwallet-mpc/dkg';
import { mockedDWallet, mockedPresign } from '../../src/dwallet-mpc/mock';
import { presign } from '../../src/dwallet-mpc/presign';
import { Hash, signMessageTransactionCall, signMockCall } from '../../src/dwallet-mpc/sign';
import { setup, TestToolbox } from './utils/setup';

describe('Test dwallet mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should create DWallet', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		console.log(dwallet);
	});

	it('should create presign', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		console.log({ dwallet });
		const presignOutput = await presign(toolbox.keypair, toolbox.client, dwallet!.dwalletID);
		console.log({ presignOutput });
	});

	it('should sign message', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		console.log({ dwallet });
		const presignOutput = await presign(toolbox.keypair, toolbox.client, dwallet!.dwalletID);
		console.log({ presignOutput });
		let messages = [Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])];
		let bcsMessages = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
		const [signed_messages, hashed_messages] = create_sign_centralized_output(
			Uint8Array.from(dwallet?.centralizedDKGOutput!),
			Uint8Array.from(presignOutput?.firstRoundOutput!),
			Uint8Array.from(presignOutput?.secondRoundOutput!),
			bcsMessages,
			Hash.SHA256,
			presignOutput?.sessionId.slice(2)!,
		);

		let response = await signMessageTransactionCall(
			toolbox.keypair,
			toolbox.client,
			dwallet?.dwalletCapID!,
			hashed_messages,
			dwallet?.dwalletID!,
			presignOutput?.id!,
			signed_messages,
			presignOutput?.sessionId!,
		);

		console.log({ response });
	}, 10_000_000);

	it('should sign a message successfully with mock ', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		let messages = [Uint8Array.from([1, 2, 3, 4, 5]), Uint8Array.from([6, 7, 8, 9, 10])];
		let bcsMessages = bcs.vector(bcs.vector(bcs.u8())).serialize(messages).toBytes();
		const [centralizely_signed_messages, hashed_messages] = create_sign_centralized_output(
			Uint8Array.from(mockedDWallet.centralizedDKGOutput),
			Uint8Array.from(mockedPresign.firstRoundOutput),
			Uint8Array.from(mockedPresign.secondRoundOutput),
			bcsMessages,
			Hash.SHA256,
			mockedPresign.firstRoundSessionID.slice(2)!,
		);

		let response = await signMockCall(
			toolbox.keypair,
			toolbox.client,
			hashed_messages,
			mockedPresign.firstRoundOutput,
			mockedPresign.secondRoundOutput,
			mockedDWallet.decentralizedDKGOutput,
			centralizely_signed_messages,
			mockedPresign.firstRoundSessionID,
		);
		console.log({ response });
	});
});
