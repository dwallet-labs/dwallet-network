// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IkaMoveObject, IkaObjectData, IkaObjectResponse } from '@ika-io/ika/client';
import { normalizeIkaAddress } from '@ika-io/ika/utils';

export const camelCase = (string: string) => string.replace(/(_\w)/g, (g) => g[1].toUpperCase());

export const parseObjectDataResponse = (response: IkaObjectResponse | undefined) =>
	((response?.data as IkaObjectData)?.content as IkaMoveObject)?.fields as Record<string, any>;

export const parseRegistryResponse = (response: IkaObjectResponse | undefined): any => {
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
					c.type.includes('Address') || key === 'addr' ? normalizeIkaAddress(value) : value,
			};
		},
		{},
	);

	return { ...object, ...data };
};
