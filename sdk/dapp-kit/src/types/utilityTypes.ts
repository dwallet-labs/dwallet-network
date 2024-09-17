// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export type PartialBy<T, K extends keyof T> = Omit<T, K> & Partial<T>;
