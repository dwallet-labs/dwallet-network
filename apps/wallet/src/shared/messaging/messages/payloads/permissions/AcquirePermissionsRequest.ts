// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { isBasePayload } from '_payloads';
import type { BasePayload, Payload } from '_payloads';

import type { PermissionType } from './PermissionType';

export interface AcquirePermissionsRequest extends BasePayload {
	type: 'acquire-permissions-request';
	permissions: readonly PermissionType[];
}

export function isAcquirePermissionsRequest(
	payload: Payload,
): payload is AcquirePermissionsRequest {
	return isBasePayload(payload) && payload.type === 'acquire-permissions-request';
}
