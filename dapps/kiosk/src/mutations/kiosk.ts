// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useCurrentAccount } from '@mysten/dapp-kit';
import { Kiosk, KioskTransaction } from '@mysten/kiosk';
import { TransactionBlock } from '@mysten/sui.js/transactions';
import { useMutation } from '@tanstack/react-query';
import { toast } from 'react-hot-toast';

import { OwnedObjectType } from '../components/Inventory/OwnedObjects';
import { useKioskClient } from '../context/KioskClientContext';
import { useOwnedKiosk } from '../hooks/kiosk';
import { useTransactionExecution } from '../hooks/useTransactionExecution';
import { findActiveCap } from '../utils/utils';

type MutationParams = {
	onSuccess?: () => void;
	onError?: (e: Error) => void;
};

const defaultOnError = (e: Error) => {
	if (typeof e === 'string') toast.error(e);
	else toast.error(e?.message);
};

/**
 * Create a new kiosk.
 */
export function useCreateKioskMutation({ onSuccess, onError }: MutationParams) {
	const currentAccount = useCurrentAccount();
	const { signAndExecute } = useTransactionExecution();
	const kioskClient = useKioskClient();

	return useMutation({
		mutationFn: () => {
			if (!currentAccount?.address) throw new Error('You need to connect your wallet!');
			const txb = new TransactionBlock();
			new KioskTransaction({ transactionBlock: txb, kioskClient }).createAndShare(
				currentAccount?.address,
			);
			return signAndExecute({ tx: txb });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}

/**
 * Place & List or List for sale in kiosk.
 */
export function usePlaceAndListMutation({ onSuccess, onError }: MutationParams) {
	const currentAccount = useCurrentAccount();
	const { data: ownedKiosk } = useOwnedKiosk(currentAccount?.address);
	const { signAndExecute } = useTransactionExecution();
	const kioskClient = useKioskClient();

	return useMutation({
		mutationFn: async ({
			item,
			price,
			shouldPlace,
			kioskId,
		}: {
			item: OwnedObjectType;
			price: string;
			shouldPlace?: boolean;
			kioskId: string;
		}) => {
			// find active kiosk cap.
			const cap = findActiveCap(ownedKiosk?.caps, kioskId);

			if (!cap || !currentAccount?.address) throw new Error('Missing account, kiosk or kiosk cap');

			const txb = new TransactionBlock();

			const kioskTx = new KioskTransaction({ kioskClient, transactionBlock: txb, cap });

			if (shouldPlace) {
				kioskTx.placeAndList({
					item: item.objectId,
					itemType: item.type,
					price,
				});
			} else {
				kioskTx.list({
					itemId: item.objectId,
					itemType: item.type,
					price,
				});
			}

			kioskTx.finalize();

			return signAndExecute({ tx: txb });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}

/**
 * Mutation to place an item in the kiosk.
 */
export function usePlaceMutation({ onSuccess, onError }: MutationParams) {
	const currentAccount = useCurrentAccount();
	const { data: ownedKiosk } = useOwnedKiosk(currentAccount?.address);
	const { signAndExecute } = useTransactionExecution();
	const kioskClient = useKioskClient();

	return useMutation({
		mutationFn: async ({ item, kioskId }: { item: OwnedObjectType; kioskId: string }) => {
			// find active kiosk cap.
			const cap = findActiveCap(ownedKiosk?.caps, kioskId);

			if (!cap || !currentAccount?.address) throw new Error('Missing account, kiosk or kiosk cap');

			const txb = new TransactionBlock();

			new KioskTransaction({ transactionBlock: txb, kioskClient, cap })
				.place({ itemType: item.type, item: item.objectId })
				.finalize();

			return signAndExecute({ tx: txb });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}

/**
 * Withdraw profits from kiosk
 */
export function useWithdrawMutation({ onError, onSuccess }: MutationParams) {
	const currentAccount = useCurrentAccount();
	const { data: ownedKiosk } = useOwnedKiosk(currentAccount?.address);

	const { signAndExecute } = useTransactionExecution();
	const kioskClient = useKioskClient();

	return useMutation({
		mutationFn: async ({ id, profits }: Kiosk) => {
			// find active kiosk cap.
			const cap = findActiveCap(ownedKiosk?.caps, id);

			if (!cap || !currentAccount?.address) throw new Error('Missing account, kiosk or kiosk cap');
			const txb = new TransactionBlock();

			new KioskTransaction({ transactionBlock: txb, kioskClient, cap })
				.withdraw(currentAccount.address, profits)
				.finalize();

			return signAndExecute({ tx: txb });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}

/**
 * Mutation to take an item from the kiosk.
 */
export function useTakeMutation({ onSuccess, onError }: MutationParams) {
	const currentAccount = useCurrentAccount();
	const { data: ownedKiosk } = useOwnedKiosk(currentAccount?.address);
	const { signAndExecute } = useTransactionExecution();
	const kioskClient = useKioskClient();

	return useMutation({
		mutationFn: async ({ item, kioskId }: { item: OwnedObjectType; kioskId: string }) => {
			// find active kiosk cap.
			const cap = findActiveCap(ownedKiosk?.caps, kioskId);

			if (!cap || !currentAccount?.address) throw new Error('Missing account, kiosk or kiosk cap');

			if (!item?.objectId) throw new Error('Missing item.');
			const txb = new TransactionBlock();

			new KioskTransaction({ transactionBlock: txb, kioskClient, cap })
				.transfer({
					itemType: item.type,
					itemId: item.objectId,
					address: currentAccount.address,
				})
				.finalize();

			return signAndExecute({ tx: txb });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}

/**
 * Mutation to delist an item.
 */
export function useDelistMutation({ onSuccess, onError }: MutationParams) {
	const currentAccount = useCurrentAccount();
	const { data: ownedKiosk } = useOwnedKiosk(currentAccount?.address);
	const { signAndExecute } = useTransactionExecution();
	const kioskClient = useKioskClient();

	return useMutation({
		mutationFn: async ({ item, kioskId }: { item: OwnedObjectType; kioskId: string }) => {
			// find active kiosk cap.
			const cap = findActiveCap(ownedKiosk?.caps, kioskId);

			if (!cap || !currentAccount?.address) throw new Error('Missing account, kiosk or kiosk cap');

			if (!item?.objectId) throw new Error('Missing item.');

			const txb = new TransactionBlock();

			new KioskTransaction({ transactionBlock: txb, kioskClient, cap })
				.delist({
					itemType: item.type,
					itemId: item.objectId,
				})
				.finalize();

			return signAndExecute({ tx: txb });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}

/**
 * Mutation to delist an item.
 */
export function usePurchaseItemMutation({ onSuccess, onError }: MutationParams) {
	const currentAccount = useCurrentAccount();
	const { data: ownedKiosk } = useOwnedKiosk(currentAccount?.address);
	const { signAndExecute } = useTransactionExecution();
	const kioskClient = useKioskClient();

	return useMutation({
		mutationFn: async ({ item, kioskId }: { item: OwnedObjectType; kioskId: string }) => {
			if (
				!item ||
				!item.listing?.price ||
				!kioskId ||
				!currentAccount?.address ||
				!ownedKiosk?.kioskId ||
				!ownedKiosk.kioskCap
			)
				throw new Error('Missing parameters');

			const cap = findActiveCap(ownedKiosk?.caps, ownedKiosk.kioskId);
			if (!cap || !currentAccount?.address) throw new Error('Missing account, kiosk or kiosk cap');

			const txb = new TransactionBlock();
			const kioskTx = new KioskTransaction({ transactionBlock: txb, kioskClient, cap });

			(
				await kioskTx.purchaseAndResolve({
					itemType: item.type,
					itemId: item.objectId,
					sellerKiosk: kioskId,
					price: item.listing!.price!,
				})
			).finalize();

			return await signAndExecute({ tx: txb });
		},
		onSuccess,
		onError: onError || defaultOnError,
	});
}
