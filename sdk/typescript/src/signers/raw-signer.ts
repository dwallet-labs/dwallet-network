// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { blake2b } from '@noble/hashes/blake2b';

import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/keypair.js';
import { toSerializedSignature } from '../cryptography/signature.js';
import type { SerializedSignature } from '../cryptography/signature.js';
import { SignerWithProvider } from './signer-with-provider.js';

export class RawSigner extends SignerWithProvider {
	private readonly keypair: Keypair;

	constructor(keypair: Keypair, client: DWalletClient) {
		super(client);
		this.keypair = keypair;
	}

	async getAddress(): Promise<string> {
		return this.keypair.getPublicKey().toSuiAddress();
	}

	async signData(data: Uint8Array): Promise<SerializedSignature> {
		const pubkey = this.keypair.getPublicKey();
		const digest = blake2b(data, { dkLen: 32 });
		const signature = this.keypair.signData(digest);
		const signatureScheme = this.keypair.getKeyScheme();

		return toSerializedSignature({
			signatureScheme,
			signature,
			publicKey: pubkey,
		});
	}

	connect(client: DWalletClient): SignerWithProvider {
		return new RawSigner(this.keypair, client);
	}
}
