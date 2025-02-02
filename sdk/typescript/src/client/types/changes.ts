// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { IkaObjectChange } from './generated.js';

export type IkaObjectChangePublished = Extract<IkaObjectChange, { type: 'published' }>;
export type IkaObjectChangeTransferred = Extract<IkaObjectChange, { type: 'transferred' }>;
export type IkaObjectChangeMutated = Extract<IkaObjectChange, { type: 'mutated' }>;
export type IkaObjectChangeDeleted = Extract<IkaObjectChange, { type: 'deleted' }>;
export type IkaObjectChangeWrapped = Extract<IkaObjectChange, { type: 'wrapped' }>;
export type IkaObjectChangeCreated = Extract<IkaObjectChange, { type: 'created' }>;
