// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { SuiObjectChange } from '@dwallet-network/dwallet.js/client';

export const getOwnerType = (change: SuiObjectChange) => {
	if (!('owner' in change)) return '';
	if (typeof change.owner === 'object') {
		if ('AddressOwner' in change.owner) return 'AddressOwner';
		if ('ObjectOwner' in change.owner) return 'ObjectOwner';
		if ('Shared' in change.owner) return 'Shared';
	}
	return change.owner;
};
