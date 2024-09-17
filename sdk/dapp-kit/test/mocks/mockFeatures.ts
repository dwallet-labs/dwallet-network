// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IdentifierRecord, PeraFeatures, PeraSignMessageFeature } from '@mysten/wallet-standard';

export const signMessageFeature: PeraSignMessageFeature = {
	'pera:signMessage': {
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

export const peraFeatures: PeraFeatures = {
	...signMessageFeature,
	'pera:signPersonalMessage': {
		version: '1.0.0',
		signPersonalMessage: vi.fn(),
	},
	'pera:signTransactionBlock': {
		version: '1.0.0',
		signTransactionBlock: vi.fn(),
	},
	'pera:signTransaction': {
		version: '2.0.0',
		signTransaction: vi.fn(),
	},
	'pera:signAndExecuteTransactionBlock': {
		version: '1.0.0',
		signAndExecuteTransactionBlock: vi.fn(),
	},
	'pera:signAndExecuteTransaction': {
		version: '2.0.0',
		signAndExecuteTransaction: vi.fn(),
	},
	'pera:reportTransactionEffects': {
		version: '1.0.0',
		reportTransactionEffects: vi.fn(),
	},
};
