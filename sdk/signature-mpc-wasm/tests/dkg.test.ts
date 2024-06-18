// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { describe, it } from 'vitest';

import { initiate_dkg } from '../pkg';

describe('initiate_dkg', () => {
	it('should work', () => {
		let commitment = initiate_dkg();
		console.log(commitment);
	});
});
