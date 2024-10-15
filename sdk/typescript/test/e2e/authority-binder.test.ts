// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { beforeAll, describe, it } from 'vitest';

import {
	createAuthority,
	createAuthorityAckTransactionHash,
	createAuthorityBinder,
	createConfig,
} from '../../src/authority-binder';
import { initEthereumState } from '../../src/eth-light-client';
import { createActiveEncryptionKeysTable, createDWallet } from '../../src/signature-mpc';
import { getOrCreateEncryptionKey } from '../../src/signature-mpc/encrypt_user_share';
import { OwnedObjectRef } from '../../src/types/objects';
import { setup, TestToolbox } from './utils/setup';

describe('Test Ethereum Light Client', () => {
	let toolbox: TestToolbox;
	let toolbox2: TestToolbox;
	let activeEncryptionKeysTableID: string;
	let activeEncryptionKeysTableID2: string;

	const packageId = '0x3';
	const ethereumStateModuleName = 'ethereum_state';
	const authorityBinderModuleName = 'authority_binder';

	beforeAll(async () => {
		toolbox = await setup();
		toolbox2 = await setup();
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			toolbox.client,
			toolbox.keypair,
		);
		const encryptionKeysHolder2 = await createActiveEncryptionKeysTable(
			toolbox2.client,
			toolbox2.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
		activeEncryptionKeysTableID2 = encryptionKeysHolder2.objectId;
	});

	it('should create an authority, dWalletBinder, create an ack transaction hash, and sign it', async () => {
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
		const dwalletCapID = dwallet?.dwalletCapID!;

		const contractAddress = '0x3e2aabb763f255cbb6a322dbe532192e120b5c6b';
		const domainName = 'dWalletAuthenticator';
		const domainVersion = '1.0.0';
		const virginBound = true;

		// create dwallet for authority
		let encryptionKeyObj2 = await getOrCreateEncryptionKey(
			toolbox2.keypair,
			toolbox2.client,
			activeEncryptionKeysTableID2,
		);
		let authorityOwnerDWallet = await createDWallet(
			toolbox2.keypair,
			toolbox2.client,
			encryptionKeyObj2.encryptionKey,
			encryptionKeyObj2.objectID,
		);
		const authorityOwnerDWalletCapID = authorityOwnerDWallet?.dwalletCapID!;

		const binderName = 'Ethereum_Holesky';
		const chainIdentifier = 123456;

		const network = 'holesky';
		const consensusRpc = 'http://unstable.holesky.beacon-api.nimbus.team';
		const contractApprovedTxSlot = 2;

		let latestStateOwnedObjectRef = (await initEthereumState(
			network,
			consensusRpc,
			contractAddress,
			contractApprovedTxSlot,
			toolbox.keypair,
			toolbox.client,
		)) as OwnedObjectRef;

		let configOwnedObjectRef = (await createConfig(
			toolbox.keypair,
			toolbox.client,
		)) as OwnedObjectRef;

		let latestStateObjType = `${packageId}::${ethereumStateModuleName}::LatestEthereumState`;
		let configObjType = `${packageId}::${authorityBinderModuleName}::Config`;

		// create authority
		const authorityOwnedObjRef = (await createAuthority(
			binderName,
			chainIdentifier.toString(),
			latestStateOwnedObjectRef,
			latestStateObjType,
			configOwnedObjectRef,
			configObjType,
			authorityOwnerDWalletCapID,
			toolbox2.keypair,
			toolbox2.client,
		)) as OwnedObjectRef;

		// create dWalletBinder
		const dWalletBinderOwnedObjRef = await createAuthorityBinder(
			dwalletCapID,
			authorityOwnedObjRef,
			virginBound,
			contractAddress,
			0, // owner type - contract
			toolbox.keypair,
			toolbox.client,
		);

		// create authorityAckTransactionHash
		const authorityAckTransactionHash = await createAuthorityAckTransactionHash(
			dWalletBinderOwnedObjRef,
			virginBound,
			chainIdentifier,
			domainName,
			domainVersion,
			toolbox.keypair,
			toolbox.client,
		);
		// sign authorityAckTransactionHash with `keccak256` and authority's dwallet
		// send bind command to smart contract
	});
});
