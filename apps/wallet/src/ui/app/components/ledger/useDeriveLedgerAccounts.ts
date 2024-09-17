// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type LedgerAccountSerializedUI } from '_src/background/accounts/LedgerAccount';
import type PeraLedgerClient from '@mysten/ledgerjs-hw-app-pera';
import { Ed25519PublicKey } from '@pera-io/pera/keypairs/ed25519';
import { useQuery, type UseQueryOptions } from '@tanstack/react-query';

import { usePeraLedgerClient } from './PeraLedgerClientProvider';

export type DerivedLedgerAccount = Pick<
	LedgerAccountSerializedUI,
	'address' | 'publicKey' | 'type' | 'derivationPath'
>;
type UseDeriveLedgerAccountOptions = {
	numAccountsToDerive: number;
} & Pick<UseQueryOptions<DerivedLedgerAccount[], unknown>, 'select'>;

export function useDeriveLedgerAccounts(options: UseDeriveLedgerAccountOptions) {
	const { numAccountsToDerive, ...useQueryOptions } = options;
	const { peraLedgerClient } = usePeraLedgerClient();

	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['derive-ledger-accounts'],
		queryFn: () => {
			if (!peraLedgerClient) {
				throw new Error("The Pera application isn't open on a connected Ledger device");
			}
			return deriveAccountsFromLedger(peraLedgerClient, numAccountsToDerive);
		},
		...useQueryOptions,
		gcTime: 0,
	});
}

async function deriveAccountsFromLedger(
	peraLedgerClient: PeraLedgerClient,
	numAccountsToDerive: number,
) {
	const ledgerAccounts: DerivedLedgerAccount[] = [];
	const derivationPaths = getDerivationPathsForLedger(numAccountsToDerive);

	for (const derivationPath of derivationPaths) {
		const publicKeyResult = await peraLedgerClient.getPublicKey(derivationPath);
		const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
		const peraAddress = publicKey.toPeraAddress();
		ledgerAccounts.push({
			type: 'ledger',
			address: peraAddress,
			derivationPath,
			publicKey: publicKey.toBase64(),
		});
	}

	return ledgerAccounts;
}

function getDerivationPathsForLedger(numDerivations: number) {
	return Array.from({
		length: numDerivations,
	}).map((_, index) => `m/44'/784'/${index}'/0'/0'`);
}
