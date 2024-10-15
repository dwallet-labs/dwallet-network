// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export type PeraNSContract = {
	packageId: string;
	perans: string;
	registry: string;
	reverseRegistry: string;
};

export type NameObject = {
	id: string;
	owner: string;
	targetAddress: string;
	avatar?: string;
	contentHash?: string;
};

export type DataFields = 'avatar' | 'contentHash';

export type NetworkType = 'devnet' | 'testnet';