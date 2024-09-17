// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type PeraSignMessageOutput } from '@mysten/wallet-standard';

import { isBasePayload, type BasePayload } from '../BasePayload';
import { type Payload } from '../Payload';

export interface SignMessageRequest extends BasePayload {
	type: 'sign-message-request';
	args?: {
		message: string; // base64
		accountAddress: string;
	};
	return?: PeraSignMessageOutput;
}

export function isSignMessageRequest(payload: Payload): payload is SignMessageRequest {
	return isBasePayload(payload) && payload.type === 'sign-message-request';
}
