// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@pera-io/pera/bcs';
import { Transaction } from '@pera-io/pera/transactions';
import { fromB64, toB64 } from '@pera-io/pera/utils';
import type { WalletWithFeatures } from '@wallet-standard/core';

import type {
	PeraSignAndExecuteTransactionInput,
	PeraSignTransactionInput,
	PeraWalletFeatures,
} from './features/index.js';

declare module '@wallet-standard/core' {
	export interface Wallet {
		/**
		 * Unique identifier of the Wallet.
		 *
		 * If not provided, the wallet name will be used as the identifier.
		 */
		readonly id?: string;
	}

	export interface StandardConnectOutput {
		supportedIntents?: string[];
	}
}

export type { Wallet } from '@wallet-standard/core';

export async function signAndExecuteTransaction(
	wallet: WalletWithFeatures<Partial<PeraWalletFeatures>>,
	input: PeraSignAndExecuteTransactionInput,
) {
	if (wallet.features['pera:signAndExecuteTransaction']) {
		return wallet.features['pera:signAndExecuteTransaction'].signAndExecuteTransaction(input);
	}

	if (!wallet.features['pera:signAndExecuteTransactionBlock']) {
		throw new Error(
			`Provided wallet (${wallet.name}) does not support the signAndExecuteTransaction feature.`,
		);
	}

	const { signAndExecuteTransactionBlock } = wallet.features['pera:signAndExecuteTransactionBlock'];

	const transactionBlock = Transaction.from(await input.transaction.toJSON());
	const { digest, rawEffects, rawTransaction } = await signAndExecuteTransactionBlock({
		account: input.account,
		chain: input.chain,
		transactionBlock,
		options: {
			showRawEffects: true,
			showRawInput: true,
		},
	});

	const [
		{
			txSignatures: [signature],
			intentMessage: { value: bcsTransaction },
		},
	] = bcs.SenderSignedData.parse(fromB64(rawTransaction!));

	const bytes = bcs.TransactionData.serialize(bcsTransaction).toBase64();

	return {
		digest,
		signature,
		bytes,
		effects: toB64(new Uint8Array(rawEffects!)),
	};
}

export async function signTransaction(
	wallet: WalletWithFeatures<Partial<PeraWalletFeatures>>,
	input: PeraSignTransactionInput,
) {
	if (wallet.features['pera:signTransaction']) {
		return wallet.features['pera:signTransaction'].signTransaction(input);
	}

	if (!wallet.features['pera:signTransactionBlock']) {
		throw new Error(
			`Provided wallet (${wallet.name}) does not support the signTransaction feature.`,
		);
	}

	const { signTransactionBlock } = wallet.features['pera:signTransactionBlock'];

	const transaction = Transaction.from(await input.transaction.toJSON());
	const { transactionBlockBytes, signature } = await signTransactionBlock({
		transactionBlock: transaction,
		account: input.account,
		chain: input.chain,
	});

	return { bytes: transactionBlockBytes, signature };
}
