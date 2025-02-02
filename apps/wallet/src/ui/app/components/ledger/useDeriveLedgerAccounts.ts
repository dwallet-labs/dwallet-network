// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type LedgerAccountSerializedUI } from '_src/background/accounts/LedgerAccount';
import type IkaLedgerClient from '@mysten/ledgerjs-hw-app-ika';
import { Ed25519PublicKey } from '@ika-io/ika/keypairs/ed25519';
import { useQuery, type UseQueryOptions } from '@tanstack/react-query';

import { useIkaLedgerClient } from './IkaLedgerClientProvider';

export type DerivedLedgerAccount = Pick<
	LedgerAccountSerializedUI,
	'address' | 'publicKey' | 'type' | 'derivationPath'
>;
type UseDeriveLedgerAccountOptions = {
	numAccountsToDerive: number;
} & Pick<UseQueryOptions<DerivedLedgerAccount[], unknown>, 'select'>;

export function useDeriveLedgerAccounts(options: UseDeriveLedgerAccountOptions) {
	const { numAccountsToDerive, ...useQueryOptions } = options;
	const { ikaLedgerClient } = useIkaLedgerClient();

	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: ['derive-ledger-accounts'],
		queryFn: () => {
			if (!ikaLedgerClient) {
				throw new Error("The Ika application isn't open on a connected Ledger device");
			}
			return deriveAccountsFromLedger(ikaLedgerClient, numAccountsToDerive);
		},
		...useQueryOptions,
		gcTime: 0,
	});
}

async function deriveAccountsFromLedger(
	ikaLedgerClient: IkaLedgerClient,
	numAccountsToDerive: number,
) {
	const ledgerAccounts: DerivedLedgerAccount[] = [];
	const derivationPaths = getDerivationPathsForLedger(numAccountsToDerive);

	for (const derivationPath of derivationPaths) {
		const publicKeyResult = await ikaLedgerClient.getPublicKey(derivationPath);
		const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
		const ikaAddress = publicKey.toIkaAddress();
		ledgerAccounts.push({
			type: 'ledger',
			address: ikaAddress,
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
