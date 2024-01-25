// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { webcrypto } from 'crypto';

if (!globalThis.crypto) {
	globalThis.crypto = webcrypto as Crypto;
}

// Create a fake chrome object so that the webextension polyfill can load:
globalThis.chrome = {
	runtime: {
		id: 'some-test-id-from-test-setup',
	},
};
