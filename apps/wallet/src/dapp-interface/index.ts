// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { registerWallet } from '@mysten/wallet-standard';

import { PeraWallet } from './WalletStandardInterface';

registerWallet(new PeraWallet());
