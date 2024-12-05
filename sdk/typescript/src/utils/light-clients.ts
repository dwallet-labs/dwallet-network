// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DWalletClient } from '../client/index.js';
import { bcs } from "../bcs";

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

export async function getObjectRefById(client: DWalletClient, id: string) {
	const res = await client.getObject({ id });

	if (!res.data) {
		throw new Error('No object found');
	}

	return {
		digest: res.data.digest,
		objectId: id,
		version: res.data.version,
	};
}

export const getDWalletBinderByID = async (binderID: string, client: DWalletClient) => {
	let authorityBinderResponse = await client.getObject({
		id: binderID,
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


/**
 * Converts a string to a Uint8Array and serializes it using BCS (Binary Canonical Serialization).
 *
 * @param {string} value - The string to convert and serialize.
 * @returns The serialized Uint8Array.
 */
export function stringToBcs(value: string) {
  let arrayU8 = Uint8Array.from(Array.from(value).map((c) => c.charCodeAt(0)));
  return bcs.vector(bcs.u8()).serialize(arrayU8, {
    size: arrayU8.length,
    maxSize: arrayU8.length * 2,
    allocateSize: arrayU8.length,
  });
}
