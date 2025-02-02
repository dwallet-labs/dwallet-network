// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IdentifierRecord, IkaFeatures, IkaSignMessageFeature } from '@mysten/wallet-standard';

export const signMessageFeature: IkaSignMessageFeature = {
	'ika:signMessage': {
		version: '1.0.0',
		signMessage: vi.fn(),
	},
};

export const superCoolFeature: IdentifierRecord<unknown> = {
	'my-dapp:super-cool-feature': {
		version: '1.0.0',
		superCoolFeature: vi.fn(),
	},
};

export const ikaFeatures: IkaFeatures = {
	...signMessageFeature,
	'ika:signPersonalMessage': {
		version: '1.0.0',
		signPersonalMessage: vi.fn(),
	},
	'ika:signTransactionBlock': {
		version: '1.0.0',
		signTransactionBlock: vi.fn(),
	},
	'ika:signTransaction': {
		version: '2.0.0',
		signTransaction: vi.fn(),
	},
	'ika:signAndExecuteTransactionBlock': {
		version: '1.0.0',
		signAndExecuteTransactionBlock: vi.fn(),
	},
	'ika:signAndExecuteTransaction': {
		version: '2.0.0',
		signAndExecuteTransaction: vi.fn(),
	},
	'ika:reportTransactionEffects': {
		version: '1.0.0',
		reportTransactionEffects: vi.fn(),
	},
};
