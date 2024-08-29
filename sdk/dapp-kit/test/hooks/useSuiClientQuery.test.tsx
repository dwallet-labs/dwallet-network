// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { getFullnodeUrl, DWalletClient } from '@dwallet-network/dwallet.js/client';
import { renderHook, waitFor } from '@testing-library/react';

import { useSuiClientQuery } from '../../src/hooks/useSuiClientQuery.js';
import { createWalletProviderContextWrapper } from '../test-utils.js';

describe('useSuiClientQuery', () => {
	it('should fetch data', async () => {
		const suiClient = new DWalletClient({ url: getFullnodeUrl('mainnet') });
		const wrapper = createWalletProviderContextWrapper({}, suiClient);

		const queryTransactionBlocks = vi.spyOn(suiClient, 'queryTransactionBlocks');

		queryTransactionBlocks.mockResolvedValueOnce({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});

		const { result } = renderHook(
			() =>
				useSuiClientQuery('queryTransactionBlocks', {
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
