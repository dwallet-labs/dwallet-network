// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@ika-io/ika/bcs';

export const BcsOrder = bcs.struct('Order', {
	orderId: bcs.u64(),
	clientOrderId: bcs.u64(),
	price: bcs.u64(),
	originalQuantity: bcs.u64(),
	quantity: bcs.u64(),
	isBid: bcs.bool(),
	owner: bcs.Address,
	expireTimestamp: bcs.u64(),
	selfMatchingPrevention: bcs.u8(),
});
