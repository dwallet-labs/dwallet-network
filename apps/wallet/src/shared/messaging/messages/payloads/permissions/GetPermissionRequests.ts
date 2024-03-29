// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { isBasePayload } from '_payloads';
import type { BasePayload, Payload } from '_payloads';

export interface GetPermissionRequests extends BasePayload {
	type: 'get-permission-requests';
}

export function isGetPermissionRequests(payload: Payload): payload is GetPermissionRequests {
	return isBasePayload(payload) && payload.type === 'get-permission-requests';
}
