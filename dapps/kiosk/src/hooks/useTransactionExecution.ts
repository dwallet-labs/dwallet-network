// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useSignTransaction, usePeraClient } from '@mysten/dapp-kit';
import { PeraTransactionBlockResponseOptions } from '@pera-io/pera/client';
import { Transaction } from '@pera-io/pera/transactions';

// A helper to execute transactions by:
// 1. Signing them using the wallet
// 2. Executing them using the rpc provider
export function useTransactionExecution() {
	const provider = usePeraClient();

	// sign transaction from the wallet
	const { mutateAsync: signTransaction } = useSignTransaction();

	// tx: Transaction
	const signAndExecute = async ({
		tx,
		options = { showEffects: true },
	}: {
		tx: Transaction;
		options?: PeraTransactionBlockResponseOptions | undefined;
	}) => {
		const signedTx = await signTransaction({ transaction: tx });

		const res = await provider.executeTransactionBlock({
			transactionBlock: signedTx.bytes,
			signature: signedTx.signature,
			options,
		});

		const status = res.effects?.status?.status === 'success';

		if (status) return true;
		else throw new Error('Transaction execution failed.');
	};

	return { signAndExecute };
}
