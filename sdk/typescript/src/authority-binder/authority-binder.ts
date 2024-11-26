// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ethers } from 'ethers';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { getDWalletBinderByID, stringToArrayU8Bcs } from "../eth-light-client/utils.js";
import { getSharedObjectRefById } from '../utils/sui-types.js';
import { toPaddedBigEndianBytes } from '../zklogin/utils.js';

const packageId = '0x3';
const authorityBinderModuleName = 'authority_binder';

/**
 * Creates an authority acknowledgment transaction hash.
 *
 * This function constructs a transaction block to call the `create_authority_ack_transaction_hash`
 * function in the authority binder module.
 *
 * @param {string} dwalletBinderId - The ID of the dWallet binder.
 * @param {boolean} virginBound - Indicates if this is a virgin binding.
 * @param {number} chainID - The chain ID of the Ethereum network.
 * @param {'Number' | 'HexString'} chainIDType - The type of the chain ID.
 * @param {string} domainName - The domain name for the transaction.
 * @param {string} domainVersion - The domain version for the transaction.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 * @returns The transaction hash as a hexadecimal string.
 * @throws Will throw an error if the transaction fails to verify the Ethereum state.
 */
export const createAuthorityEIP712Acknowledgement = async (
	dwalletBinderId: string,
	virginBound: boolean,
	chainID: string,
	chainIDType: 'Number' | 'HexString',
	chainType: string,
	domainName: string,
	domainVersion: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let domainNameBcs = stringToArrayU8Bcs(domainName);
	let domainVersionBcs = stringToArrayU8Bcs(domainVersion);
	let chainTypeBcs = stringToArrayU8Bcs(chainType);

	let chainIdTypeArg = chainIDType === 'Number' ? 0 : 1;
	let chainIdBcs;
	if (chainIdTypeArg === 0) {
		let chainIdArg = toPaddedBigEndianBytes(BigInt(chainID), 32).slice(1);
		chainIdBcs = bcs.vector(bcs.u8()).serialize(chainIdArg);
	} else {
		chainIdBcs = stringToArrayU8Bcs(chainID);
	}

	let binderSharedObjRef = await getSharedObjectRefById(dwalletBinderId, client);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_authority_ack_transaction_hash`,
		arguments: [
			tx.sharedObjectRef(binderSharedObjRef),
			tx.pure.bool(virginBound),
			tx.pure(chainIdBcs),
			tx.pure(domainNameBcs),
			tx.pure(domainVersionBcs),
			tx.pure(chainTypeBcs),
		],
		typeArguments: [],
	});

	await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	let res = await client.devInspectTransactionBlock({
		sender: keypair.toSuiAddress(),
		transactionBlock: tx,
	});

	const array = new Uint8Array(res.results?.at(0)?.returnValues?.at(0)?.at(0)! as number[]);
	return ethers.hexlify(array);
};

export const sendSuiBindingTransaction = async (
	dWalletBinderId: string,
	virginEthDwalletCapId: string,
	message: Uint8Array,
	signature: Uint8Array,
	publicKey: Uint8Array,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let binderObject = await getDWalletBinderByID(dWalletBinderId, client);
	let bindToAuthority = binderObject?.bind_to_authority!;
	let bindToAuthorityId = bindToAuthority.id!.id;
	let bindToAuthorityNonce = bindToAuthority.nonce!;
	let virginBound = binderObject?.virgin_bound!;

	let sharedBinderObj = await getSharedObjectRefById(dWalletBinderId, client);
	// let dWalletBinderIdBcs = stringToArrayU8Bcs(dWalletBinderId);
	// let virginEthDwalletCapIdBcs = stringToArrayU8Bcs(virginEthDwalletCapId);
	// let bindToAuthorityIdBcs = stringToArrayU8Bcs(bindToAuthorityId);
	let messageBcs = bcs.vector(bcs.u8()).serialize(message);
	let signatureBcs = bcs.vector(bcs.u8()).serialize(signature);
	let publicKeyBcs = bcs.vector(bcs.u8()).serialize(publicKey);
	// let publicKey = keypair.getPublicKey().toSuiBytes();

	let tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::bind_dwallet_to_authority`,
		arguments: [
			tx.sharedObjectRef(sharedBinderObj),
			tx.pure.id(dWalletBinderId),
			tx.pure.id(virginEthDwalletCapId),
			tx.pure.id(bindToAuthorityId),
			tx.pure.u64(bindToAuthorityNonce),
			tx.pure.bool(virginBound),
			tx.pure(messageBcs),
			tx.pure(signatureBcs),
			tx.pure(publicKeyBcs),
		],
		typeArguments: [],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (result.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to verify Ethereum state. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}
};

