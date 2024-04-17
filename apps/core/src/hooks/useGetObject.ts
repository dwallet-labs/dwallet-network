// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useSuiClient } from '@mysten/dapp-kit';
import { normalizeSuiAddress } from '@dwallet/dwallet.js/utils';
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
	const client = useSuiClient();
	const normalizedObjId = objectId && normalizeSuiAddress(objectId);
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
