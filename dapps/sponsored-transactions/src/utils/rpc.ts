// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, SuiClient } from '@mysten/sui.js/client';

export const client = new SuiClient({ url: getFullnodeUrl('testnet') });
