// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaObjectChange } from '@ika-io/ika/client';

export const getOwnerType = (change: IkaObjectChange) => {
	if (!('owner' in change)) return '';
	if (typeof change.owner === 'object') {
		if ('AddressOwner' in change.owner) return 'AddressOwner';
		if ('ObjectOwner' in change.owner) return 'ObjectOwner';
		if ('Shared' in change.owner) return 'Shared';
	}
	return change.owner;
};
