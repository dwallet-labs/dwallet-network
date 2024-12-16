// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ethers } from 'ethers';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder';
import type { DWalletClient } from '../client';
import type { Keypair } from '../cryptography';
import { stringToBcs, getSharedObjectRefById } from '../utils/light-clients';

const packageId = '0x3';
const authorityBinderModuleName = 'authority_binder';

/**
 * Creates an authority acknowledgment transaction hash.
 *
 * This function constructs a transaction block to call the `create_authority_ack`
 * function in the authority binder module.
 *
 * @param {string} dwalletBinderId - The ID of the dWallet binder.
 * @param {boolean} virginBound - Indicates if this is a virgin binding.
 * @param {number} chainID - The chain ID of the Ethereum network.
 * @param {string} chainType - The type of the chain, e.g., "Ethereum", "Sui".
 * @param {string} domainName - The domain name for the transaction.
 * @param {string} domainVersion - The domain version for the transaction.
 * @param {Keypair} keypair - The keypair used to sign the transaction.
 * @param {DWalletClient} client - The dWallet client instance.
 * @returns The transaction hash as a hexadecimal string.
 * @throws Will throw an error if the transaction fails to verify the Ethereum state.
 */
export const createAuthorityAck = async (
	dwalletBinderId: string,
	virginBound: boolean,
	chainID: bigint,
	chainType: string,
	domainName: string,
	domainVersion: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let domainNameBcs = stringToBcs(domainName);
	let domainVersionBcs = stringToBcs(domainVersion);
	let chainTypeBcs = stringToBcs(chainType);

	let binderSharedObjRef = await getSharedObjectRefById(dwalletBinderId, client);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_authority_ack`,
		arguments: [
			tx.sharedObjectRef(binderSharedObjRef),
			tx.pure.bool(virginBound),
			tx.pure.u64(chainID),
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
	let ownerBcs = stringToBcs(owner);

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