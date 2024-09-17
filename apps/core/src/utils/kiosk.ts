// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { KIOSK_OWNER_CAP, Network, PERSONAL_KIOSK_RULE_ADDRESS } from '@mysten/kiosk';
import { PeraObjectData, PeraObjectResponse } from '@pera-io/pera/client';

export const ORIGINBYTE_KIOSK_MODULE =
	'0x95a441d389b07437d00dd07e0b6f05f513d7659b13fd7c5d3923c7d9d847199b::ob_kiosk';

export const ORIGINBYTE_KIOSK_OWNER_TOKEN = `${ORIGINBYTE_KIOSK_MODULE}::OwnerToken`;

export function isKioskOwnerToken(
	network: Network,
	object?: PeraObjectResponse | PeraObjectData | null,
) {
	if (!object) return false;
	const objectData = 'data' in object && object.data ? object.data : (object as PeraObjectData);
	return [
		KIOSK_OWNER_CAP,
		`${PERSONAL_KIOSK_RULE_ADDRESS[network]}::personal_kiosk::PersonalKioskCap`,
		ORIGINBYTE_KIOSK_OWNER_TOKEN,
	].includes(objectData?.type ?? '');
}

export function getKioskIdFromOwnerCap(object: PeraObjectResponse | PeraObjectData) {
	const objectData = 'data' in object && object.data ? object.data : (object as PeraObjectData);
	const fields =
		objectData.content?.dataType === 'moveObject'
			? (objectData.content.fields as {
					for?: string;
					kiosk?: string;
					cap?: { fields: { for: string } };
				})
			: null;
	return fields?.for ?? fields?.kiosk ?? fields?.cap?.fields.for!;
}
