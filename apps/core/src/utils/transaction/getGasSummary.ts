// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import {
	DryRunTransactionBlockResponse,
	GasCostSummary,
	PeraGasData,
	PeraTransactionBlockResponse,
	TransactionEffects,
} from '@pera-io/pera/client';

type Optional<T> = {
	[K in keyof T]?: T[K];
};

export type GasSummaryType =
	| (GasCostSummary &
			Optional<PeraGasData> & {
				totalGas?: string;
				owner?: string;
				isSponsored: boolean;
				gasUsed: GasCostSummary;
			})
	| null;

export function getGasSummary(
	transaction: PeraTransactionBlockResponse | DryRunTransactionBlockResponse,
): GasSummaryType {
	const { effects } = transaction;
	if (!effects) return null;
	const totalGas = getTotalGasUsed(effects);

	let sender = 'transaction' in transaction ? transaction.transaction?.data.sender : undefined;

	const gasData = 'transaction' in transaction ? transaction.transaction?.data.gasData : {};

	const owner =
		'transaction' in transaction
			? transaction.transaction?.data.gasData.owner
			: typeof effects.gasObject.owner === 'object' && 'AddressOwner' in effects.gasObject.owner
				? effects.gasObject.owner.AddressOwner
				: '';

	return {
		...effects.gasUsed,
		...gasData,
		owner,
		totalGas: totalGas?.toString(),
		isSponsored: !!owner && !!sender && owner !== sender,
		gasUsed: transaction?.effects!.gasUsed,
	};
}

export function getTotalGasUsed(effects: TransactionEffects): bigint | undefined {
	const gasSummary = effects?.gasUsed;
	return gasSummary
		? BigInt(gasSummary.computationCost) +
				BigInt(gasSummary.storageCost) -
				BigInt(gasSummary.storageRebate)
		: undefined;
}
