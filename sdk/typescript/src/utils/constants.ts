// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { normalizePeraObjectId } from './pera-types.js';

export const PERA_DECIMALS = 9;
export const NPERA_PER_PERA = BigInt(1000000000);

export const MOVE_STDLIB_ADDRESS = '0x1';
export const PERA_FRAMEWORK_ADDRESS = '0x2';
export const PERA_SYSTEM_ADDRESS = '0x3';
export const PERA_CLOCK_OBJECT_ID = normalizePeraObjectId('0x6');
export const PERA_SYSTEM_MODULE_NAME = 'pera_system';
export const PERA_TYPE_ARG = `${PERA_FRAMEWORK_ADDRESS}::pera::PERA`;
export const PERA_SYSTEM_STATE_OBJECT_ID: string = normalizePeraObjectId('0x5');
