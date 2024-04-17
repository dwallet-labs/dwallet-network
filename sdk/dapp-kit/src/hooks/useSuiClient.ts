// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SuiClient } from '@dwallet-network/dwallet.js/client';
import { useContext } from 'react';

import { SuiClientContext } from '../components/SuiClientProvider.js';

export function useSuiClientContext() {
	const suiClient = useContext(SuiClientContext);

	if (!suiClient) {
		throw new Error(
			'Could not find SuiClientContext. Ensure that you have set up the SuiClientProvider',
		);
	}

	return suiClient;
}

export function useSuiClient(): SuiClient {
	return useSuiClientContext().client;
}
