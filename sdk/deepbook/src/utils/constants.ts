// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { normalizeStructTag, PERA_TYPE_ARG } from '@pera-io/pera/utils';

export const PACKAGE_ID = '0xdee9';

export const MODULE_CLOB = 'clob_v2';

export const MODULE_CUSTODIAN = 'custodian_v2';

export const CREATION_FEE = 100 * 1e9;

export const NORMALIZED_PERA_COIN_TYPE = normalizeStructTag(PERA_TYPE_ARG);

export const ORDER_DEFAULT_EXPIRATION_IN_MS = 1000 * 60 * 60 * 24; // 24 hours

export const FLOAT_SCALING_FACTOR = 1_000_000_000n;
