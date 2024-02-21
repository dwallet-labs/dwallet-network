// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { WalletWithFeatures, WalletWithRequiredFeatures } from '@mysten/wallet-standard';
import type { ReactNode } from 'react';
import { useRef } from 'react';
import type { StateStorage } from 'zustand/middleware';

import {
	DEFAULT_REQUIRED_FEATURES,
	DEFAULT_STORAGE,
	DEFAULT_STORAGE_KEY,
	SUI_WALLET_NAME,
} from '../constants/walletDefaults.js';
import { WalletContext } from '../contexts/walletContext.js';
import { useAutoConnectWallet } from '../hooks/wallet/useAutoConnectWallet.js';
import { useUnsafeBurnerWallet } from '../hooks/wallet/useUnsafeBurnerWallet.js';
import { useWalletPropertiesChanged } from '../hooks/wallet/useWalletPropertiesChanged.js';
import { useWalletsChanged } from '../hooks/wallet/useWalletsChanged.js';
import { lightTheme } from '../themes/lightTheme.js';
import type { Theme } from '../themes/themeContract.js';
import { createInMemoryStore } from '../utils/stateStorage.js';
import { getRegisteredWallets } from '../utils/walletUtils.js';
import { createWalletStore } from '../walletStore.js';
import { InjectedThemeStyles } from './styling/InjectedThemeStyles.js';

export type WalletProviderProps = {
	/** A list of wallets that are sorted to the top of the wallet list, if they are available to connect to. By default, wallets are sorted by the order they are loaded in. */
	preferredWallets?: string[];

	/** A list of features that are required for the dApp to function. This filters the list of wallets presented to users when selecting a wallet to connect from, ensuring that only wallets that meet the dApps requirements can connect. */
	requiredFeatures?: (keyof WalletWithRequiredFeatures['features'])[];

	/** Enables the development-only unsafe burner wallet, which can be useful for testing. */
	enableUnsafeBurner?: boolean;

	/** Enables automatically reconnecting to the most recently used wallet account upon mounting. */
	autoConnect?: boolean;

	/** Configures how the most recently connected to wallet account is stored. Set to `null` to disable persisting state entirely. Defaults to using localStorage if it is available. */
	storage?: StateStorage | null;

	/** The key to use to store the most recently connected wallet account. */
	storageKey?: string;

	/** The theme to use for styling UI components. Defaults to using the light theme. */
	theme?: Theme | null;

	children: ReactNode;
};

export type { WalletWithFeatures };

export function WalletProvider({
	preferredWallets = [SUI_WALLET_NAME],
	requiredFeatures = DEFAULT_REQUIRED_FEATURES,
	storage = DEFAULT_STORAGE,
	storageKey = DEFAULT_STORAGE_KEY,
	enableUnsafeBurner = false,
	autoConnect = false,
	theme = lightTheme,
	children,
}: WalletProviderProps) {
	const storeRef = useRef(
		createWalletStore({
			autoConnectEnabled: autoConnect,
			wallets: getRegisteredWallets(preferredWallets, requiredFeatures),
			storage: storage || createInMemoryStore(),
			storageKey,
		}),
	);

	return (
		<WalletContext.Provider value={storeRef.current}>
			<WalletConnectionManager
				preferredWallets={preferredWallets}
				requiredFeatures={requiredFeatures}
				enableUnsafeBurner={enableUnsafeBurner}
			>
				{/* TODO: We ideally don't want to inject styles if people aren't using the UI components */}
				{theme ? <InjectedThemeStyles theme={theme} /> : null}
				{children}
			</WalletConnectionManager>
		</WalletContext.Provider>
	);
}

type WalletConnectionManagerProps = Required<
	Pick<
		WalletProviderProps,
		'preferredWallets' | 'requiredFeatures' | 'enableUnsafeBurner' | 'children'
	>
>;

function WalletConnectionManager({
	preferredWallets,
	requiredFeatures,
	enableUnsafeBurner,
	children,
}: WalletConnectionManagerProps) {
	useWalletsChanged(preferredWallets, requiredFeatures);
	useWalletPropertiesChanged();
	useUnsafeBurnerWallet(enableUnsafeBurner);
	useAutoConnectWallet();

	return children;
}
