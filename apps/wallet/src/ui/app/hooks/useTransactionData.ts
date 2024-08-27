// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useFormatCoin } from '@mysten/core';
import { useSuiClient } from '@mysten/dapp-kit';
import { TransactionBlock } from '@dwallet-network/dwallet.js/transactions';
import { SUI_TYPE_ARG } from '@dwallet-network/dwallet.js/utils';
import { useQuery } from '@tanstack/react-query';

export function useTransactionData(sender?: string | null, transaction?: TransactionBlock | null) {
	const client = useSuiClient();
	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['transaction-data', transaction?.serialize()],
		queryFn: async () => {
			const clonedTransaction = new TransactionBlock(transaction!);
			if (sender) {
				clonedTransaction.setSenderIfNotSet(sender);
			}
			// Build the transaction to bytes, which will ensure that the transaction data is fully populated:
			await clonedTransaction!.build({ client });
			return clonedTransaction!.blockData;
		},
		enabled: !!transaction,
	});
}

export function useTransactionGasBudget(
	sender?: string | null,
	transaction?: TransactionBlock | null,
) {
	const { data, ...rest } = useTransactionData(sender, transaction);

	const [formattedGas] = useFormatCoin(data?.gasConfig.budget, SUI_TYPE_ARG);

	return {
		data: formattedGas,
		...rest,
	};
}
