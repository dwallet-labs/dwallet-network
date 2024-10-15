// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ethers } from 'ethers';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { stringToArrayU8Bcs } from '../eth-light-client/utils.js';
import type { OwnedObjectRef } from '../types/objects.js';

const packageId = '0x3';
const authorityBinderModuleName = 'authority_binder';

export const createConfig = async (keypair: Keypair, client: DWalletClient) => {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_config`,
		arguments: [],
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

	return result.effects?.created?.at(0);
};

export const createAuthorityAckTransactionHash = async (
	binderObjRef: OwnedObjectRef,
	virginBound: boolean,
	chainID: number,
	domainName: string,
	domainVersion: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let domainNameBcs = stringToArrayU8Bcs(domainName);
	let domainVersionBcs = stringToArrayU8Bcs(domainVersion);

	let binderSharedObjRef = getSharedObjectRefFromOwner(binderObjRef);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_authority_ack_transaction_hash`,
		arguments: [
			tx.sharedObjectRef(binderSharedObjRef),
			tx.pure.bool(virginBound),
			tx.pure.u64(chainID),
			tx.pure(domainNameBcs),
			tx.pure(domainVersionBcs),
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
	const hexString = Array.from(array)
		.map((byte) => byte.toString(16).padStart(2, '0'))
		.join('');
	return hexString;

	// todo(yuval): make sure to update nonce on chain if needed
};

/**
 * Creates an authority object on the blockchain.
 *
 * @param {string} binderName - The name of the binder.
 * @param {string} chainIdentifier - A unique identifier for the chain.
 * @param {OwnedObjectRef} latestSnapshotOwnerObjRef - The reference to the latest snapshot object.
 * @param {string} latestSnapshotObjType - The type of the latest snapshot object.
 * @param {OwnedObjectRef} configObjRef - The reference to the configuration object.
 * @param {string} configObjType - The type of the configuration object.
 * @param {string} authorityOwnerDWalletCapID - The ID of the dWallet capability for the authority owner.
 * @param {Keypair} keypair - The keypair used for signing the transaction.
 * @param {DWalletClient} client - The dWallet client to interact with the blockchain.
 * @returns The created authority object.
 * @throws Will throw an error if the transaction fails.
 */
export const createAuthority = async (
	binderName: string,
	chainIdentifier: string,
	latestSnapshotOwnerObjRef: OwnedObjectRef,
	latestSnapshotObjType: string,
	configObjRef: OwnedObjectRef,
	configObjType: string,
	authorityOwnerDWalletCapID: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let uniqueIdentifierBcs = stringToArrayU8Bcs(chainIdentifier);

	let latestSnapshotSharedObjRef = getSharedObjectRefFromOwner(latestSnapshotOwnerObjRef);
	let configSharedObjRef = getSharedObjectRefFromOwner(configObjRef);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_authority`,
		arguments: [
			tx.pure.string(binderName),
			tx.pure(uniqueIdentifierBcs),
			tx.sharedObjectRef(latestSnapshotSharedObjRef),
			tx.sharedObjectRef(configSharedObjRef),
			tx.object(authorityOwnerDWalletCapID),
		],
		typeArguments: [latestSnapshotObjType, configObjType],
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

	return result.effects?.created?.at(0)!;
};

/**
 * Creates an authority binder on the blockchain.
 *
 * @param {string} dWalletCapID - The ID of the dWallet capability.
 * @param {OwnedObjectRef} authorityOwnerObjRef - The reference to the authority object.
 * @param {boolean} virginBound - Whether this is a virgin binding.
 * @param {string} ownerAddress - The address of the owner.
 * @param {number} ownerType - The type of the owner (e.g., user, contract).
 * @param {Keypair} keypair - The keypair used for signing the transaction.
 * @param {DWalletClient} client - The dWallet client to interact with the blockchain.
 * @returns The created binder object.
 * @throws Will throw an error if the transaction fails.
 */
export const createAuthorityBinder = async (
	dWalletCapID: string,
	authorityOwnerObjRef: OwnedObjectRef,
	virginBound: boolean,
	ownerAddress: string,
	ownerType: number,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let ownerAddressArrayU8 = ethers.getBytes(ownerAddress);
	let ownerAddressBcs = bcs.vector(bcs.u8()).serialize(ownerAddressArrayU8);

	let authoritySharedObjRef = getSharedObjectRefFromOwner(authorityOwnerObjRef);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_binder`,
		arguments: [
			tx.object(dWalletCapID),
			tx.sharedObjectRef(authoritySharedObjRef),
			tx.pure(ownerAddressBcs),
			tx.pure.u8(ownerType),
			tx.pure.bool(virginBound),
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

	return result.effects?.created?.at(0)!;
};

/**
 * Binds an authority to an existing binder.
 *
 * @param {string} binderID - The ID of the binder.
 * @param {string} authorityID - The ID of the authority to bind.
 * @param {string} owner - The address of the owner.
 * @param {number} ownerType - The type of the owner (e.g., user, contract).
 * @param {Keypair} keypair - The keypair used for signing the transaction.
 * @param {DWalletClient} client - The dWallet client to interact with the blockchain.
 * @returns The object ID of the newly bound authority.
 * @throws Will throw an error if the transaction fails.
 */
// todo(yuval): test this function
export const setBindToAuthority = async (
	binderID: string,
	authorityID: string,
	owner: string,
	ownerType: number,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let authorityIDBcs = stringToArrayU8Bcs(authorityID);
	let ownerBcs = stringToArrayU8Bcs(owner);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::set_bind_to_authority`,
		arguments: [
			tx.object(binderID),
			tx.pure(authorityIDBcs),
			tx.pure(ownerBcs),
			tx.pure.u64(ownerType),
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

	return result.effects?.created?.at(0)?.reference.objectId!;
};

function getSharedObjectRefFromOwner(ownerObjRef: OwnedObjectRef) {
	let owner = ownerObjRef.owner;
	const initialSharedVersion =
		owner && typeof owner === 'object' && 'Shared' in owner
			? owner.Shared.initial_shared_version!
			: undefined;

	if (initialSharedVersion === undefined) {
		throw new Error('Failed to create authority: owner is not a shared object');
	}

	return {
		objectId: ownerObjRef.reference.objectId,
		initialSharedVersion: initialSharedVersion,
		mutable: false,
	};
}
