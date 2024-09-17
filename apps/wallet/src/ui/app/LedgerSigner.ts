// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type PeraLedgerClient from '@mysten/ledgerjs-hw-app-pera';
import { type PeraClient } from '@pera-io/pera/client';
import { toSerializedSignature, type SignatureScheme } from '@pera-io/pera/cryptography';
import { Ed25519PublicKey } from '@pera-io/pera/keypairs/ed25519';

import { WalletSigner } from './WalletSigner';

export class LedgerSigner extends WalletSigner {
	#peraLedgerClient: PeraLedgerClient | null;
	readonly #connectToLedger: () => Promise<PeraLedgerClient>;
	readonly #derivationPath: string;
	readonly #signatureScheme: SignatureScheme = 'ED25519';

	constructor(
		connectToLedger: () => Promise<PeraLedgerClient>,
		derivationPath: string,
		client: PeraClient,
	) {
		super(client);
		this.#connectToLedger = connectToLedger;
		this.#peraLedgerClient = null;
		this.#derivationPath = derivationPath;
	}

	async #initializePeraLedgerClient() {
		if (!this.#peraLedgerClient) {
			// We want to make sure that there's only one connection established per Ledger signer
			// instance since some methods make multiple calls like getAddress and signData
			this.#peraLedgerClient = await this.#connectToLedger();
		}
		return this.#peraLedgerClient;
	}

	async getAddress(): Promise<string> {
		const ledgerClient = await this.#initializePeraLedgerClient();
		const publicKeyResult = await ledgerClient.getPublicKey(this.#derivationPath);
		const publicKey = new Ed25519PublicKey(publicKeyResult.publicKey);
		return publicKey.toPeraAddress();
	}

	async getPublicKey(): Promise<Ed25519PublicKey> {
		const ledgerClient = await this.#initializePeraLedgerClient();
		const { publicKey } = await ledgerClient.getPublicKey(this.#derivationPath);
		return new Ed25519PublicKey(publicKey);
	}

	async signData(data: Uint8Array): Promise<string> {
		const ledgerClient = await this.#initializePeraLedgerClient();
		const { signature } = await ledgerClient.signTransaction(this.#derivationPath, data);
		const publicKey = await this.getPublicKey();
		return toSerializedSignature({
			signature,
			signatureScheme: this.#signatureScheme,
			publicKey,
		});
	}

	connect(client: PeraClient) {
		return new LedgerSigner(this.#connectToLedger, this.#derivationPath, client);
	}
}
