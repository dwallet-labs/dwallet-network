// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { BalanceChangeSummary } from './getBalanceChangeSummary';
import { GasSummaryType } from './getGasSummary';
import { ObjectChangeSummary } from './getObjectChangeSummary';

export type TransactionSummary = {
	digest?: string;
	sender?: string;
	timestamp?: string | null;
	balanceChanges: BalanceChangeSummary;
	gas?: GasSummaryType;
	objectSummary: ObjectChangeSummary | null;
} | null;

export type SuiObjectChangeTypes =
	| 'published'
	| 'transferred'
	| 'mutated'
	| 'deleted'
	| 'wrapped'
	| 'created';
