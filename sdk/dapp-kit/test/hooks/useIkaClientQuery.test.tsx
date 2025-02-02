// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { getFullnodeUrl, IkaClient } from '@ika-io/ika/client';
import { renderHook, waitFor } from '@testing-library/react';

import { useIkaClientQuery } from '../../src/hooks/useIkaClientQuery.js';
import { createWalletProviderContextWrapper } from '../test-utils.js';

describe('useIkaClientQuery', () => {
	it('should fetch data', async () => {
		const ikaClient = new IkaClient({ url: getFullnodeUrl('mainnet') });
		const wrapper = createWalletProviderContextWrapper({}, ikaClient);

		const queryTransactionBlocks = vi.spyOn(ikaClient, 'queryTransactionBlocks');

		queryTransactionBlocks.mockResolvedValueOnce({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});

		const { result } = renderHook(
			() =>
				useIkaClientQuery('queryTransactionBlocks', {
					filter: {
						FromAddress: '0x123',
					},
				}),
			{ wrapper },
		);

		expect(result.current.isLoading).toBe(true);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toBe(undefined);
		expect(queryTransactionBlocks).toHaveBeenCalledWith({
			filter: {
				FromAddress: '0x123',
			},
		});

		await waitFor(() => expect(result.current.isSuccess).toBe(true));

		expect(result.current.isLoading).toBe(false);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toEqual({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});
	});
});
