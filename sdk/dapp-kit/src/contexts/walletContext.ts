// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { createContext } from 'react';

import type { WalletStore } from '../walletStore.js';

export const WalletContext = createContext<WalletStore | null>(null);
