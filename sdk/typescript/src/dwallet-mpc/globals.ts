// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Buffer } from 'buffer';
import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';

export const DWALLET_ECDSAK1_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1';
export const DWALLET_ECDSAK1_INNER_MOVE_MODULE_NAME = 'dwallet_2pc_mpc_secp256k1_inner';
export const DWALLET_NETWORK_VERSION = 0;

export const SUI_PACKAGE_ID = '0x2';
export const checkpointCreationTime = 2000;

interface IkaConfig {
	ika_package_id: string;
	ika_system_package_id: string;
	ika_system_obj_id: string;
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
	presign: Uint8Array;
}

export function isPresign(obj: any): obj is Presign {
	return obj?.id !== undefined && obj?.dwallet_id !== undefined && obj?.presign !== undefined;
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

// Mocked protocol parameters used for testing purposes in non-production environments.
export const mockedProtocolPublicParameters = Uint8Array.from(
	Buffer.from(
		'OlRoZSBmaW5pdGUgZmllbGQgb2YgaW50ZWdlcnMgbW9kdWxvIHByaW1lIHEgJFxtYXRoYmJ7Wn1fcSQgQUE20Ixe0r87oEiv5tyuuv7///////////////////8gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEABgAA6wIAAGwEAACAAonSHaQHw7+qQR8dFyA1X5JN+d4Yx0HlyYrzpbdugKCc4GlMI/iEB2t95JulvM0XgcG/S2W1wEgo4kvUHZVJvyc8J9CtmphZCkSUnm/J4SM9Y9UC/2avePLPhn6EwdDERgXQf12/YcRUSONaNWkrJTJf30YIkophcapYs+YmSUoJn9LyCaaU9lxayrQj63/UDHiJJ7mWT575nU3ZOVueqRBkEozajUUCgpBNf3KcrA6ImjHSDFvKF4oFUBi9zSF6zXudfb4hmRLocjhJrynn6uF+pdRx04tMt1os5vJpRxvKbE2NjOtg+Any//////////////////////////////8CAAAAAAAAAMABCSzxVVYPeHKsSDHG05sSENPEJ9VTqdpeNoLEb8SPBZ+jweRKvnXLJpWH12gyquRaXTL/5TUdgtP5UxNBandiqalhzgLOdcjfPaH48se1KmmY6Ljo/MZ0AGcp9XQNx77r61v7lpjdwN/bdD5JUXvDeN4YBIRorHLYnuYO8VjQEwaTBujYpekJ9y0KaZttgZGVnL8A6Q2twzs3LObyaUcbymxNjYzrYPgJ8v//////////////////////////////gAKJ0h2kB8O/qkEfHRcgNV+STfneGMdB5cmK86W3boCgnOBpTCP4hAdrfeSbpbzNF4HBv0tltcBIKOJL1B2VSb8nPCfQrZqYWQpElJ5vyeEjPWPVAv9mr3jyz4Z+hMHQxEYF0H9dv2HEVEjjWjVpKyUyX99GCJKKYXGqWLPmJklKCZ/S8gmmlPZcWsq0I+t/1Ax4iSe5lk+e+Z1N2TlbnqkQZBKM2o1FAoKQTX9ynKwOiJox0gxbyheKBVAYvc0hes17nX2+IZkS6HI4Sa8p5+rhfqXUcdOLTLdaLObyaUcbymxNjYzrYPgJ8v//////////////////////////////IEFBNtCMXtK/O6BIr+bcrrr+////////////////////wAE3D5RH/7X2VuBo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',
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

/**
 * Represents a Move Address object owner.
 */
interface AddressObjectOwner {
	AddressOwner: string;
}

interface MoveObject {
	fields: any;
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
	const innerSystemState = await c.client.getDynamicFieldObject({
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
	eventType: string,
	isEventFn: (parsedJson: any) => parsedJson is TEvent,
): Promise<TEvent> {
	const startTime = Date.now();

	while (Date.now() - startTime <= c.timeout) {
		// Wait for a bit before polling again, objects might not be available immediately.
		const interval = 5_000;
		await delay(interval);

		const { data } = await c.client.queryEvents({
			query: {
				TimeRange: {
					startTime: (Date.now() - interval * 2).toString(),
					endTime: Date.now().toString(),
				},
			},
			limit: 1000,
		});

		const match = data.find(
			(event) =>
				event.type === eventType &&
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
