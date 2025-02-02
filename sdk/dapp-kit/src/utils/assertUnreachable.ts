// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/**
 * Utility for compile-time exhaustiveness checking.
 */
export function assertUnreachable(value: never): never {
	throw new Error(`ERROR! Encountered an unexpected value: ${value}`);
}
