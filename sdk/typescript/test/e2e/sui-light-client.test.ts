// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// import { recovery_id_keccak256 } from '@dwallet-network/signature-mpc-wasm';

import { recovery_id_keccak256 as recoveryIdKeccak256 } from '@dwallet-network/signature-mpc-wasm';
import { recovery_id_sha256 as recoveryIdSha256 } from '@dwallet-network/signature-mpc-wasm';
import { SuiClient } from '@mysten/sui.js/client';
import { TransactionBlock } from '@mysten/sui.js/transactions';
import { ethers, SigningKey } from 'ethers';
import { assert, beforeAll, describe, it } from 'vitest';

import {
	createAuthorityEIP712Acknowledgement,
	createBindToAuthority,
	sendSuiBindingTransaction,
} from '../../src/authority-binder';
import { requestSuiFromFaucetV0 } from '../../src/faucet';
import {
	approveAndSign,
	// approveAndSign,
	// approveAndSignWithBinder,
	approveMessageInSui,
	createActiveEncryptionKeysTable,
	createDWallet,
	createPartialUserSignedMessages,
	createSuiDWalletCap,
	createVirginBoundDWallet,
	getDwalletByObjID,
	submitDWalletCreationProof,
	submitTxStateProof,
} from '../../src/signature-mpc';
import { getOrCreateEncryptionKey } from '../../src/signature-mpc/encrypt_user_share';
import { presignWithDWalletID } from '../../src/signature-mpc/sign';
import { setup, TestToolbox } from './utils/setup';

