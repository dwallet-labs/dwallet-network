;

// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Buffer } from 'buffer';
import type { SuiClient } from '@mysten/sui/client';
import type { Ed25519Keypair } from '@mysten/sui/keypairs/ed25519';





;










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

// Mocked protocol parameters used for testing purposes in non-production environments.
export const mockedProtocolPublicParameters = Uint8Array.from(
	Buffer.from(
		'OlRoZSBmaW5pdGUgZmllbGQgb2YgaW50ZWdlcnMgbW9kdWxvIHByaW1lIHEgJFxtYXRoYmJ7Wn1fcSQgQUE20Ixe0r87oEiv5tyuuv7///////////////////8gAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEABgAA6gIAAGsEAACAAo1qxRDiOhcwi8nC09bb2iJyluygPGzlTUrN4KcED/uOx+47p3hWgWzBu4Ijhb/s4+5ahv9VIKdY8S0oX9wtCQ36c+MLev/6KS9F9cxBP9ifvHMXeC8mT+eyMd8g0yWPeUWSNsQJv/lX0gyUDPkpHzh07qydLdRy4k9bXMt958oW3s9sj5TQFszXRj/hQ4ypgaH7VVZqqehuoBDIq8OuUoGe3FpedpCIMB3le9rhRNcgL1kWY8UQB3u7LKtq5zU/nsDiH6MFOCcnG4F1QUFhYi1rwEQTe/IfwcWgr5IA1KJTOM+CEMZpNUj0//////////////////////////////8CAAAAAAAAAMABDXq92tZ4wZPMECefOilfEFfzuunwjdYvySIXucnlGD/58sPmRatB/njykbEZAHEvHXwteMme2zfcMNJ2ANy51gQC1croFeBK5DRGiELE3LGSdX108WdS90cTIuRry5M0hDOWly8rVmElrneNILvgFcjW9PBM7hR+CF7LZ3OTwpXPJH9SDOf9hM0u53V3QE6rowA6OkYMCvmnoK+SANSiUzjPghDGaTVI9P//////////////////////////////gAKNasUQ4joXMIvJwtPW29oicpbsoDxs5U1KzeCnBA/7jsfuO6d4VoFswbuCI4W/7OPuWob/VSCnWPEtKF/cLQkN+nPjC3r/+ikvRfXMQT/Yn7xzF3gvJk/nsjHfINMlj3lFkjbECb/5V9IMlAz5KR84dO6snS3UcuJPW1zLfefKFt7PbI+U0BbM10Y/4UOMqYGh+1VWaqnobqAQyKvDrlKBntxaXnaQiDAd5Xva4UTXIC9ZFmPFEAd7uyyrauc1P57A4h+jBTgnJxuBdUFBYWIta8BEE3vyH8HFoK+SANSiUzjPghDGaTVI9P//////////////////////////////IEFBNtCMXtK/O6BIr+bcrrr+////////////////////wAEzBuz9efyojxs+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ynVYLLuR23IXL3HIO6KFEulVbrlgfElFDEN/YYwPaH/9TE/Smk0UaIkMtMGgh1l1f6okpEo8+GZrJgEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAJhtqtPbr7G2jXbe7vy9KqvvOR7h/tUm8jtJcOJtx8O7UfNrNqgwLQ/1/Q7g/6YwUho3+XqD9khsfBS+nXVBI0ST7ZUT3crsy78T8zexKsSxP2citKvNSuaH6T3MGRVgvEubrbM00aVqIfmHQmj2m3Mkr+W4P//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////gAJjui20v9js+5zYELTMqb2PHWBJoUQA4q7ZvFP2K6DXixy5xlN9wYjqcFmXXB2dUeniKJb5ll5s5P1mBNFtZMLi60oY6NaGjx/nZUfGvmy3pV2tSZdnynHIsuNBBO0baCg4Y+eCtvPlFH6L+617tSO6s07sjAIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAKNasUQ4joXMIvJwtPW29oicpbsoDxs5U1KzeCnBA/7jsfuO6d4VoFswbuCI4W/7OPuWob/VSCnWPEtKF/cLQkN+nPjC3r/+ikvRfXMQT/Yn7xzF3gvJk/nsjHfINMlj3lFkjbECb/5V9IMlAz5KR84dO6snS3UcuJPW1zLfefKFt7PbI+U0BbM10Y/4UOMqYGh+1VWaqnobqAQyKvDrlKBntxaXnaQiDAd5Xva4UTXIC9ZFmPFEAd7uyyrauc1P57A4h+jBTgnJxuBdUFBYWIta8BEE3vyH8HFoK+SANSiUzjPghDGaTVI9P//////////////////////////////AYACT9duVyfU0iq8W7OZsFiga+z78RGloK5rKC8gfyOsZDzs0TTxxW39zkpyUFxmJ9lsIoi8AroqzJ1mrcp1WCy7kdtyFy9xyDuihRLpVW65YHxJRQxDf2GMD2h//UxP0ppNFGiJDLTBoIdZdX+qJKRKPPhmayYBAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAIACYbarT26+xto123u78vSqr7zke4f7VJvI7SXDibcfDu1HzazaoMC0P9f0O4P+mMFIaN/l6g/ZIbHwUvp11QSNEk+2VE93K7Mu/E/M3sSrEsT9nIrSrzUrmh+k9zBkVYLxLm62zNNGlaiH5h0Jo9ptzJK/luD//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////4ACY7ottL/Y7Puc2BC0zKm9jx1gSaFEAOKu2bxT9iug14scucZTfcGI6nBZl1wdnVHp4iiW+ZZebOT9ZgTRbWTC4utKGOjWho8f52VHxr5st6VdrUmXZ8pxyLLjQQTtG2goOGPngrbz5RR+i/ute7UjurNO7IwCAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAALsAAAB2AQAAgALByjOA8B4LcbkbeRpKU25V/GAVzPTaQuDT5YNyMpFktl4Q4qUXaO7nY9NWDN+noN8raJayDi+pxbgO6gagiT3K1jqQtc4fkgY61IbcRUb6Mfgo6RzChIS6apTfq0clEO6+owUENhYSjx7KroFd1O/9esYCDgEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAIJlv941G0L1toy/qZ5rE88ZZIJAb3ynhza332PsROK55GApOD60RuXdmcfsxhlEmRvZasJMEf7gtyDbDnYxSzzjFGHUepl400R6qOZWUJARtcHulbJ6ZEXo2/yn317unV11Xf8aCyark71I+txcmklhkxG5f//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////gAKx2dn/5QBuVYDNB4SqP0feJlXgL8Pthm0mLwADkoWywcdb7hbrMke2lFCYYzKBdS+RYI8ZZJD7J4c7sQnGy3x7mYo/2KyEBM/SNCCQGlWAPNawfkzAm+KgpONQV9L3mkYAo4hAl0QQoqgx7wtlm8ylYMWxxwIAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAJTCSx5wCO07En6bU2MTwqBv00sdEL+WDJy+VeORteps6YiI+QvqW7V8AREifsTYjFaJsgkZe+6RPSCa3bWXOFg6smWZQjdptQED29HDd9mW3rdWy7YFiwuMeUgET4bw8dCGzgK1TGP+pkDVGrnwMZR2U/LcAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA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////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////gALRDPkb5MBxNgg/9De4nN0wY0x4DU++paEuDCHE0nCsT/f8WKvDYXFIv6boavvtY0A+nceOZD714hgO12wrVeSe5vEKYjkNeO/0p4J+N2ul+JzVg62+JOtdYlJIMHIRKQFRG1s3A21UsHt7CFBujecDQyadWgMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAgAKNasUQ4joXMIvJwtPW29oicpbsoDxs5U1KzeCnBA/7jsfuO6d4VoFswbuCI4W/7OPuWob/VSCnWPEtKF/cLQkN+nPjC3r/+ikvRfXMQT/Yn7xzF3gvJk/nsjHfINMlj3lFkjbECb/5V9IMlAz5KR84dO6snS3UcuJPW1zLfefKFt7PbI+U0BbM10Y/4UOMqYGh+1VWaqnobqAQyKvDrlKBntxaXnaQiDAd5Xva4UTXIC9ZFmPFEAd7uyyrauc1P57A4h+jBTgnJxuBdUFBYWIta8BEE3vyH8HFoK+SANSiUzjPghDGaTVI9P//////////////////////////////AAACAAA=',
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
		let interval = 5_000;
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