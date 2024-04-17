// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PublicKey } from '@dwallet/dwallet.js/cryptography';
import type { ZkLoginSignatureInputs } from '@dwallet/dwallet.js/zklogin';

import type { AuthProvider } from '../EnokiFlow.js';

export interface GetAppApiInput {}
export interface GetAppApiResponse {
	authenticationProviders: {
		providerType: AuthProvider;
		clientId: string;
	}[];
}

export interface GetZkLoginApiInput {
	jwt: string;
}
export interface GetZkLoginApiResponse {
	address: string;
	salt: string;
}

export interface CreateZkLoginNonceApiInput {
	ephemeralPublicKey: PublicKey;
}
export interface CreateZkLoginNonceApiResponse {
	nonce: string;
	randomness: string;
	epoch: number;
	maxEpoch: number;
	estimatedExpiration: number;
}

export interface CreateZkLoginZkpApiInput {
	jwt: string;
	ephemeralPublicKey: PublicKey;
	randomness: string;
	maxEpoch: number;
}
export interface CreateZkLoginZkpApiResponse extends ZkLoginSignatureInputs {}

export interface CreateSponsoredTransactionBlockApiInput {
	network?: 'mainnet' | 'testnet';
	jwt: string;
	transactionBlockKindBytes: string;
}

export interface CreateSponsoredTransactionBlockApiResponse {
	bytes: string;
	digest: string;
}

export interface ExecuteSponsoredTransactionBlockApiInput {
	digest: string;
	signature: string;
}

export interface ExecuteSponsoredTransactionBlockApiResponse {
	digest: string;
}
