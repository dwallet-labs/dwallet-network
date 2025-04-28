// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';

export const DWALLET_ECDSA_K1_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1';
export const DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1_inner';
export const DWALLET_NETWORK_VERSION = 0;

export const SUI_PACKAGE_ID = '0x2';
export const checkpointCreationTime = 2000;

interface IkaConfig {
	ika_package_id: string;
	ika_system_package_id: string;
	ika_system_object_id: string;
}

export interface Config {
	suiClientKeypair: Ed25519Keypair;
	encryptedSecretShareSigningKeypair: Ed25519Keypair;
	client: SuiClient;
	timeout: number;
	ikaConfig: IkaConfig;
	dWalletSeed: Uint8Array;
}

export enum MPCKeyScheme {
	Secp256k1 = 1,
	Ristretto = 2,
}

/**
 * Utility function to create a delay.
 */
export function delay(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

export interface Presign {
	id: { id: string };
	dwallet_id: string;
	state: {
		fields: {
			presign: Uint8Array;
		};
	};
	cap_id: string;
}

export function isPresign(obj: any): obj is Presign {
	return (
		obj?.id !== undefined &&
		obj?.dwallet_id !== undefined &&
		obj?.state?.fields?.presign !== undefined
	);
}

export async function getObjectWithType<TObject>(
	conf: Config,
	objectID: string,
	isObject: (obj: any) => obj is TObject,
): Promise<TObject> {
	const obj = await conf.client.getObject({
		id: objectID,
		options: { showContent: true },
	});
	if (!isMoveObject(obj.data?.content)) {
		throw new Error('Invalid object');
	}
	const objContent = obj.data?.content.fields;
	if (!isObject(objContent)) {
		throw new Error('Invalid object fields');
	}
	return objContent;
}

/**
 * Represents the Move `SystemInnerV1` struct.
 */
interface IKASystemStateInner {
	fields: {
		value: {
			fields: {
				dwallet_2pc_mpc_secp256k1_id: string;
				dwallet_2pc_mpc_secp256k1_network_decryption_keys: Array<any>;
			};
		};
	};
}

interface DWalletNetworkDecryptionKey {
	fields: {
		id: { id: string };
		network_dkg_public_output: Uint8Array;
	};
}

/**
 * Represents a Move shared object owner.
 */
interface SharedObjectOwner {
	Shared: {
		// The object version when it became shared.
		initial_shared_version: number;
	};
}

/**
 * Represents a Move Address object owner.
 */
interface AddressObjectOwner {
	AddressOwner: string;
}

interface MoveObject {
	fields: any;
}

interface MoveDynamicField {
	fields: {
		name: string;
		value: Uint8Array;
	};
}

export interface SharedObjectData {
	object_id: string;
	initial_shared_version: number;
}

export function isAddressObjectOwner(obj: any): obj is AddressObjectOwner {
	return obj?.AddressOwner !== undefined;
}

export function isMoveObject(obj: any): obj is MoveObject {
	return obj?.fields !== undefined;
}

export function isMoveDynamicField(obj: any): obj is MoveDynamicField {
	return obj?.fields.name !== undefined || obj?.fields.value !== undefined;
}

export function getEncryptionKeyMoveType(ikaSystemPackageID: string): string {
	return `${ikaSystemPackageID}::${DWALLET_ECDSA_K1_INNER_MOVE_MODULE_NAME}::EncryptionKey`;
}

export function isIKASystemStateInner(obj: any): obj is IKASystemStateInner {
	return (
		obj?.fields?.value?.fields?.dwallet_2pc_mpc_secp256k1_network_decryption_keys !== undefined &&
		obj?.fields?.value?.fields?.dwallet_2pc_mpc_secp256k1_id !== undefined
	);
}

export function isDWalletNetworkDecryptionKey(obj: any): obj is DWalletNetworkDecryptionKey {
	return obj?.fields?.network_dkg_public_output !== undefined;
}

export async function getDwalletSecp256k1ObjID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: c.ikaConfig.ika_system_object_id,
	});
	const innerSystemState = await c.client.getDynamicFieldObject({
		parentId: c.ikaConfig.ika_system_object_id,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	if (!isIKASystemStateInner(innerSystemState.data?.content)) {
		throw new Error('Invalid inner system state');
	}
	return innerSystemState.data?.content?.fields.value.fields.dwallet_2pc_mpc_secp256k1_id;
}

export function isSharedObjectOwner(obj: any): obj is SharedObjectOwner {
	return obj?.Shared?.initial_shared_version !== undefined;
}

export async function getInitialSharedVersion(c: Config, objectID: string): Promise<number> {
	const obj = await c.client.getObject({
		id: objectID,
		options: {
			showOwner: true,
		},
	});
	const owner = obj.data?.owner;
	if (!owner || !isSharedObjectOwner(owner)) {
		throw new Error('Object is not shared');
	}
	return owner.Shared?.initial_shared_version;
}

export async function getDWalletSecpState(c: Config): Promise<SharedObjectData> {
	const dwalletSecp256k1ObjID = await getDwalletSecp256k1ObjID(c);
	const initialSharedVersion = await getInitialSharedVersion(c, dwalletSecp256k1ObjID);
	return {
		object_id: dwalletSecp256k1ObjID,
		initial_shared_version: initialSharedVersion,
	};
}

export async function fetchObjectWithType<TObject>(
	conf: Config,
	objectType: string,
	isObject: (obj: any) => obj is TObject,
	objectId: string,
) {
	const res = await conf.client.getObject({
		id: objectId,
		options: { showContent: true },
	});

	const objectData =
		res.data?.content?.dataType === 'moveObject' &&
		res.data?.content.type === objectType &&
		isObject(res.data.content.fields)
			? (res.data.content.fields as TObject)
			: null;

	if (!objectData) {
		throw new Error(
			`invalid object of type ${objectType}, got: ${JSON.stringify(res.data?.content)}`,
		);
	}

	return objectData;
}

interface StartSessionEvent {
	session_id: string;
}

export function isStartSessionEvent(event: any): event is StartSessionEvent {
	return event.session_id !== undefined;
}

export async function fetchCompletedEvent<TEvent extends { session_id: string }>(
	c: Config,
	sessionID: string,
	isEventFn: (parsedJson: any) => parsedJson is TEvent,
	eventType: string = '',
): Promise<TEvent> {
	const startTime = Date.now();

	while (Date.now() - startTime <= c.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		const interval = 500;
		await delay(interval);

		const { data } = await c.client.queryEvents({
			query: {
				TimeRange: {
					startTime: (Date.now() - interval * 4).toString(),
					endTime: Date.now().toString(),
				},
			},
			limit: 1000,
		});

		const match = data.find(
			(event) =>
				(event.type === eventType || !eventType) &&
				isEventFn(event.parsedJson) &&
				event.parsedJson.session_id === sessionID,
		);

		if (match) return match.parsedJson as TEvent;
	}

	const seconds = ((Date.now() - startTime) / 1000).toFixed(2);
	throw new Error(
		`timeout: unable to fetch an event of type ${eventType} within ${
			c.timeout / (60 * 1000)
		} minutes (${seconds} seconds passed).`,
	);
}

export interface DWalletCap {
	dwallet_id: string;
}

export function isDWalletCap(obj: any): obj is DWalletCap {
	return !!obj?.dwallet_id;
}

interface ActiveDWallet {
	state: {
		fields: {
			public_output: Uint8Array;
		};
	};
}

export function isActiveDWallet(obj: any): obj is ActiveDWallet {
	return obj?.state?.fields?.public_output !== undefined;
}

export async function getNetworkDecryptionKeyPublicOutputID(
	c: Config,
	networkDecryptionKeyId?: string | null,
): Promise<string> {
	networkDecryptionKeyId = networkDecryptionKeyId ?? (await getNetworkDecryptionKeyID(c));
	const networkDecryptionKey = await c.client.getObject({
		id: networkDecryptionKeyId,
		options: { showContent: true },
	});

	if (
		!networkDecryptionKey ||
		!isMoveObject(networkDecryptionKey?.data?.content) ||
		!isDWalletNetworkDecryptionKey(networkDecryptionKey.data.content) ||
		!isMoveObject(networkDecryptionKey.data.content.fields.network_dkg_public_output)
	) {
		throw new Error(`invalid network decryption key object: ${networkDecryptionKeyId}`);
	}
	return networkDecryptionKey.data.content.fields.network_dkg_public_output.fields.contents.fields
		.id?.id;
}

async function readTableVecAsRawBytes(c: Config, table_id: string): Promise<Uint8Array> {
	let cursor: string | null = null;
	const allTableRows: { objectId: string }[] = [];

	// Fetch all dynamic fields using pagination with cursor
	do {
		const dynamicFieldPage = await c.client.getDynamicFields({
			parentId: table_id,
			cursor,
		});

		if (!dynamicFieldPage?.data?.length) {
			if (allTableRows.length === 0) {
				throw new Error('no dynamic fields found');
			}
			break;
		}

		allTableRows.push(...dynamicFieldPage.data);
		cursor = dynamicFieldPage.nextCursor;
	} while (cursor);

	const data: Uint8Array[] = [];
	for (const tableRowResult of allTableRows) {
		const id = tableRowResult.objectId;

		const dynField = await c.client.getObject({
			id: id,
			options: { showContent: true },
		});
		if (!isMoveObject(dynField.data?.content) || !isMoveDynamicField(dynField.data?.content)) {
			throw new Error('invalid dynamic field object');
		}
		const tableIndex = parseInt(dynField.data.content.fields.name);
		data[tableIndex] = dynField.data.content.fields.value;
	}
	return new Uint8Array(data.flatMap((arr) => Array.from(arr)));
}

export async function getNetworkDecryptionKeyPublicOutput(c: Config): Promise<Uint8Array> {
	const networkDecryptionKeyPublicOutputID = await getNetworkDecryptionKeyPublicOutputID(c, null);
	return await readTableVecAsRawBytes(c, networkDecryptionKeyPublicOutputID);
}

export async function getNetworkDecryptionKeyID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: c.ikaConfig.ika_system_object_id,
	});
	const innerSystemState = await c.client.getDynamicFieldObject({
		parentId: c.ikaConfig.ika_system_object_id,
		name: dynamicFields.data[DWALLET_NETWORK_VERSION].name,
	});
	if (!isIKASystemStateInner(innerSystemState.data?.content)) {
		throw new Error('Invalid inner system state');
	}

	const network_decryption_keys =
		innerSystemState.data.content.fields.value.fields
			.dwallet_2pc_mpc_secp256k1_network_decryption_keys;
	const decryptionKeyID =
		network_decryption_keys[network_decryption_keys.length - 1]?.fields
			?.dwallet_network_decryption_key_id;
	if (!decryptionKeyID) {
		throw new Error('No network decryption key found');
	}
	return decryptionKeyID;
}

export interface DWallet {
	dwalletID: string;
	dwallet_cap_id: string;
	secret_share: Uint8Array;
	output: Uint8Array;
	encrypted_secret_share_id: string;
}

export interface EncryptedDWalletData {
	dwallet_id: string;
	encrypted_user_secret_key_share_id: string;
}
