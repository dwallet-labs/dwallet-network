// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { TransactionBlock } from '@dwallet-network/dwallet.js/transactions';
import { expect } from 'vitest';

import {
	KioskClient,
	KioskOwnerCap,
	KioskTransaction,
	percentageToBasisPoints,
	TransferPolicyTransaction,
} from '../../src';
import { executeTransactionBlock, getPublisherObject, TestToolbox } from './setup';

// Creates a fresh transfer policy for Heroes and attaches all the rules.
export async function prepareHeroRuleset({
	toolbox,
	heroPackageId,
	kioskClient,
}: {
	toolbox: TestToolbox;
	heroPackageId: string;
	kioskClient: KioskClient;
}) {
	/// Do a full rule setup for `Hero` type.
	const publisher = await getPublisherObject(toolbox);
	const txb = new TransactionBlock();
	const tpTx = new TransferPolicyTransaction({ kioskClient, transactionBlock: txb });

	await tpTx.create({
		type: `${heroPackageId}::hero::Hero`,
		publisher,
	});

	tpTx
		.addLockRule()
		.addFloorPriceRule(1000n)
		.addRoyaltyRule(percentageToBasisPoints(10), 100)
		.addPersonalKioskRule()
		.shareAndTransferCap(toolbox.address());

	await executeTransactionBlock(toolbox, txb);
}

// Creates a fresh transfer policy for Heroes and attaches all the rules.
export async function prepareVillainTransferPolicy({
	toolbox,
	heroPackageId,
	kioskClient,
}: {
	toolbox: TestToolbox;
	heroPackageId: string;
	kioskClient: KioskClient;
}) {
	/// Do a plain TP creation for `Villain` type.
	const publisher = await getPublisherObject(toolbox);
	const txb = new TransactionBlock();
	const tpTx = new TransferPolicyTransaction({ kioskClient, transactionBlock: txb });

	await tpTx.createAndShare({
		type: `${heroPackageId}::hero::Villain`,
		publisher,
		address: toolbox.address(),
	});

	await executeTransactionBlock(toolbox, txb);
}

export async function testLockItemFlow(
	toolbox: TestToolbox,
	kioskClient: KioskClient,
	cap: KioskOwnerCap,
	itemType: string,
	itemId: string,
) {
	const txb = new TransactionBlock();
	const kioskTx = new KioskTransaction({ transactionBlock: txb, kioskClient, cap });

	const policies = await kioskClient.getTransferPolicies({ type: itemType });
	expect(policies).toHaveLength(1);

	kioskTx
		.lock({
			itemType,
			itemId,
			policy: policies[0].id,
		})
		.finalize();

	await executeTransactionBlock(toolbox, txb);
}

// A helper that does a full run for kiosk management.
export async function existingKioskManagementFlow(
	toolbox: TestToolbox,
	kioskClient: KioskClient,
	cap: KioskOwnerCap,
	itemType: string,
	itemId: string,
) {
	const txb = new TransactionBlock();
	const kioskTx = new KioskTransaction({ transactionBlock: txb, kioskClient, cap });

	kioskTx
		.place({
			itemType,
			item: itemId,
		})
		.list({
			itemType,
			itemId: itemId,
			price: 100000n,
		})
		.delist({
			itemType,
			itemId: itemId,
		});

	const item = kioskTx.take({
		itemType,
		itemId: itemId,
	});

	kioskTx
		.placeAndList({
			itemType,
			item,
			price: 100000n,
		})
		.delist({
			itemType,
			itemId: itemId,
		})
		.transfer({
			itemType,
			itemId: itemId,
			address: toolbox.address(),
		})
		.withdraw(toolbox.address())
		.finalize();

	await executeTransactionBlock(toolbox, txb);
}

/**
 * Lists an item for sale using one kiosk, and purchases it using another.
 * Depending on the rules, the buyer kiosk might have to be personal.
 */
export async function purchaseFlow(
	toolbox: TestToolbox,
	kioskClient: KioskClient,
	buyerCap: KioskOwnerCap,
	sellerCap: KioskOwnerCap,
	itemType: string,
	itemId: string,
) {
	/**
	 * Lists an item for sale
	 */
	const SALE_PRICE = 100000n;
	const sellTxb = new TransactionBlock();
	new KioskTransaction({ transactionBlock: sellTxb, kioskClient, cap: sellerCap })
		.placeAndList({
			itemType,
			item: itemId,
			price: SALE_PRICE,
		})
		.finalize();

	await executeTransactionBlock(toolbox, sellTxb);

	/**
	 * Purchases the item using a different kiosk (must be personal)
	 */
	const purchaseTxb = new TransactionBlock();
	const purchaseTx = new KioskTransaction({
		transactionBlock: purchaseTxb,
		kioskClient,
		cap: buyerCap,
	});

	(
		await purchaseTx.purchaseAndResolve({
			itemType,
			itemId,
			sellerKiosk: sellerCap.kioskId,
			price: SALE_PRICE,
		})
	).finalize();

	await executeTransactionBlock(toolbox, purchaseTxb);
}

export async function purchaseOnNewKiosk(
	toolbox: TestToolbox,
	kioskClient: KioskClient,
	sellerCap: KioskOwnerCap,
	itemType: string,
	itemId: string,
	personal?: boolean,
) {
	/**
	 * Lists an item for sale
	 */
	const SALE_PRICE = 100000n;
	const sellTxb = new TransactionBlock();
	new KioskTransaction({ transactionBlock: sellTxb, kioskClient, cap: sellerCap })
		.placeAndList({
			itemType,
			item: itemId,
			price: SALE_PRICE,
		})
		.finalize();

	await executeTransactionBlock(toolbox, sellTxb);

	/**
	 * Purchases the item using a different kiosk (must be personal)
	 */
	const purchaseTxb = new TransactionBlock();
	const purchaseTx = new KioskTransaction({ transactionBlock: purchaseTxb, kioskClient });

	// create personal kiosk (`true` means that we can use this kiosk for extra transactions)
	if (personal) purchaseTx.createPersonal(true);
	else purchaseTx.create();

	// do the purchase.
	await purchaseTx.purchaseAndResolve({
		itemType,
		itemId,
		sellerKiosk: sellerCap.kioskId,
		price: SALE_PRICE,
	});
	if (!personal) purchaseTx.shareAndTransferCap(toolbox.address());
	purchaseTx.finalize();

	await executeTransactionBlock(toolbox, purchaseTxb);
}