/**
 * Creates a `BindToAuthority` object on the blockchain.
 *
 * This function constructs a transaction block to call the `create_bind_to_authority`
 * function in the authority binder module.
 *
 * @param {string} authorityId - The ID of the authority to bind to.
 * @param {string} ownerAddress - The address of the owner.
 * @param {number} ownerType - The type of the owner (e.g., user, contract).
 * @param {string} configType - The configuration type.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 * @returns The ObjectID of the created binding.
 * @throws Will throw an error if the transaction fails to verify the Ethereum state.
 */
export const createBindToAuthority = async (
	authorityId: string,
	ownerAddress: string,
	ownerType: number,
	configType: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let ownerAddressArrayU8 = ethers.getBytes(ownerAddress);
	let ownerAddressBcs = bcs.vector(bcs.u8()).serialize(ownerAddressArrayU8);
	let authoritySharedObjectRef = await getSharedObjectRefById(authorityId, client);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_bind_to_authority`,
		arguments: [
			tx.sharedObjectRef(authoritySharedObjectRef),
			tx.pure(ownerAddressBcs),
			tx.pure.u8(ownerType),
		],
		typeArguments: [configType],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (result.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to verify Ethereum state. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}

	return result.effects?.created?.at(0)?.reference.objectId!;
};

/**
 * Binds an `Authority` to an existing `DWalletBinder`.
 *
 * @param {string} binderID - The ID of the binder.
 * @param {string} authorityID - The ID of the authority to bind.
 * @param {string} authorityConfigType - The configuration type of the authority.
 * @param {string} owner - The address of the owner.
 * @param {number} ownerType - The type of the owner (e.g., user, contract).
 * @param {Keypair} keypair - The keypair used for signing the transaction.
 * @param {DWalletClient} client - The dWallet client to interact with the blockchain.
 * @returns The object ID of the dWalletBinder.
 * @throws Will throw an error if the transaction fails.
 */
export const setBindToAuthority = async (
	binderID: string,
	authorityID: string,
	authorityConfigType: string,
	owner: string,
	ownerType: number,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let binderSharedObjectRef = await getSharedObjectRefById(binderID, client, true);
	let authoritySharedObjectRef = await getSharedObjectRefById(authorityID, client);
	let ownerBcs = stringToArrayU8Bcs(owner);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::set_bind_to_authority`,
		arguments: [
			tx.sharedObjectRef(binderSharedObjectRef),
			tx.sharedObjectRef(authoritySharedObjectRef),
			tx.pure(ownerBcs),
			tx.pure.u8(ownerType),
		],
		typeArguments: [authorityConfigType],
	});

	let result = await client.signAndExecuteTransactionBlock({
		signer: keypair,
		transactionBlock: tx,
		options: { showEffects: true },
	});

	if (result.effects?.status.status !== 'success') {
		throw new Error(
			'Failed to verify Ethereum state. Transaction effects: ' + JSON.stringify(result.effects),
		);
	}

	return result.effects?.mutated?.filter(
		(o) =>
			typeof o.owner === 'object' &&
			'Shared' in o.owner &&
			o.owner.Shared.initial_shared_version !== undefined,
	)[0].reference.objectId!;
};