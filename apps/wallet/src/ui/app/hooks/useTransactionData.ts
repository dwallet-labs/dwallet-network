// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@mysten/core';
import { useIkaClient } from '@mysten/dapp-kit';
import { Transaction } from '@ika-io/ika/transactions';
import { IKA_TYPE_ARG } from '@ika-io/ika/utils';
import { useQuery } from '@tanstack/react-query';

export function useTransactionData(sender?: string | null, transaction?: Transaction | null) {
	const client = useIkaClient();
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

	const [formattedGas] = useFormatCoin(data?.gasData.budget, IKA_TYPE_ARG);

	return {
		data: formattedGas,
		...rest,
	};
}
