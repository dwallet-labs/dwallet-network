// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { TransactionBlock, TransactionObjectArgument } from '@dwallet/dwallet.js/transactions';

import { ObjectArgument } from '../types';

export function convertToPersonalTx(
	tx: TransactionBlock,
	kiosk: ObjectArgument,
	kioskOwnerCap: ObjectArgument,
	packageId: string,
): TransactionObjectArgument {
	const personalKioskCap = tx.moveCall({
		target: `${packageId}::personal_kiosk::new`,
		arguments: [tx.object(kiosk), tx.object(kioskOwnerCap)],
	});

	return personalKioskCap;
}

/**
 * Transfers the personal kiosk Cap to the sender.
 */
export function transferPersonalCapTx(
	tx: TransactionBlock,
	personalKioskCap: TransactionObjectArgument,
	packageId: string,
) {
	tx.moveCall({
		target: `${packageId}::personal_kiosk::transfer_to_sender`,
		arguments: [personalKioskCap],
	});
}
