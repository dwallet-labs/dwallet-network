// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type TransactionBlock } from '@dwallet-network/dwallet.js/transactions';
import { useQuery } from '@tanstack/react-query';

import { useAccountByAddress } from './useAccountByAddress';
import { useSigner } from './useSigner';

export function useTransactionDryRun(
	sender: string | undefined,
	transactionBlock: TransactionBlock,
) {
	const { data: account } = useAccountByAddress(sender);
	const signer = useSigner(account || null);
	const response = useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['dryRunTransaction', transactionBlock.serialize()],
		queryFn: () => {
			return signer!.dryRunTransactionBlock({ transactionBlock });
		},
		enabled: !!signer,
	});
	return response;
}
