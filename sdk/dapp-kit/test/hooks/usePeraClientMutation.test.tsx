// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { getFullnodeUrl, PeraClient } from '@pera-io/pera/client';
import { act, renderHook, waitFor } from '@testing-library/react';

import { usePeraClientMutation } from '../../src/hooks/usePeraClientMutation.js';
import { createWalletProviderContextWrapper } from '../test-utils.js';

describe('usePeraClientMutation', () => {
	it('should fetch data', async () => {
		const peraClient = new PeraClient({ url: getFullnodeUrl('mainnet') });
		const wrapper = createWalletProviderContextWrapper({}, peraClient);

		const queryTransactionBlocks = vi.spyOn(peraClient, 'queryTransactionBlocks');

		queryTransactionBlocks.mockResolvedValueOnce({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});

		const { result } = renderHook(() => usePeraClientMutation('queryTransactionBlocks'), {
			wrapper,
		});

		act(() => {
			result.current.mutate({
				filter: {
					FromAddress: '0x123',
				},
			});
		});

		await waitFor(() => expect(result.current.status).toBe('success'));

		expect(queryTransactionBlocks).toHaveBeenCalledWith({
			filter: {
				FromAddress: '0x123',
			},
		});
		expect(result.current.isPending).toBe(false);
		expect(result.current.isError).toBe(false);
		expect(result.current.data).toEqual({
			data: [{ digest: '0x123' }],
			hasNextPage: true,
			nextCursor: 'page2',
		});
	});
});
