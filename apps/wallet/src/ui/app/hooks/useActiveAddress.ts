// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useActiveAccount } from './useActiveAccount';

export function useActiveAddress() {
	return useActiveAccount()?.address || null;
}
