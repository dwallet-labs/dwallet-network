// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { getFullnodeUrl, PeraClient } from '@pera-io/pera/client';
import { renderHook } from '@testing-library/react';

import { usePeraClient } from '../../src/index.js';
import { createPeraClientContextWrapper } from '../test-utils.js';

describe('usePeraClient', () => {
	test('throws without a PeraClientContext', () => {
		expect(() => renderHook(() => usePeraClient())).toThrowError(
			'Could not find PeraClientContext. Ensure that you have set up the PeraClientProvider',
		);
	});

	test('returns a PeraClient', () => {
		const peraClient = new PeraClient({ url: getFullnodeUrl('localnet') });
		const wrapper = createPeraClientContextWrapper(peraClient);
		const { result } = renderHook(() => usePeraClient(), { wrapper });

		expect(result.current).toBe(peraClient);
	});
});
