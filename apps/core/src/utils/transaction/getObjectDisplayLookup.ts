// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { DisplayFieldsResponse, PeraObjectResponse } from '@pera-io/pera/client';

import { hasDisplayData } from '../hasDisplayData';

export function getObjectDisplayLookup(objects: PeraObjectResponse[] = []) {
	const lookup: Map<string, DisplayFieldsResponse> = new Map();
	return objects?.filter(hasDisplayData).reduce((acc, curr) => {
		if (curr.data?.objectId) {
			acc.set(curr.data.objectId, curr.data.display as DisplayFieldsResponse);
		}
		return acc;
	}, lookup);
}
