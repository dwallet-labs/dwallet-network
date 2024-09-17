// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ObjectChangesByOwner, PeraObjectChangeWithDisplay } from './getObjectChangeSummary';
import { getOwnerType } from './getOwnerType';

const getOwner = (change: PeraObjectChangeWithDisplay) => {
	// published changes don't have an owner
	if ('owner' in change && typeof change.owner === 'object') {
		if ('AddressOwner' in change.owner) return change.owner.AddressOwner;
		if ('ObjectOwner' in change.owner) return change.owner.ObjectOwner;
		if ('Shared' in change.owner) return change.objectId;
	}
	return '';
};

export const groupByOwner = (changes: PeraObjectChangeWithDisplay[]) =>
	changes.reduce((acc, change) => {
		const owner = getOwner(change);
		if (!acc[owner])
			acc[owner] = {
				changesWithDisplay: [],
				changes: [],
				ownerType: getOwnerType(change),
			};

		if (change.display?.data) {
			acc[owner].changesWithDisplay.push(change);
		} else {
			acc[owner].changes.push(change);
		}

		return acc;
	}, {} as ObjectChangesByOwner);
