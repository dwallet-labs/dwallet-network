// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { getFullnodeUrl, PeraClient } from '@pera-io/pera/client';

export const client = new PeraClient({ url: getFullnodeUrl('testnet') });
