// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { usePeraClient } from '@mysten/dapp-kit';
import { normalizePeraAddress } from '@pera-io/pera/utils';
import { useQuery } from '@tanstack/react-query';

const defaultOptions = {
	showType: true,
	showContent: true,
	showOwner: true,
	showPreviousTransaction: true,
	showStorageRebate: true,
	showDisplay: true,
};

export function useGetObject(objectId?: string | null) {
	const client = usePeraClient();
	const normalizedObjId = objectId && normalizePeraAddress(objectId);
	return useQuery({
		queryKey: ['object', normalizedObjId],
		queryFn: () =>
			client.getObject({
				id: normalizedObjId!,
				options: defaultOptions,
			}),
		enabled: !!normalizedObjId,
	});
}
