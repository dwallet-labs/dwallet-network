// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ethers } from 'ethers';

import { bcs } from '../bcs/index.js';
import { TransactionBlock } from '../builder/index.js';
import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';
import { stringToArrayU8Bcs } from '../eth-light-client/utils.js';

const packageId = '0x3';
const authorityBinderModuleName = 'authority_binder';

// todo(yuval): update this function, now it's just garbage code
export const createAuthorityAckTransactionHash = async (
	binderID: string,
	authorityDWalletCapId: string,
	bindToAuthorityID: string,
	virginBound: boolean,
	chainID: string,
	domainName: string,
	domainVersion: string,
	contractAddress: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let contractAddressArrayU8 = ethers.getBytes(contractAddress);
	let contractAddressBcs = bcs.vector(bcs.u8()).serialize(contractAddressArrayU8);
	let domainNameArrayU8 = ethers.getBytes(domainName);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::binder::create_authority_ack_transaction_hash`,
		arguments: [
			tx.object(binderID),
			tx.object(authorityDWalletCapId),
			tx.object(bindToAuthorityID),
			tx.pure.bool(virginBound),
			tx.pure.u64(chainID),
			tx.pure(domainNameArrayU8),
			tx.pure.u64(domainVersion),
			tx.pure(contractAddressBcs),
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

	// todo(yuval): update nonce on chain if needed

};

export const createAuthority = async (
	binderName: string,
	chainIdentifier: string,
	latestSnapshot: string, // todo(yuval): update to snapshot type
	config: string, // todo(yuval): update to config type
	authorityOwnerDWalletCapID: string,
	keypair: Keypair,
	client: DWalletClient,
) => {
	let binderNameBcs = stringToArrayU8Bcs(binderName);
	let uniqueIdentifierBcs = stringToArrayU8Bcs(chainIdentifier);
	let latestSnapshotBcs = stringToArrayU8Bcs(latestSnapshot);
	let configBcs = stringToArrayU8Bcs(config);
	let authorityOwnerDWalletCapIDBcs = stringToArrayU8Bcs(authorityOwnerDWalletCapID);

	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_authority`,
		arguments: [
			tx.pure(binderNameBcs),
			tx.pure(uniqueIdentifierBcs),
			tx.pure(latestSnapshotBcs),
			tx.pure(configBcs),
			tx.pure(authorityOwnerDWalletCapIDBcs),
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

export const createAuthorityBinder = async (
	dWalletCapID: string,
	bindToAuthorityID: string,
	virginBound: boolean,
	keypair: Keypair,
	client: DWalletClient,
) => {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: `${packageId}::${authorityBinderModuleName}::create_binder`,
		arguments: [tx.object(dWalletCapID), tx.object(bindToAuthorityID), tx.pure.bool(virginBound)],
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

export const createBindToAuthority = async (
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
		target: `${packageId}::${authorityBinderModuleName}::create_bind_to_authority`,
		arguments: [tx.pure(authorityIDBcs), tx.pure(ownerBcs), tx.pure.u64(ownerType)],
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
