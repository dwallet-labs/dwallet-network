// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { normalizeStructTag } from '@pera-io/pera/utils';
import { useQuery } from '@tanstack/react-query';

import { useAppsBackend } from '../../../../../core';

export function useBlockedObjectList() {
	const { request } = useAppsBackend();
	return useQuery({
		queryKey: ['apps-backend', 'guardian', 'object-list'],
		queryFn: () => request<{ blocklist: string[] }>('guardian/object-list'),
		select: (data) => data?.blocklist.map(normalizeStructTag) ?? [],
	});
}
