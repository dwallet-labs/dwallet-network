// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { BasePayload } from '_payloads';

export interface GetAccountResponse extends BasePayload {
	type: 'get-account-response';
	accounts: { address: string; publicKey: string | null; nickname: string | null }[];
}
