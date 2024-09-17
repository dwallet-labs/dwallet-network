// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type SerializedUIAccount } from '_src/background/accounts/Account';
import { type PeraClient } from '@pera-io/pera/client';

import type { BackgroundClient } from '.';
import { WalletSigner } from '../WalletSigner';

export class BackgroundServiceSigner extends WalletSigner {
	readonly #account: SerializedUIAccount;
	readonly #backgroundClient: BackgroundClient;

	constructor(account: SerializedUIAccount, backgroundClient: BackgroundClient, client: PeraClient) {
		super(client);
		this.#account = account;
		this.#backgroundClient = backgroundClient;
	}

	async getAddress(): Promise<string> {
		return this.#account.address;
	}

	signData(data: Uint8Array): Promise<string> {
		return this.#backgroundClient.signData(this.#account.id, data);
	}

	connect(client: PeraClient) {
		return new BackgroundServiceSigner(this.#account, this.#backgroundClient, client);
	}
}
