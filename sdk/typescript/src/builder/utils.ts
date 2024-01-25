// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { Struct } from 'superstruct';
import { create as superstructCreate } from 'superstruct';

export function create<T, S>(value: T, struct: Struct<T, S>): T {
	return superstructCreate(value, struct);
}
