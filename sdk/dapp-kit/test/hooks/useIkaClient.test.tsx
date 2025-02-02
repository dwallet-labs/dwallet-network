// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { getFullnodeUrl, IkaClient } from '@ika-io/ika/client';
import { renderHook } from '@testing-library/react';

import { useIkaClient } from '../../src/index.js';
import { createIkaClientContextWrapper } from '../test-utils.js';

describe('useIkaClient', () => {
	test('throws without a IkaClientContext', () => {
		expect(() => renderHook(() => useIkaClient())).toThrowError(
			'Could not find IkaClientContext. Ensure that you have set up the IkaClientProvider',
		);
	});

	test('returns a IkaClient', () => {
		const ikaClient = new IkaClient({ url: getFullnodeUrl('localnet') });
		const wrapper = createIkaClientContextWrapper(ikaClient);
		const { result } = renderHook(() => useIkaClient(), { wrapper });

		expect(result.current).toBe(ikaClient);
	});
});
