// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getAmount } from '_helpers';
import type { PeraTransactionBlockResponse } from '@pera-io/pera/client';
import { PERA_TYPE_ARG } from '@pera-io/pera/utils';
import { useMemo } from 'react';

export function useGetTransferAmount({
	txn,
	activeAddress,
}: {
	txn: PeraTransactionBlockResponse;
	activeAddress: string;
}) {
	const { effects, events } = txn;
	// const { coins } = getEventsSummary(events!, activeAddress);

	const peraTransfer = useMemo(() => {
		const txdetails = txn.transaction?.data.transaction!;
		return getAmount(txdetails, effects!, events!)?.map(
			({ amount, coinType, recipientAddress }) => {
				return {
					amount: amount || 0,
					coinType: coinType || PERA_TYPE_ARG,
					receiverAddress: recipientAddress,
				};
			},
		);
	}, [txn, effects, events]);

	// MUSTFIX(chris)
	// const transferAmount = useMemo(() => {
	//     return peraTransfer?.length
	//         ? peraTransfer
	//         : coins.filter(
	//               ({ receiverAddress }) => receiverAddress === activeAddress
	//           );
	// }, [peraTransfer, coins, activeAddress]);

	// return peraTransfer ?? transferAmount;
	return peraTransfer;
}
