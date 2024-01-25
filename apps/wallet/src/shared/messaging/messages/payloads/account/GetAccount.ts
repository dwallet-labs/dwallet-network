// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { isBasePayload } from '_payloads';
import type { BasePayload, Payload } from '_payloads';

export interface GetAccount extends BasePayload {
	type: 'get-account';
}

export function isGetAccount(payload: Payload): payload is GetAccount {
	return isBasePayload(payload) && payload.type === 'get-account';
}
