// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@dwallet-network/dwallet.js/bcs';

import {
	KIOSK_PURCHASE_CAP,
	KIOSK_TYPE,
	TRANSFER_POLICY_CREATED_EVENT,
	TRANSFER_POLICY_TYPE,
} from './types';

// Register the `Kiosk` struct for faster queries.
bcs.registerStructType(KIOSK_TYPE, {
	id: 'address',
	profits: 'u64',
	owner: 'address',
	itemCount: 'u32',
	allowExtensions: 'bool',
});

// Register the `PurchaseCap` for faster queries.
bcs.registerStructType(KIOSK_PURCHASE_CAP, {
	id: 'address',
	kioskId: 'address',
	itemId: 'address',
	minPrice: 'u64',
});

// Register the `TransferPolicyCreated` event data.
bcs.registerStructType(TRANSFER_POLICY_CREATED_EVENT, {
	id: 'address',
});

bcs.registerStructType(TRANSFER_POLICY_TYPE, {
	id: 'address',
	balance: 'u64',
	rules: ['vector', 'string'],
});

export { bcs };
