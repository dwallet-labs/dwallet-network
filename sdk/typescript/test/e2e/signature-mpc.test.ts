// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { create_dkg_centralized_output } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, it } from 'vitest';

import { launchDKGSecondRound, startFirstDKGSession } from '../../src/signature-mpc/proof';
import { setup, TestToolbox } from './utils/setup';

describe('Test signature mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should create DWallet', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const firstDKGOutput = await startFirstDKGSession(toolbox.keypair, toolbox.client);
		let [publicKeyShareAndProof, _] = create_dkg_centralized_output(
			Uint8Array.from(firstDKGOutput!.output),
			firstDKGOutput?.session_id!.slice(2)!,
		);
		console.log(publicKeyShareAndProof);
		await launchDKGSecondRound(
			toolbox.keypair,
			toolbox.client,
			publicKeyShareAndProof,
			Uint8Array.from(firstDKGOutput!.output),
			firstDKGOutput?.dwallet_cap_id!,
			firstDKGOutput?.session_id!,
		);
	});
});
