// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { ethers } from 'ethers';

import { bcs } from '../bcs/index.js';
import type { DWalletClient } from '../client/index.js';

/// The `EthereumState` Move object fields.
export type EthereumState = {
	id: object;
	data: number[];
	time_slot: number;
	authority_binder_id: string;
	state_root: number[];
	block_number: number;
	network: number[];
};

/**
 * Calculates the key for a given message and dWallet ID.
 * In the smart contract, the key is calculated by hashing the message and the dWallet ID together.
 * The result is a 32-byte hash represented as a hexadecimal string.
 * @param {Uint8Array} message - A Uint8Array representing the message to be stored.
 * @param {Uint8Array} dwalletId - A Uint8Array representing the dWallet ID.
 * @returns A string representing the calculated key (hexadecimal format).
 */
function calculateKey(message: Uint8Array, dwalletId: Uint8Array): string {
	const combined = ethers.concat([message, dwalletId]);
	return ethers.keccak256(combined);
}

/**
 * Calculates the mapping slot for a given key and storage slot in the contract's storage layout.
 * The key and slot are ABI-encoded and hashed together to produce a storage slot.
 * @param {string} key - A string (hexadecimal format) representing the key for which the mapping slot is to be calculated.
 * @param {number} mappingSlot - A BigInt value representing the mapping slot in the contract storage layout.
 * @returns A string representing the calculated storage slot (hexadecimal format).
 */
function calculateMappingSlotForKey(key: string, mappingSlot: number): string {
	const abiCoder = ethers.AbiCoder.defaultAbiCoder();
	const encoded = abiCoder.encode(['bytes32', 'uint256'], [key, mappingSlot]);
	return ethers.keccak256(encoded);
}

/**
 * Calculates the storage slot for a given message, dWallet ID, and data slot.
 * The function first calculates a key by hashing the message and the dWallet ID together.
 * Then, it calculates the mapping slot for the calculated key and the provided data slot.
 * The calculated mapping slot can be used to locate the (key, value) pair in the contract's storage.
 * @param {string} message - A string representing the message to be stored.
 * @param {string} dwalletId - A Uint8Array representing the dWallet ID.
 * @param {number} dataSlot - A BigInt value representing the data slot.
 * @returns A string representing the calculated storage slot (hexadecimal format).
 */
export function calculateMessageStorageSlot(
	message: string,
	dwalletId: string,
	dataSlot: number,
): string {
	const messageBytes = ethers.toUtf8Bytes(message);
	const dwalletIdBytes = ethers.getBytes(dwalletId);
	const key = calculateKey(messageBytes, dwalletIdBytes);
	return calculateMappingSlotForKey(key, dataSlot);
}

export const getAuthorityBinderByID = async (authorityBinderID: string, client: DWalletClient) => {
	let authorityBinderResponse = await client.getObject({
		id: authorityBinderID,
		options: { showContent: true },
	});

	if (authorityBinderResponse.data?.content?.dataType === 'moveObject') {
		const fields = authorityBinderResponse.data?.content?.fields as any;

		return {
			id: fields.id.id,
			dwallet_cap: parseNestedStruct(fields.dwallet_cap),
			bind_to_authority: parseNestedStruct(fields.bind_to_authority),
			virgin_bound: fields.virgin_bound,
		};
	}
	return null;
};

/**
 * Retrieves an Ethereum authority object by its ID.
 *
 * This function fetches the Ethereum authority object from the dWallet client using the provided authority ID.
 * It then parses the fields of the object and returns them in a structured format.
 *
 * @param {string} authorityID - The ObjectID of the Ethereum authority.
 * @param {DWalletClient} client - The dWallet client instance.
 * @returns An object containing the parsed fields of the Ethereum authority, or null if not found.
 */
export const getAuthorityByID = async (authorityID: string, client: DWalletClient) => {
	let authorityResponse = await client.getObject({
		id: authorityID,
		options: { showContent: true },
	});

	if (authorityResponse.data?.content?.dataType === 'moveObject') {
		const fields = authorityResponse.data?.content?.fields as any;

		return {
			id: fields.id.id,
			name: fields.name,
			unique_identifier: fields.unique_identifier,
			latest: parseNestedStruct(fields.latest),
			config: parseNestedStruct(fields.config),
			authority_owner_dwallet_cap: parseNestedStruct(fields.authority_owner_dwallet_cap),
		};
	}
	return null;
};

