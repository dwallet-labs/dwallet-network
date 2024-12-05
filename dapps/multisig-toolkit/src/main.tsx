// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import '@mysten/dapp-kit/dist/index.css';
import './index.css';
import '@fontsource-variable/inter';
import '@fontsource-variable/red-hat-mono';

import { IkaClientProvider, WalletProvider } from '@mysten/dapp-kit';
import { getFullnodeUrl } from '@ika-io/ika/client';
import { QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { queryClient } from './lib/queryClient';
import { router } from './routes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<IkaClientProvider
				defaultNetwork="ika:mainnet"
				networks={{
					'ika:testnet': { url: getFullnodeUrl('testnet') },
					'ika:mainnet': { url: getFullnodeUrl('mainnet') },
					'ika:devnet': { url: getFullnodeUrl('devnet') },
				}}
			>
				<WalletProvider>
					<RouterProvider router={router} />
				</WalletProvider>
			</IkaClientProvider>
		</QueryClientProvider>
	</React.StrictMode>,
);
