// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import '@mysten/dapp-kit/dist/index.css';
import './index.css';
import '@fontsource-variable/inter';
import '@fontsource-variable/red-hat-mono';

import { PeraClientProvider, WalletProvider } from '@mysten/dapp-kit';
import { getFullnodeUrl } from '@pera-io/pera/client';
import { QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { queryClient } from './lib/queryClient';
import { router } from './routes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<PeraClientProvider
				defaultNetwork="pera:mainnet"
				networks={{
					'pera:testnet': { url: getFullnodeUrl('testnet') },
					'pera:mainnet': { url: getFullnodeUrl('mainnet') },
					'pera:devnet': { url: getFullnodeUrl('devnet') },
				}}
			>
				<WalletProvider>
					<RouterProvider router={router} />
				</WalletProvider>
			</PeraClientProvider>
		</QueryClientProvider>
	</React.StrictMode>,
);
