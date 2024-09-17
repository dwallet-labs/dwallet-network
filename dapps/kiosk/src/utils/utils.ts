// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { KioskListing, KioskOwnerCap } from '@mysten/kiosk';
import { PeraObjectResponse } from '@pera-io/pera/client';
import { NPERA_PER_PERA, normalizePeraAddress } from '@pera-io/pera/utils';

// Parse the display of a list of objects into a simple {object_id: display} map
// to use throughout the app.
export const parseObjectDisplays = (
	data: PeraObjectResponse[],
): Record<string, Record<string, string> | undefined> => {
	return data.reduce<Record<string, Record<string, string> | undefined>>(
		(acc, item: PeraObjectResponse) => {
			const display = item.data?.display?.data;
			const id = item.data?.objectId!;
			acc[id] = display || undefined;
			return acc;
		},
		{},
	);
};

export const processKioskListings = (data: KioskListing[]): Record<string, KioskListing> => {
	const results: Record<string, KioskListing> = {};

	data
		.filter((x) => !!x)
		.map((x: KioskListing) => {
			results[x.objectId || ''] = x;
			return x;
		});
	return results;
};

export const nperaToPera = (npera: bigint | string | undefined) => {
	if (!npera) return 0;
	return Number(npera || 0) / Number(NPERA_PER_PERA);
};

export const formatPera = (amount: number) => {
	return new Intl.NumberFormat('en-US', {
		minimumFractionDigits: 2,
		maximumFractionDigits: 5,
	}).format(amount);
};

/**
 * Finds an active owner cap for a kioskId based on the
 * address owned kiosks.
 */
export const findActiveCap = (
	caps: KioskOwnerCap[] = [],
	kioskId: string,
): KioskOwnerCap | undefined => {
	return caps.find((x) => normalizePeraAddress(x.kioskId) === normalizePeraAddress(kioskId));
};
