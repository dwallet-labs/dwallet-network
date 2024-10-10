// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import {
	approveEthereumMessage,
	createEthereumDWallet,
	initEthereumState,
} from '../../src/eth-light-client';
import { createActiveEncryptionKeysTable, createDWallet } from '../../src/signature-mpc';
import { getOrCreateEncryptionKey } from '../../src/signature-mpc/encrypt_user_share';
import { setup, TestToolbox } from './utils/setup';

describe('Test Ethereum Light Client', () => {
	let toolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	beforeAll(async () => {
		toolbox = await setup();
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			toolbox.client,
			toolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
	});

	it('should init the state, create ethereum dwallet, and verify a message', async () => {
		let encryptionKeyObj = await getOrCreateEncryptionKey(
			toolbox.keypair,
			toolbox.client,
			activeEncryptionKeysTableID,
		);
		const dwallet = await createDWallet(
			toolbox.keypair,
			toolbox.client,
			encryptionKeyObj.encryptionKey,
			encryptionKeyObj.objectID,
		);
		const dwalletID = dwallet?.dwalletID!;
		const dwalletCapID = dwallet?.dwalletCapID!;
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
			let messageApprovalBcs = await approveEthereumMessage(
				ethereumDWallet!,
				message,
				dwalletID,
				latestStateId,
				executionRpc,
				consensusRpc,
				toolbox.keypair,
				toolbox.client,
			);
		}
	});
});
