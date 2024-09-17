// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraMoveObject, PeraObjectData, PeraObjectResponse } from '@pera-io/pera/client';
import { normalizePeraAddress } from '@pera-io/pera/utils';

export const camelCase = (string: string) => string.replace(/(_\w)/g, (g) => g[1].toUpperCase());

export const parseObjectDataResponse = (response: PeraObjectResponse | undefined) =>
	((response?.data as PeraObjectData)?.content as PeraMoveObject)?.fields as Record<string, any>;

export const parseRegistryResponse = (response: PeraObjectResponse | undefined): any => {
	const fields = parseObjectDataResponse(response)?.value?.fields || {};

	const object = Object.fromEntries(
		Object.entries({ ...fields }).map(([key, val]) => [camelCase(key), val]),
	);

	if (response?.data?.objectId) {
		object.id = response.data.objectId;
	}

	delete object.data;

	const data = (fields.data?.fields.contents || []).reduce(
		(acc: Record<string, any>, c: Record<string, any>) => {
			const key = c.fields.key;
			const value = c.fields.value;

			return {
				...acc,
				[camelCase(key)]:
					c.type.includes('Address') || key === 'addr' ? normalizePeraAddress(value) : value,
			};
		},
		{},
	);

	return { ...object, ...data };
};
