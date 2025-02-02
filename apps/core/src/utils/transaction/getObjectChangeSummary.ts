// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import {
	DisplayFieldsResponse,
	IkaObjectChange,
	IkaObjectChangeCreated,
	IkaObjectChangeDeleted,
	IkaObjectChangeMutated,
	IkaObjectChangePublished,
	IkaObjectChangeTransferred,
	IkaObjectChangeWrapped,
} from '@ika-io/ika/client';

import { groupByOwner } from './groupByOwner';
import { IkaObjectChangeTypes } from './types';

export type WithDisplayFields<T> = T & { display?: DisplayFieldsResponse };
export type IkaObjectChangeWithDisplay = WithDisplayFields<IkaObjectChange>;

export type ObjectChanges = {
	changesWithDisplay: IkaObjectChangeWithDisplay[];
	changes: IkaObjectChange[];
	ownerType: string;
};
export type ObjectChangesByOwner = Record<string, ObjectChanges>;

export type ObjectChangeSummary = {
	[K in IkaObjectChangeTypes]: ObjectChangesByOwner;
};

export const getObjectChangeSummary = (objectChanges: IkaObjectChangeWithDisplay[]) => {
	if (!objectChanges) return null;

	const mutated = objectChanges.filter(
		(change) => change.type === 'mutated',
	) as IkaObjectChangeMutated[];

	const created = objectChanges.filter(
		(change) => change.type === 'created',
	) as IkaObjectChangeCreated[];

	const transferred = objectChanges.filter(
		(change) => change.type === 'transferred',
	) as IkaObjectChangeTransferred[];

	const published = objectChanges.filter(
		(change) => change.type === 'published',
	) as IkaObjectChangePublished[];

	const wrapped = objectChanges.filter(
		(change) => change.type === 'wrapped',
	) as IkaObjectChangeWrapped[];

	const deleted = objectChanges.filter(
		(change) => change.type === 'deleted',
	) as IkaObjectChangeDeleted[];

	return {
		transferred: groupByOwner(transferred),
		created: groupByOwner(created),
		mutated: groupByOwner(mutated),
		published: groupByOwner(published),
		wrapped: groupByOwner(wrapped),
		deleted: groupByOwner(deleted),
	};
};
