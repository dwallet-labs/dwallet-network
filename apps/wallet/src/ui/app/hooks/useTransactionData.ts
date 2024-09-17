// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useFormatCoin } from '@mysten/core';
import { usePeraClient } from '@mysten/dapp-kit';
import { Transaction } from '@pera-io/pera/transactions';
import { PERA_TYPE_ARG } from '@pera-io/pera/utils';
import { useQuery } from '@tanstack/react-query';

export function useTransactionData(sender?: string | null, transaction?: Transaction | null) {
	const client = usePeraClient();
	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['transaction-data', transaction?.serialize()],
		queryFn: async () => {
			const clonedTransaction = Transaction.from(transaction!);
			if (sender) {
				clonedTransaction.setSenderIfNotSet(sender);
			}
			// Build the transaction to bytes, which will ensure that the transaction data is fully populated:
			await clonedTransaction!.build({ client });
			return clonedTransaction!.getData();
		},
		enabled: !!transaction,
	});
}

export function useTransactionGasBudget(sender?: string | null, transaction?: Transaction | null) {
	const { data, ...rest } = useTransactionData(sender, transaction);

	const [formattedGas] = useFormatCoin(data?.gasData.budget, PERA_TYPE_ARG);

	return {
		data: formattedGas,
		...rest,
	};
}
