// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import { createAuthority, createAuthorityBinder, createBindToAuthority, createAuthorityAckTransactionHash } from '../../src/authority-binder';
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

	it('should create an authority, bindToAuthority, dWalletBinder, create an ack transaction hash, and sign it', async () => {
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

		const contractAddress = '0x3e2aabb763f255cbb6a322dbe532192e120b5c6b';
		const chainID = '123456';
		const domainName = 'dWalletAuthenticator';
		const domainVersion = '1.0.0';
		const virginBound = false;

		// create dwallet for authority
		// create authority
		// create bindToAuthority
		// create dWalletBinder
		// create authorityAckTransactionHash
		// sign authorityAckTransactionHash with `keccak256` and authority's dwallet
		// send bind command to smart contract
	});
});
