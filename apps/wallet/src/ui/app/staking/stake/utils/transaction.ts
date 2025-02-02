// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Transaction } from '@ika-io/ika/transactions';
import { IKA_SYSTEM_STATE_OBJECT_ID } from '@ika-io/ika/utils';

export function createStakeTransaction(amount: bigint, validator: string) {
	const tx = new Transaction();
	const stakeCoin = tx.splitCoins(tx.gas, [amount]);
	tx.moveCall({
		target: '0x3::ika_system::request_add_stake',
		arguments: [
			tx.sharedObjectRef({
				objectId: IKA_SYSTEM_STATE_OBJECT_ID,
				initialSharedVersion: 1,
				mutable: true,
			}),
			stakeCoin,
			tx.pure.address(validator),
		],
	});
	return tx;
}

export function createUnstakeTransaction(stakedIkaId: string) {
	const tx = new Transaction();
	tx.moveCall({
		target: '0x3::ika_system::request_withdraw_stake',
		arguments: [tx.object(IKA_SYSTEM_STATE_OBJECT_ID), tx.object(stakedIkaId)],
	});
	return tx;
}
