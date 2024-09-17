// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraObjectChange } from '@pera-io/pera/client';

export const getOwnerType = (change: PeraObjectChange) => {
	if (!('owner' in change)) return '';
	if (typeof change.owner === 'object') {
		if ('AddressOwner' in change.owner) return 'AddressOwner';
		if ('ObjectOwner' in change.owner) return 'ObjectOwner';
		if ('Shared' in change.owner) return 'Shared';
	}
	return change.owner;
};
