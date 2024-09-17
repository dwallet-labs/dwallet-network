// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraClient } from '@pera-io/pera/client';
import { useContext } from 'react';

import { PeraClientContext } from '../components/PeraClientProvider.js';

export function usePeraClientContext() {
	const peraClient = useContext(PeraClientContext);

	if (!peraClient) {
		throw new Error(
			'Could not find PeraClientContext. Ensure that you have set up the PeraClientProvider',
		);
	}

	return peraClient;
}

export function usePeraClient(): PeraClient {
	return usePeraClientContext().client;
}
