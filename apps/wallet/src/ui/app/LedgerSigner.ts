// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type IkaLedgerClient from '@mysten/ledgerjs-hw-app-ika';
import { type IkaClient } from '@ika-io/ika/client';
import { toSerializedSignature, type SignatureScheme } from '@ika-io/ika/cryptography';
import { Ed25519PublicKey } from '@ika-io/ika/keypairs/ed25519';

import { WalletSigner } from './WalletSigner';

export class LedgerSigner extends WalletSigner {
	#ikaLedgerClient: IkaLedgerClient | null;
	readonly #connectToLedger: () => Promise<IkaLedgerClient>;
	readonly #derivationPath: string;
	readonly #signatureScheme: SignatureScheme = 'ED25519';

	constructor(
		connectToLedger: () => Promise<IkaLedgerClient>,
		derivationPath: string,
		client: IkaClient,
	) {
		super(client);
		this.#connectToLedger = connectToLedger;
		this.#ikaLedgerClient = null;
		this.#derivationPath = derivationPath;
	}

	async #initializeIkaLedgerClient() {
		if (!this.#ikaLedgerClient) {
			// We want to make sure that there's only one connection established per Ledger signer
			// instance since some methods make multiple calls like getAddress and signData
			this.#ikaLedgerClient = await this.#connectToLedger();
		}
		return this.#ikaLedgerClient;
	}

	async getAddress(): Promise<string> {
		const ledgerClient = await this.#initializeIkaLedgerClient();
		const publicKeyResult = await ledgerClient.getPublicKey(this.#derivationPath);
		const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
		return publicKey.toIkaAddress();
	}

	async getPublicKey(): Promise<Ed25519PublicKey> {
		const ledgerClient = await this.#initializeIkaLedgerClient();
		const { publicKey } = await ledgerClient.getPublicKey(this.#derivationPath);
		return new Ed25519PublicKey(publicKey);
	}

	async signData(data: Uint8Array): Promise<string> {
		const ledgerClient = await this.#initializeIkaLedgerClient();
		const { signature } = await ledgerClient.signTransaction(this.#derivationPath, data);
		const publicKey = await this.getPublicKey();
		return toSerializedSignature({
			signature,
			signatureScheme: this.#signatureScheme,
			publicKey,
		});
	}

	connect(client: IkaClient) {
		return new LedgerSigner(this.#connectToLedger, this.#derivationPath, client);
	}
}
