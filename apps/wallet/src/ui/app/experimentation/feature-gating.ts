// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { GrowthBook } from '@growthbook/growthbook';

// This is a separate growthbook instance for the wallet UI, with flag values synced from the service worker.
export const growthbook = new GrowthBook();
