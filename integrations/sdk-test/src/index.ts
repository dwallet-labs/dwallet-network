// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SuiClient } from '@mysten/sui.js/client';
import { createBindToAuthority } from '../../../sdk/typescript/src/authority-binder';
import {
	approveMessageInSui,
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
	createSuiDWalletCap,
	createVirginBoundDWallet,
	submitDWalletCreationProof,
	submitTxStateProof,
} from '../../../sdk/typescript/src/signature-mpc';
import { getOrCreateEncryptionKey } from '../../src/signature-mpc/encrypt_user_share';
import { setup, TestToolbox } from './utils/setup';

async function main() {
	// Full Flow:
	// 1. Create a dWallet for the Authority
	// 2. Create the Authority using the Rust Sui Light Client interface (not implemented here).
	// 		Use dWalletCapId as parameter.
	// 3. Create BindToAuthority Object.
	// 4. Create a Virgin Bound dWallet for the User.
	// 5. Link the virgin dwallet with sui dwallet cap (actually create Sui dwallet cap).
	// 6. Submit the dWallet Creation Proof to the dWallet network.
	// 7. Approve a message with the Sui dwallet cap.
	// 8. Presign the message with the dWallet, in dWallet Network.
	// 9. Submit the Tx State Proof to the dWallet network.

	try {

		let toolbox: TestToolbox;
		let authorityToolbox: TestToolbox;
		let activeEncryptionKeysTableID: string;

		const packageId = '0x3';
		const suiStateModuleName = 'sui_state_proof';

		const serviceUrl = 'http://localhost:6920/gettxdata';
		const contractAddress = '0xEd34EE41cA84042b619E9AEBF6175bB4a0069a05'; // remix IDE address
		const domainName = 'dWalletAuthenticator';
		const domainVersion = '1.0.0';
		const virginBound = true;
		const chainIdentifier = BigInt(101); // Sui Testnet ID

		let authorityId = '0xfaacfd76aab0de938473de461b90a79f8a2d30f4b0c5a40cbd3e604821292d47';

		const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';
		const suiClient = new SuiClient({url: suiTestnetURL});

		// This is the updated packageId for the dWallet module in Sui.
		// const dWalletCapPackageSUI = '0x8fa033eeb4d0e97e5558b2307f932b11c6e6f9cc4240b5285a3370bf25924a6f';

		toolbox = await setup();
		authorityToolbox = await setup();
		const encryptionKeysHolder = await createActiveEncryptionKeysTable(
			toolbox.client,
			toolbox.keypair,
		);
		activeEncryptionKeysTableID = encryptionKeysHolder.objectId;

		// Request gas from faucet.
		toolbox.keypair.toSuiAddress();
		await requestSuiFromFaucetV0({
			host: suiTestnetURL,
			recipient: toolbox.keypair.toSuiAddress(),
		});

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


		// Note: Put a breakpoint here, and create the Authority using the Rust Sui Light Client interface.
		// 		After creating the authority, update `authorityId` with the new authority ID.

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
		let {createDWalletTxDigest, suiDWalletCapId} = await createSuiDWalletCap(
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
	} catch (error) {
		console.error('Failed to retrieve transaction data:', error);
	}
}

main();
