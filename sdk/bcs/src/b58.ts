// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import bs58 from 'bs58';

export const toB58 = (buffer: Uint8Array) => bs58.encode(buffer);
export const fromB58 = (str: string) => bs58.decode(str);
