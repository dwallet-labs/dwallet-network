// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { KioskListing, KioskOwnerCap } from '@mysten/kiosk';
import { IkaObjectResponse } from '@ika-io/ika/client';
import { NIKA_PER_IKA, normalizeIkaAddress } from '@ika-io/ika/utils';

// Parse the display of a list of objects into a simple {object_id: display} map
// to use throughout the app.
export const parseObjectDisplays = (
	data: IkaObjectResponse[],
): Record<string, Record<string, string> | undefined> => {
	return data.reduce<Record<string, Record<string, string> | undefined>>(
		(acc, item: IkaObjectResponse) => {
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

export const nikaToIka = (nika: bigint | string | undefined) => {
	if (!nika) return 0;
	return Number(nika || 0) / Number(NIKA_PER_IKA);
};

export const formatIka = (amount: number) => {
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
	return caps.find((x) => normalizeIkaAddress(x.kioskId) === normalizeIkaAddress(kioskId));
};
