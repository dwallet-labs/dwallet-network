// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import bs58 from 'bs58';

export const toBase58 = (buffer: Uint8Array) => bs58.encode(buffer);
export const fromBase58 = (str: string) => bs58.decode(str);

/** @deprecated use toBase58 instead */
export const toB58 = toBase58;

/** @deprecated use fromBase58 instead */
export const fromB58 = fromBase58;
