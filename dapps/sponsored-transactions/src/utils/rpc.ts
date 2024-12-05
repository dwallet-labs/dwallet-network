// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getFullnodeUrl, IkaClient } from '@ika-io/ika/client';

export const client = new IkaClient({ url: getFullnodeUrl('testnet') });
