// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getAmount } from '_helpers';
import type { IkaTransactionBlockResponse } from '@ika-io/ika/client';
import { IKA_TYPE_ARG } from '@ika-io/ika/utils';
import { useMemo } from 'react';

export function useGetTransferAmount({
	txn,
	activeAddress,
}: {
	txn: IkaTransactionBlockResponse;
	activeAddress: string;
}) {
	const { effects, events } = txn;
	// const { coins } = getEventsSummary(events!, activeAddress);

	const ikaTransfer = useMemo(() => {
		const txdetails = txn.transaction?.data.transaction!;
		return getAmount(txdetails, effects!, events!)?.map(
			({ amount, coinType, recipientAddress }) => {
				return {
					amount: amount || 0,
					coinType: coinType || IKA_TYPE_ARG,
					receiverAddress: recipientAddress,
				};
			},
		);
	}, [txn, effects, events]);

	// MUSTFIX(chris)
	// const transferAmount = useMemo(() => {
	//     return ikaTransfer?.length
	//         ? ikaTransfer
	//         : coins.filter(
	//               ({ receiverAddress }) => receiverAddress === activeAddress
	//           );
	// }, [ikaTransfer, coins, activeAddress]);

	// return ikaTransfer ?? transferAmount;
	return ikaTransfer;
}
