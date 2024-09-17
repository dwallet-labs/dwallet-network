// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraObjectChange } from './generated.js';

export type PeraObjectChangePublished = Extract<PeraObjectChange, { type: 'published' }>;
export type PeraObjectChangeTransferred = Extract<PeraObjectChange, { type: 'transferred' }>;
export type PeraObjectChangeMutated = Extract<PeraObjectChange, { type: 'mutated' }>;
export type PeraObjectChangeDeleted = Extract<PeraObjectChange, { type: 'deleted' }>;
export type PeraObjectChangeWrapped = Extract<PeraObjectChange, { type: 'wrapped' }>;
export type PeraObjectChangeCreated = Extract<PeraObjectChange, { type: 'created' }>;
