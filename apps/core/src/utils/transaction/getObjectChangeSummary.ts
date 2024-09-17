// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import {
	DisplayFieldsResponse,
	PeraObjectChange,
	PeraObjectChangeCreated,
	PeraObjectChangeDeleted,
	PeraObjectChangeMutated,
	PeraObjectChangePublished,
	PeraObjectChangeTransferred,
	PeraObjectChangeWrapped,
} from '@pera-io/pera/client';

import { groupByOwner } from './groupByOwner';
import { PeraObjectChangeTypes } from './types';

export type WithDisplayFields<T> = T & { display?: DisplayFieldsResponse };
export type PeraObjectChangeWithDisplay = WithDisplayFields<PeraObjectChange>;

export type ObjectChanges = {
	changesWithDisplay: PeraObjectChangeWithDisplay[];
	changes: PeraObjectChange[];
	ownerType: string;
};
export type ObjectChangesByOwner = Record<string, ObjectChanges>;

export type ObjectChangeSummary = {
	[K in PeraObjectChangeTypes]: ObjectChangesByOwner;
};

export const getObjectChangeSummary = (objectChanges: PeraObjectChangeWithDisplay[]) => {
	if (!objectChanges) return null;

	const mutated = objectChanges.filter(
		(change) => change.type === 'mutated',
	) as PeraObjectChangeMutated[];

	const created = objectChanges.filter(
		(change) => change.type === 'created',
	) as PeraObjectChangeCreated[];

	const transferred = objectChanges.filter(
		(change) => change.type === 'transferred',
	) as PeraObjectChangeTransferred[];

	const published = objectChanges.filter(
		(change) => change.type === 'published',
	) as PeraObjectChangePublished[];

	const wrapped = objectChanges.filter(
		(change) => change.type === 'wrapped',
	) as PeraObjectChangeWrapped[];

	const deleted = objectChanges.filter(
		(change) => change.type === 'deleted',
	) as PeraObjectChangeDeleted[];

	return {
		transferred: groupByOwner(transferred),
		created: groupByOwner(created),
		mutated: groupByOwner(mutated),
		published: groupByOwner(published),
		wrapped: groupByOwner(wrapped),
		deleted: groupByOwner(deleted),
	};
};
