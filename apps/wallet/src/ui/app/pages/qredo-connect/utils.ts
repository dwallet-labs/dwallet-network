// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type UIQredoPendingRequest } from '_src/background/qredo/types';

export function isUntrustedQredoConnect({ apiUrl, origin }: UIQredoPendingRequest) {
	try {
		return new URL(origin).protocol !== 'https:' || new URL(apiUrl).protocol !== 'https:';
	} catch (e) {
		return false;
	}
}
