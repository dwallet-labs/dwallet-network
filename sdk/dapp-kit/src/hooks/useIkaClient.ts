// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaClient } from '@ika-io/ika/client';
import { useContext } from 'react';

import { IkaClientContext } from '../components/IkaClientProvider.js';

export function useIkaClientContext() {
	const ikaClient = useContext(IkaClientContext);

	if (!ikaClient) {
		throw new Error(
			'Could not find IkaClientContext. Ensure that you have set up the IkaClientProvider',
		);
	}

	return ikaClient;
}

export function useIkaClient(): IkaClient {
	return useIkaClientContext().client;
}
