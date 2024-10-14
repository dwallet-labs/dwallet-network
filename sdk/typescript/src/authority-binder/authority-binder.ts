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

	// todo(yuval): make sure that the objects are shared objects
	let binderSharedObjRef = {
		objectId: binderObjRef.reference.objectId,
		initialSharedVersion: binderObjRef.owner.Shared.initial_shared_version,
		mutable: false,
	};

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::authority_binder::create_authority_ack_transaction_hash`,
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

export const createAuthority = async (
	binderName: string,
	chainIdentifier: string,
	latestSnapshotObjRef: OwnedObjectRef,
	configObjRef: OwnedObjectRef,
	authorityOwnerDWalletCapID: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let uniqueIdentifierBcs = stringToArrayU8Bcs(chainIdentifier);

	// todo(yuval): make sure that the objects are shared objects
	let latestSnapshotSharedObjRef = {
		objectId: latestSnapshotObjRef.reference.objectId,
		initialSharedVersion: latestSnapshotObjRef.owner.Shared.initial_shared_version,
		mutable: false,
	};

	let configSharedObjRef = {
		objectId: configObjRef.reference.objectId,
		initialSharedVersion: configObjRef.owner.Shared.initial_shared_version,
		mutable: false,
	};

	const tx = new TransactionBlock();
	let check = tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_authority`,
		arguments: [
			tx.pure.string(binderName),
			tx.pure(uniqueIdentifierBcs),
			tx.sharedObjectRef(latestSnapshotSharedObjRef),
			tx.sharedObjectRef(configSharedObjRef),
			tx.object(authorityOwnerDWalletCapID),
		],
		typeArguments: [
			`${packageId}::authority_binder::Config`,
			`${packageId}::ethereum_state::LatestEthereumState`,
		],
	});

	if (check === undefined) {
		throw new Error('Failed to create authority');
	}

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

export const createAuthorityBinder = async (
	dWalletCapID: string,
	authorityObjRef: OwnedObjectRef,
	virginBound: boolean,
	ownerAddress: string,
	ownerType: number,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let ownerAddressArrayU8 = ethers.getBytes(ownerAddress);
	let ownerAddressBcs = bcs.vector(bcs.u8()).serialize(ownerAddressArrayU8);

	// todo(yuval): make sure that the objects are shared objects
	let authoritySharedObjRef = {
		objectId: authorityObjRef.reference.objectId,
		initialSharedVersion: authorityObjRef.owner.Shared.initial_shared_version,
		mutable: false,
	};

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
