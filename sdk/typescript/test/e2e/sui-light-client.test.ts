// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// import { recovery_id_keccak256 } from '@dwallet-network/signature-mpc-wasm';

import { SuiClient } from '@mysten/sui.js/client';
import { ethers, SigningKey } from 'ethers';
import { beforeAll, describe, it } from 'vitest';

import { createBindToAuthority } from '../../src/authority-binder';
import {
	approveMessageInSui,
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
	createSuiDWalletCap,
	createVirginBoundDWallet,
	submitDWalletCreationProof,
	submitTxStateProof,
} from '../../src/signature-mpc';
import { getOrCreateEncryptionKey } from '../../src/signature-mpc/encrypt_user_share';
import { setup, TestToolbox } from './utils/setup';

describe('Test Sui Light Client', () => {
	let toolbox: TestToolbox;
	let authorityToolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	const packageId = '0x3';
	const suiStateModuleName = 'sui_state_proof';
	// const dWalletCapPackageSUI = '0x8b527e2c7b0b29f2f6fe25a5b4505a4e0473f2d54a1c9dfaff125eed1eb327fd';

	beforeAll(async () => {
		toolbox = await setup(false);
		authorityToolbox = await setup(false);
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			toolbox.client,
			toolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;
		// let reciepient = toolbox.keypair.toSuiAddress();
		// await requestSuiFromFaucetV0({
		// 	host: 'http://127.0.0.1:6124',
		// 	recipient: toolbox.keypair.toSuiAddress(),
		// });
	});

	it('full flow', async () => {
		// create authority encryption key
		let authorityEncryptionKeyObj = await getOrCreateEncryptionKey(
			authorityToolbox.keypair,
			authorityToolbox.client,
			activeEncryptionKeysTableID,
		);

		let authorityOwnerDWallet = await createDWallet(
			authorityToolbox.keypair,
			authorityToolbox.client,
			authorityEncryptionKeyObj.encryptionKey,
			authorityEncryptionKeyObj.objectID,
		);

		let encryptionKeyObj = await getOrCreateEncryptionKey(
			toolbox.keypair,
			toolbox.client,
			activeEncryptionKeysTableID,
		);

		const serviceUrl = 'http://localhost:6920/gettxdata';
		// const contractAddress2 = '0x4a22eaef6ba256D46Fb7935B1bdAd8cEb454EFCd';
		const contractAddress = '0xEd34EE41cA84042b619E9AEBF6175bB4a0069a05'; // remix IDE address
		const domainName = 'dWalletAuthenticator';
		const domainVersion = '1.0.0';
		const virginBound = true;
		const chainIdentifier = BigInt(101); // Sui Testnet ID

		let authorityId = '0xfaacfd76aab0de938473de461b90a79f8a2d30f4b0c5a40cbd3e604821292d47';

		const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';
		const suiClient = new SuiClient({ url: suiTestnetURL });

		// create bind to authority
		let configType = `${packageId}::${suiStateModuleName}::SuiStateProofConfig`;
		let bindToAuthorityId = await createBindToAuthority(
			authorityId,
			contractAddress,
			0,
			configType,
			toolbox.keypair,
			toolbox.client,
		);

		await new Promise((r) => setTimeout(r, 2000));

		// create virgin dwallet for user
		const virginEthDwallet = await createVirginBoundDWallet(
			encryptionKeyObj.encryptionKey,
			encryptionKeyObj.objectID,
			bindToAuthorityId,
			toolbox.keypair,
			toolbox.client,
		);

		const dWalletBinderId = virginEthDwallet?.dWalletBinderID!;

		// link virgin dwallet with sui dwallet cap
		let { createDWalletTxDigest, suiDWalletCapId } = await createSuiDWalletCap(
			virginEthDwallet!,
			activeEncryptionKeysTableID,
			authorityOwnerDWallet?.dwalletID!,
			authorityId,
			chainIdentifier,
			domainName,
			domainVersion,
			bindToAuthorityId,
			virginBound,
			suiClient,
			toolbox.client,
			toolbox.keypair,
		);

		await new Promise((r) => setTimeout(r, 6000));

		const message = 'dWallets are coming...';
		const messageEncoded = new TextEncoder().encode(message);

		let approveTxDigest = await approveMessageInSui(
			suiDWalletCapId,
			[messageEncoded],
			suiClient,
			toolbox.keypair,
		);

		// link dwallet to Sui dwallet cap in dwallet network
		await submitDWalletCreationProof(
			toolbox.client,
			suiClient,
			authorityId,
			dWalletBinderId,
			createDWalletTxDigest,
			serviceUrl,
			toolbox.keypair,
		);

		// partial sign same message with dwallet
		const signMessagesIdSHA256 = await createPartialUserSignedMessages(
			virginEthDwallet?.dwalletID!,
			virginEthDwallet?.decentralizedDKGOutput!,
			new Uint8Array(virginEthDwallet?.secretKeyShare!),
			[messageEncoded],
			'SHA256',
			toolbox.keypair,
			toolbox.client,
		);
		console.log('created signMessages');

		if (signMessagesIdSHA256 == null) {
			throw new Error('createSignMessages returned null');
		}

		// submit tx state proof to dwallet network
		await submitTxStateProof(
			toolbox.client,
			suiClient,
			authorityId,
			dWalletBinderId,
			signMessagesIdSHA256,
			approveTxDigest,
			virginEthDwallet!,
			serviceUrl,
			toolbox.keypair,
		);

		console.log(`dwallet binder id: ` + dWalletBinderId);
		console.log(`dwallet cap id: ` + virginEthDwallet?.dwalletCapID!);
		console.log(`bind to authority id: ` + bindToAuthorityId);
		console.log(`virgin bound:` + virginBound);
		console.log(`nonce:` + 123);
	});
});
