// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { TransactionBlock } from '@dwallet-network/dwallet.js/transactions';
import { SUI_SYSTEM_STATE_OBJECT_ID } from '@dwallet-network/dwallet.js/utils';

export function createStakeTransaction(amount: bigint, validator: string) {
	const tx = new TransactionBlock();
	const stakeCoin = tx.splitCoins(tx.gas, [amount]);
	tx.moveCall({
		target: '0x3::dwallet_system::request_add_stake',
		arguments: [
			tx.sharedObjectRef({
				objectId: SUI_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
			}),
			stakeCoin,
			tx.pure.address(validator),
		],
	});
	return tx;
}

export function createUnstakeTransaction(stakedSuiId: string) {
	const tx = new TransactionBlock();
	tx.moveCall({
		target: '0x3::dwallet_system::request_withdraw_stake',
		arguments: [tx.object(SUI_SYSTEM_STATE_OBJECT_ID), tx.object(stakedSuiId)],
	});
	return tx;
}
