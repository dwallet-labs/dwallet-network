// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import {
	approveEthereumMessage,
	createEthereumDWallet,
	initEthereumState,
} from '../../src/eth-light-client';
import { createDWallet } from '../../src/signature-mpc';
import { setup, TestToolbox } from './utils/setup';

describe('Test Ethereum Light Client', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	it('should init the state, create ethereum dwallet, and verify a message', async () => {
		const dwallet = await createDWallet(toolbox.keypair, toolbox.client);
		const dwalletID = dwallet?.dwalletId!;
		const dwalletCapID = dwallet?.dwalletCapId!;
		const network = 'holesky';
		const consensusRpc = 'http://unstable.holesky.beacon-api.nimbus.team';
		const executionRpc = 'https://eth-holesky.g.alchemy.com/v2/KxdGyszqQHA3rcEpy44FqOH1hhx7vq8g';
		const contractAddress = '0x4a22eaef6ba256D46Fb7935B1bdAd8cEb454EFCd';
		const contractApprovedTxSlot = 2;

		let latestStateId = await initEthereumState(
			network,
			consensusRpc,
			contractAddress,
			contractApprovedTxSlot,
			toolbox.keypair,
			toolbox.client,
		);

		let ethereumDWallet = await createEthereumDWallet(
			dwalletCapID,
			latestStateId,
			toolbox.keypair,
			toolbox.client,
		);

		// For this part to work, you need to wait until the block that includes the transaction we want to verify, is FINALIZED.
		if (ethereumDWallet !== undefined) {
			let message = 'U3VwcmlzZSEgSGF2ZSBhIGdyZWF0IGRheSE=';
			let messageApproval = await approveEthereumMessage(
				ethereumDWallet!,
				message,
				dwalletID,
				latestStateId,
				executionRpc,
				consensusRpc,
				toolbox.keypair,
				toolbox.client,
			);

			console.log(`messageApproval: ${messageApproval}`);
		}
	});
});
