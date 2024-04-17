// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type SuiTransaction } from '@dwallet/dwallet.js/client';

import { Transaction } from './Transaction';
import { ProgrammableTxnBlockCard } from '~/components/transactions/ProgTxnBlockCard';
import { CollapsibleSection } from '~/ui/collapsible/CollapsibleSection';

interface TransactionsCardProps {
	transactions: SuiTransaction[];
}

export function TransactionsCard({ transactions }: TransactionsCardProps) {
	if (!transactions?.length) {
		return null;
	}

	const expandableItems = transactions.map((transaction, index) => {
		const [[type, data]] = Object.entries(transaction);

		return (
			<CollapsibleSection defaultOpen key={index} title={type}>
				<div data-testid="transactions-card-content">
					<Transaction key={index} type={type} data={data} />
				</div>
			</CollapsibleSection>
		);
	});

	return (
		<ProgrammableTxnBlockCard
			initialClose
			items={expandableItems}
			itemsLabel={transactions.length > 1 ? 'Transactions' : 'Transaction'}
			count={transactions.length}
		/>
	);
}
