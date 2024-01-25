// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SuiObjectChangeTypes } from './types';

export const ObjectChangeLabels = {
	created: 'Created',
	mutated: 'Updated',
	transferred: 'Transfer',
	published: 'Publish',
	deleted: 'Deleted',
	wrapped: 'Wrap',
};

export function getObjectChangeLabel(type: SuiObjectChangeTypes) {
	return ObjectChangeLabels[type];
}
