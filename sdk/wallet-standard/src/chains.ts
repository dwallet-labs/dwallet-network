// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IdentifierString } from '@wallet-standard/core';

/** Pera Devnet */
export const PERA_DEVNET_CHAIN = 'pera:devnet';

/** Pera Testnet */
export const PERA_TESTNET_CHAIN = 'pera:testnet';

/** Pera Localnet */
export const PERA_LOCALNET_CHAIN = 'pera:localnet';

/** Pera Mainnet */
export const PERA_MAINNET_CHAIN = 'pera:mainnet';

export const PERA_CHAINS = [
	PERA_DEVNET_CHAIN,
	PERA_TESTNET_CHAIN,
	PERA_LOCALNET_CHAIN,
	PERA_MAINNET_CHAIN,
] as const;

export type PeraChain =
	| typeof PERA_DEVNET_CHAIN
	| typeof PERA_TESTNET_CHAIN
	| typeof PERA_LOCALNET_CHAIN
	| typeof PERA_MAINNET_CHAIN;

/**
 * Utility that returns whether or not a chain identifier is a valid Pera chain.
 * @param chain a chain identifier in the form of `${string}:{$string}`
 */
export function isPeraChain(chain: IdentifierString): chain is PeraChain {
	return PERA_CHAINS.includes(chain as PeraChain);
}