/**
 * Retrieves the Ethereum state object by its ID.
 *
 * @param {DWalletClient} client - The dWallet client instance.
 * @param {string | undefined} currentEthereumStateId - The ObjectID of the current Ethereum state.
 * @returns An object containing the Ethereum state fields, or null if not found.
 */
export const getEthereumStateById = async (
	client: DWalletClient,
	currentEthereumStateId: string | undefined,
) => {
	let currentEthereumStateResponse = await client.getObject({
		id: currentEthereumStateId as string,
		options: { showContent: true },
	});

	return currentEthereumStateResponse.data?.content?.dataType === 'moveObject'
		? (currentEthereumStateResponse.data?.content?.fields as EthereumState)
		: null;
};

/**
 * Converts a string to a Uint8Array and serializes it using BCS (Binary Canonical Serialization).
 *
 * @param {string} value - The string to convert and serialize.
 * @returns The serialized Uint8Array.
 */
export function stringToArrayU8Bcs(value: string) {
	let arrayU8 = Uint8Array.from(Array.from(value).map((c) => c.charCodeAt(0)));
	return bcs.vector(bcs.u8()).serialize(arrayU8, {
		size: arrayU8.length,
		maxSize: arrayU8.length * 2,
		allocateSize: arrayU8.length,
	});
}

/**
 * Compares two Uint8Arrays for equality.
 *
 * @param {Uint8Array} a - The first Uint8Array to compare.
 * @param {Uint8Array} b - The second Uint8Array to compare.
 * @returns {boolean} True if both arrays are equal, false otherwise.
 */
export function compareUint8Arrays(a: Uint8Array, b: Uint8Array): boolean {
	if (a === b) return true;
	if (a.length !== b.length) return false;
	for (let i = 0; i < a.length; i++) {
		if (a[i] !== b[i]) return false;
	}
	return true;
}

/**
 * Converts all keys in an object to snake_case recursively.
 *
 * @param {any} obj - The object to convert.
 * @returns {any} A new object with all keys converted to snake_case.
 */
export function keysToSnakeCase(obj: any): any {
	if (Array.isArray(obj)) {
		return obj.map((item) => keysToSnakeCase(item));
	} else if (obj !== null && typeof obj === 'object') {
		return Object.fromEntries(
			Object.entries(obj).map(([key, value]) => {
				const newKey = camelToSnake(key);
				return [newKey, keysToSnakeCase(value)];
			}),
		);
	} else {
		return obj;
	}
}

/**
 * Converts a camelCase string to snake_case.
 *
 * @param {string} key - The camelCase string to convert.
 * @returns {string} The converted snake_case string.
 */
function camelToSnake(key: string): string {
	return key.replace(/([A-Z])/g, (letter) => `_${letter.toLowerCase()}`);
}

/**
 * Retrieves a shared object reference of an object by its ID.
 *
 * This function fetches the object from the dWallet client using the provided object ID.
 * It then checks if the object is a shared object and retrieves its initial shared version.
 * If the object is not a shared object, an error is thrown.
 *
 * @param {string} objectId - The ID of the object to retrieve.
 * @param {DWalletClient} client - The dWallet client instance.
 * @param {boolean} [mutable=false] - Indicates if the shared object reference should be mutable.
 * @returns An object containing the shared object reference details.
 * @throws Will throw an error if the object is not a shared object.
 */
export async function getSharedObjectRefById(
	objectId: string,
	client: DWalletClient,
	mutable: boolean = false,
) {
	let objectResponse = await client.getObject({
		id: objectId,
		options: { showContent: true, showOwner: true },
	});
	let owner = objectResponse.data?.owner;
	const initialSharedVersion =
		owner &&
		typeof owner === 'object' &&
		'Shared' in owner &&
		owner.Shared.initial_shared_version !== undefined
			? owner.Shared.initial_shared_version!
			: undefined;

	if (initialSharedVersion === undefined) {
		throw new Error('Failed to create shared ref: object is not a shared object');
	}

	return {
		objectId: objectResponse.data?.objectId!,
		initialSharedVersion: initialSharedVersion,
		mutable: mutable,
	};
}

// Helper function to parse nested structs
const parseNestedStruct = (data: any): any => {
	if (data?.fields) {
		let parsedData: any = {};
		for (const key in data.fields) {
			if (typeof data.fields[key] === 'object' && data.fields[key] !== null) {
				parsedData[key] = parseNestedStruct(data.fields[key]);
			} else {
				parsedData[key] = data.fields[key];
			}
		}
		return parsedData;
	}
	return data;
};
