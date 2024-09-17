// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Transaction } from '@pera-io/pera/transactions';
import { PERA_SYSTEM_STATE_OBJECT_ID } from '@pera-io/pera/utils';

export function createStakeTransaction(amount: bigint, validator: string) {
	const tx = new Transaction();
	const stakeCoin = tx.splitCoins(tx.gas, [amount]);
	tx.moveCall({
		target: '0x3::pera_system::request_add_stake',
		arguments: [
			tx.sharedObjectRef({
				objectId: PERA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
			}),
			stakeCoin,
			tx.pure.address(validator),
		],
	});
	return tx;
}

export function createUnstakeTransaction(stakedPeraId: string) {
	const tx = new Transaction();
	tx.moveCall({
		target: '0x3::pera_system::request_withdraw_stake',
		arguments: [tx.object(PERA_SYSTEM_STATE_OBJECT_ID), tx.object(stakedPeraId)],
	});
	return tx;
}