describe('Test Ethereum Light Client', () => {
	let toolbox: TestToolbox;
	let authorityToolbox: TestToolbox;
	let activeEncryptionKeysTableID: string;

	const packageId = '0x3';
	const suiStateModuleName = 'sui_state_proof';
	const dWalletCapPackageSUI = '0x96c235dfd098a3e0404cfe5bf9c05bbc268b75649d051d4808019f5eb81d3eec';

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
		// 	host: 'https://faucet.testnet.sui.io',
		// 	recipient: toolbox.keypair.toSuiAddress(),
		// });
	});

	it('should init the state, create ethereum dwallet, and verify a message', async () => {
		let encryptionKeyObj = await getOrCreateEncryptionKey(
			toolbox.keypair,
			toolbox.client,
			activeEncryptionKeysTableID,
		);

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

		const serviceUrl = 'http://localhost:6920/gettxdata';
		// const contractAddress2 = '0x4a22eaef6ba256D46Fb7935B1bdAd8cEb454EFCd';
		const contractAddress = '0xEd34EE41cA84042b619E9AEBF6175bB4a0069a05'; // remix IDE address
		const domainName = 'dWalletAuthenticator';
		const domainVersion = '1.0.0';
		const virginBound = true;
		const chainIdentifier = '0x55740dca4b9cedc152ec045f5184f58dea8f740fe1622bb9c416fb62a91095fd';

		let authorityId = '0xfaacfd76aab0de938473de461b90a79f8a2d30f4b0c5a40cbd3e604821292d47';

		// create bind to authority
		let configType = `${packageId}::${suiStateModuleName}::StateProofConfig`;
		let bindToAuthorityId = await createBindToAuthority(
			authorityId,
			contractAddress,
			0,
			configType,
			toolbox.keypair,
			toolbox.client,
		);

		await new Promise((r) => setTimeout(r, 2000));

		// create virgin ethereum dwallet for user
		const virginEthDwallet = await createVirginBoundDWallet(
			encryptionKeyObj.encryptionKey,
			encryptionKeyObj.objectID,
			bindToAuthorityId,
			toolbox.keypair,
			toolbox.client,
		);

		const virginEthDwalletId = virginEthDwallet?.dwalletID!;
		const dWalletBinderId = virginEthDwallet?.dWalletBinderID!;
		const virginEthDwalletCapId = virginEthDwallet?.dwalletCapID!;

		const suiTestnetURL = 'https://fullnode.testnet.sui.io:443';
		let suiClient = new SuiClient({ url: suiTestnetURL });
		// create sui dwallet cap
		let { createDWalletTxDigest, suiDWalletCapId } = await createSuiDWalletCap(
			virginEthDwalletCapId,
			dWalletCapPackageSUI,
			suiClient,
			toolbox.keypair,
		);

		await new Promise((r) => setTimeout(r, 6000));

		const messageToSign = 'dWallets are coming...';
		let approveTxDigest = await approveMessageInSui(
			suiDWalletCapId,
			messageToSign,
			dWalletCapPackageSUI,
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
		const messageEncoded = new TextEncoder().encode(messageToSign);
		// const messageEncoded = ethers.getBytes(messageToSign);
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

		// submit tx state proof to dwallet -> Message Approval
		let res = await submitTxStateProof(
			authorityToolbox.client,
			suiClient,
			authorityId,
			dWalletBinderId,
			signMessagesIdSHA256,
			approveTxDigest,
			virginEthDwallet?.dwalletID!,
			serviceUrl,
			authorityToolbox.keypair,
		);

		// create sponsored transaction for binding

		let transactionHash = await createAuthorityEIP712Acknowledgement(
			dWalletBinderId,
			true,
			chainIdentifier,
			'HexString', // Chain ID type
			'Sui', // Chain Type
			domainName,
			domainVersion,
			authorityToolbox.keypair,
			authorityToolbox.client,
		);
		assert(transactionHash !== undefined);

		// sign the transaction using the authority keypair, then send to sui for validation.

		// raw bytes
		const message: Uint8Array = ethers.getBytes(transactionHash);
		// sign the transaction hash
		let presignObjID = await presignWithDWalletID(
			authorityToolbox.client,
			authorityToolbox.keypair,
			authorityOwnerDWallet?.dwalletID!,
			message,
			'SHA256',
			activeEncryptionKeysTableID,
		);
		// todo(yuval): this function is modified. need to create a new one instead.
		let signatures = await approveAndSign(
			authorityId,
			presignObjID!,
			[message],
			authorityOwnerDWallet?.dwalletID!,
			'SHA256',
			authorityToolbox.keypair,
			authorityToolbox.client,
		);

		const dwalletObj = await getDwalletByObjID(
			authorityToolbox.client,
			authorityOwnerDWallet?.dwalletID!,
		);
		const publicKey = Buffer.from(dwalletObj?.publicKey!);

		// let publicKey = Buffer.from(authorityToolbox.keypair.getPublicKey().toRawBytes());
		let recoveryId = recoveryIdSha256(publicKey, message, signatures[0]!);
		let signature = new Uint8Array([...signatures[0], recoveryId]);
		// let sigHex = Buffer.from(signature).toString('hex');

		await sendSuiBindingTransaction(
			dWalletBinderId,
			virginEthDwalletCapId,
			message,
			signature,
			publicKey,
			authorityToolbox.keypair,
			authorityToolbox.client,
		);

		console.log(`dwallet binder id: ` + dWalletBinderId);
		console.log(`dwallet cap id: ` + virginEthDwallet?.dwalletCapID!);
		console.log(`bind to authority id: ` + bindToAuthorityId);
		console.log(`virgin bound:` + virginBound);
		console.log(`nonce:` + 123);
		let sig = Buffer.from(signatures[0]!).toString('hex');

		// const dwalletObj = await getDwalletByObjID(
		// 	authorityToolbox.client,
		// 	virginEthDwallet?.dwalletID!,
		// );
		// const publicKey = Buffer.from(dwalletObj?.publicKey!);

		// let recoveryId = recoveryIdKeccak256(publicKey, message, signatures[0]!);
		// recoveryId += 27;

		// const signature = '0x' + sig + recoveryId.toString(16);
		let ethereumAddress = deriveEthereumAddress(publicKey);

		// send the signed transaction hash to the Ethereum network

		// For this part to work, you need to wait until the block that includes the transaction
		// we want to verify, is FINALIZED (takes between 13-20 minutes).
		// message = 'U3VwcmlzZSEgSGF2ZSBhIGdyZWF0IGRheSE=';
		const provider = ethers.getDefaultProvider('http://localhost:8545');
		const privateKey = '0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80'; // Keep this secure!
		const wallet = new ethers.Wallet(privateKey, provider);
		const contractAbi = [
			{
				inputs: [],
				stateMutability: 'nonpayable',
				type: 'constructor',
			},
			{
				inputs: [],
				name: 'AccessControlBadConfirmation',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'address',
						name: 'account',
						type: 'address',
					},
					{
						internalType: 'bytes32',
						name: 'neededRole',
						type: 'bytes32',
					},
				],
				name: 'AccessControlUnauthorizedAccount',
				type: 'error',
			},
			{
				inputs: [],
				name: 'ECDSAInvalidSignature',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'uint256',
						name: 'length',
						type: 'uint256',
					},
				],
				name: 'ECDSAInvalidSignatureLength',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 's',
						type: 'bytes32',
					},
				],
				name: 'ECDSAInvalidSignatureS',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'address',
						name: 'add',
						type: 'address',
					},
				],
				name: 'InvalidAddress',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'dWalletID',
						type: 'bytes32',
					},
				],
				name: 'InvalidDWalletIDInput',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'dWalletID',
						type: 'bytes32',
					},
					{
						internalType: 'address',
						name: 'ownerID',
						type: 'address',
					},
				],
				name: 'InvalidInput',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'messageHash',
						type: 'bytes32',
					},
				],
				name: 'InvalidMessageHash',
				type: 'error',
			},
			{
				inputs: [],
				name: 'InvalidShortString',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'dWalletID',
						type: 'bytes32',
					},
					{
						internalType: 'address',
						name: 'ownerID',
						type: 'address',
					},
				],
				name: 'NotDwalletOwner',
				type: 'error',
			},
			{
				inputs: [
					{
						internalType: 'string',
						name: 'str',
						type: 'string',
					},
				],
				name: 'StringTooLong',
				type: 'error',
			},
			{
				anonymous: false,
				inputs: [],
				name: 'EIP712DomainChanged',
				type: 'event',
			},
			{
				anonymous: false,
				inputs: [
					{
						indexed: false,
						internalType: 'bytes32',
						name: 'messageHash',
						type: 'bytes32',
					},
				],
				name: 'MessageApproved',
				type: 'event',
			},
			{
				anonymous: false,
				inputs: [
					{
						indexed: false,
						internalType: 'bytes32',
						name: 'dWalletID',
						type: 'bytes32',
					},
					{
						indexed: false,
						internalType: 'address',
						name: 'newOwner',
						type: 'address',
					},
				],
				name: 'NewDWalletOwner',
				type: 'event',
			},
			{
				anonymous: false,
				inputs: [
					{
						indexed: true,
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
					{
						indexed: true,
						internalType: 'bytes32',
						name: 'previousAdminRole',
						type: 'bytes32',
					},
					{
						indexed: true,
						internalType: 'bytes32',
						name: 'newAdminRole',
						type: 'bytes32',
					},
				],
				name: 'RoleAdminChanged',
				type: 'event',
			},
			{
				anonymous: false,
				inputs: [
					{
						indexed: true,
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
					{
						indexed: true,
						internalType: 'address',
						name: 'account',
						type: 'address',
					},
					{
						indexed: true,
						internalType: 'address',
						name: 'sender',
						type: 'address',
					},
				],
				name: 'RoleGranted',
				type: 'event',
			},
			{
				anonymous: false,
				inputs: [
					{
						indexed: true,
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
					{
						indexed: true,
						internalType: 'address',
						name: 'account',
						type: 'address',
					},
					{
						indexed: true,
						internalType: 'address',
						name: 'sender',
						type: 'address',
					},
				],
				name: 'RoleRevoked',
				type: 'event',
			},
			{
				anonymous: false,
				inputs: [
					{
						indexed: false,
						internalType: 'bytes32',
						name: 'dWalletID',
						type: 'bytes32',
					},
					{
						indexed: false,
						internalType: 'address',
						name: 'oldOwner',
						type: 'address',
					},
					{
						indexed: false,
						internalType: 'address',
						name: 'newOwner',
						type: 'address',
					},
				],
				name: 'TransferWalletOwnership',
				type: 'event',
			},
			{
				inputs: [],
				name: 'DEFAULT_ADMIN_ROLE',
				outputs: [
					{
						internalType: 'bytes32',
						name: '',
						type: 'bytes32',
					},
				],
				stateMutability: 'view',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'transactionHash',
						type: 'bytes32',
					},
					{
						internalType: 'bytes32',
						name: 'dWalletID',
						type: 'bytes32',
					},
				],
				name: 'approveMessage',
				outputs: [],
				stateMutability: 'nonpayable',
				type: 'function',
			},
			{
				inputs: [
					{
						components: [
							{
								internalType: 'bytes',
								name: 'id',
								type: 'bytes',
							},
							{
								internalType: 'bytes',
								name: 'signature',
								type: 'bytes',
							},
						],
						internalType: 'struct DWalletAuthenticator.DWalletBinder',
						name: 'binder',
						type: 'tuple',
					},
				],
				name: 'bindDWallet',
				outputs: [],
				stateMutability: 'view',
				type: 'function',
			},
			{
				inputs: [],
				name: 'eip712Domain',
				outputs: [
					{
						internalType: 'bytes1',
						name: 'fields',
						type: 'bytes1',
					},
					{
						internalType: 'string',
						name: 'name',
						type: 'string',
					},
					{
						internalType: 'string',
						name: 'version',
						type: 'string',
					},
					{
						internalType: 'uint256',
						name: 'chainId',
						type: 'uint256',
					},
					{
						internalType: 'address',
						name: 'verifyingContract',
						type: 'address',
					},
					{
						internalType: 'bytes32',
						name: 'salt',
						type: 'bytes32',
					},
					{
						internalType: 'uint256[]',
						name: 'extensions',
						type: 'uint256[]',
					},
				],
				stateMutability: 'view',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
				],
				name: 'getRoleAdmin',
				outputs: [
					{
						internalType: 'bytes32',
						name: '',
						type: 'bytes32',
					},
				],
				stateMutability: 'view',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
					{
						internalType: 'address',
						name: 'account',
						type: 'address',
					},
				],
				name: 'grantRole',
				outputs: [],
				stateMutability: 'nonpayable',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
					{
						internalType: 'address',
						name: 'account',
						type: 'address',
					},
				],
				name: 'hasRole',
				outputs: [
					{
						internalType: 'bool',
						name: '',
						type: 'bool',
					},
				],
				stateMutability: 'view',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
					{
						internalType: 'address',
						name: 'callerConfirmation',
						type: 'address',
					},
				],
				name: 'renounceRole',
				outputs: [],
				stateMutability: 'nonpayable',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'role',
						type: 'bytes32',
					},
					{
						internalType: 'address',
						name: 'account',
						type: 'address',
					},
				],
				name: 'revokeRole',
				outputs: [],
				stateMutability: 'nonpayable',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes4',
						name: 'interfaceId',
						type: 'bytes4',
					},
				],
				name: 'supportsInterface',
				outputs: [
					{
						internalType: 'bool',
						name: '',
						type: 'bool',
					},
				],
				stateMutability: 'view',
				type: 'function',
			},
			{
				inputs: [
					{
						internalType: 'bytes32',
						name: 'dWalletID',
						type: 'bytes32',
					},
					{
						internalType: 'address',
						name: 'newOwnerID',
						type: 'address',
					},
				],
				name: 'transferOwnership',
				outputs: [],
				stateMutability: 'nonpayable',
				type: 'function',
			},
		];
		const contract = new ethers.Contract(contractAddress, contractAbi, wallet);
		const contractWithSigner = contract.connect(wallet);
		//
		// let dwalletObj = await getDwalletByObjID(
		// 	authorityToolbox.client,
		// 	authorityOwnerDWallet?.dwalletID!,
		// );
		// let publicKey = Uint8Array.from(Buffer.from(dwalletObj?.publicKey!));
		// const recoveryId = '0' + recovery_id_keccak256(publicKey, message, signatures[0]!).toString(16);
		// const signature = '0x' + sig + recoveryId;
		// let ethereumAddress = deriveEthereumAddress(publicKey);
		let dwalletBinder = {
			id: ethers.getBytes(dWalletBinderId),
			signature: ethers.getBytes(signature),
		};

		const bindDWallet = contractWithSigner.getFunction('bindDWallet');
		const txResponse = await bindDWallet.send(dwalletBinder, {
			gasLimit: 3000000,
		});

		let receipt = await txResponse.wait();
		console.log(receipt);

		// let messageApprovalBcs = await approveEthereumMessage(
		// 	authorityId,
		// 	dWalletBinderId,
		// 	message,
		// 	dwalletId,
		// 	executionRpc,
		// 	consensusRpc,
		// 	toolbox.keypair,
		// 	toolbox.client,
		// );
		//
		// assert(messageApprovalBcs !== undefined);
	});
});

/**
 * Derive a 20-byte Ethereum address from a given public key.
 * @returns The Ethereum address as a string (20 bytes, hex format)
 * @param compressedPubKeyBytes
 */
function deriveEthereumAddress(compressedPubKeyBytes: Uint8Array): string {
	// const pubKeyHex = Buffer.from(publicKey).toString('hex');
	const compressedPubKeyHex = '0x' + Buffer.from(compressedPubKeyBytes).toString('hex');
	if (!/^0x0[23]/.test(compressedPubKeyHex)) {
		throw new Error('Invalid compressed public key format');
	}
	const uncompressedPubKeyHex = SigningKey.computePublicKey(compressedPubKeyHex, false);
	const uncompressedPubKeyWithoutPrefix = uncompressedPubKeyHex.slice(4); // Remove '0x04'
	const uncompressedPubKeyBytes = ethers.getBytes('0x' + uncompressedPubKeyWithoutPrefix);
	const pubKeyHash = ethers.keccak256(uncompressedPubKeyBytes);
	return ethers.getAddress('0x' + pubKeyHash.slice(-40));
}
