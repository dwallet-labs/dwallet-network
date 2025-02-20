// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Buffer } from 'buffer';
import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';

export const DWALLET_ECDSAK1_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1';
export const DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1_inner';
export const DWALLET_NETWORK_VERSION = 0;

export const SUI_PACKAGE_ID = '0x2';

interface IkaConfig {
	ika_package_id: string;
	ika_system_package_id: string;
	ika_system_obj_id: string;
}

export interface Config {
	keypair: Ed25519Keypair;
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

// Mocked protocol parameters used for testing purposes in non-production environments.
export const mockedProtocolPublicParameters = Uint8Array.from(
	Buffer.from(
		'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',
		'base64',
	),
);

/**
 * Represents the Move `SystemInnerV1` struct.
 */
interface IKASystemStateInner {
	fields: {
		value: {
			fields: {
				dwallet_2pc_mpc_secp256k1_id: string;
				dwallet_network_decryption_key: {
					fields: {
						dwallet_network_decryption_key_id: string;
					};
				};
			};
		};
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

interface MoveObject {
	fields: any;
}

export function isMoveObject(obj: any): obj is MoveObject {
	return obj?.fields !== undefined;
}

export function isIKASystemStateInner(obj: any): obj is IKASystemStateInner {
	return (
		obj?.fields?.value?.fields?.dwallet_network_decryption_key !== undefined &&
		obj?.fields?.value?.fields?.dwallet_2pc_mpc_secp256k1_id !== undefined
	);
}

export async function getDwalletSecp256k1ObjID(c: Config): Promise<string> {
	const dynamicFields = await c.client.getDynamicFields({
		parentId: c.ikaConfig.ika_system_obj_id,
	});
	let innerSystemState = await c.client.getDynamicFieldObject({
		parentId: c.ikaConfig.ika_system_obj_id,
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
	let obj = await c.client.getObject({
		id: objectID,
		options: {
			showOwner: true,
		},
	});
	let owner = obj.data?.owner;
	if (!owner || !isSharedObjectOwner(owner)) {
		throw new Error('Object is not shared');
	}
	return owner.Shared?.initial_shared_version;
}

interface SharedObjectData {
	object_id: string;
	initial_shared_version: number;
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
