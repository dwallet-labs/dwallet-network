// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IdentifierString } from '@wallet-standard/core';

/** Ika Devnet */
export const IKA_DEVNET_CHAIN = 'ika:devnet';

/** Ika Testnet */
export const IKA_TESTNET_CHAIN = 'ika:testnet';

/** Ika Localnet */
export const IKA_LOCALNET_CHAIN = 'ika:localnet';

/** Ika Mainnet */
export const IKA_MAINNET_CHAIN = 'ika:mainnet';

export const IKA_CHAINS = [
	IKA_DEVNET_CHAIN,
	IKA_TESTNET_CHAIN,
	IKA_LOCALNET_CHAIN,
	IKA_MAINNET_CHAIN,
] as const;

export type IkaChain =
	| typeof IKA_DEVNET_CHAIN
	| typeof IKA_TESTNET_CHAIN
	| typeof IKA_LOCALNET_CHAIN
	| typeof IKA_MAINNET_CHAIN;

/**
 * Utility that returns whether or not a chain identifier is a valid Ika chain.
 * @param chain a chain identifier in the form of `${string}:{$string}`
 */
export function isIkaChain(chain: IdentifierString): chain is IkaChain {
	return IKA_CHAINS.includes(chain as IkaChain);
}
