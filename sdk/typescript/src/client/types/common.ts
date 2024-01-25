// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { CallArg } from '../../bcs/index.js';

export type SuiJsonValue = boolean | number | string | CallArg | Array<SuiJsonValue>;
export type Order = 'ascending' | 'descending';
export type Unsubscribe = () => Promise<boolean>;
