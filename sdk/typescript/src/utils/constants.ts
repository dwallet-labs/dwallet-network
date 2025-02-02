// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { normalizeIkaObjectId } from './ika-types.js';

export const IKA_DECIMALS = 9;
export const NIKA_PER_IKA = BigInt(1000000000);

export const MOVE_STDLIB_ADDRESS = '0x1';
export const IKA_FRAMEWORK_ADDRESS = '0x2';
export const IKA_SYSTEM_ADDRESS = '0x3';
export const IKA_CLOCK_OBJECT_ID = normalizeIkaObjectId('0x6');
export const IKA_SYSTEM_MODULE_NAME = 'ika_system';
export const IKA_TYPE_ARG = `${IKA_FRAMEWORK_ADDRESS}::ika::IKA`;
export const IKA_SYSTEM_STATE_OBJECT_ID: string = normalizeIkaObjectId('0x5');
