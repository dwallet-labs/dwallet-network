// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type ExportedKeypair, type Keypair } from '@dwallet-network/dwallet.js/cryptography';
import { Ed25519Keypair } from '@dwallet-network/dwallet.js/keypairs/ed25519';
import { Secp256k1Keypair } from '@dwallet-network/dwallet.js/keypairs/secp256k1';
import { Secp256r1Keypair } from '@dwallet-network/dwallet.js/keypairs/secp256r1';
import { fromB64 } from '@dwallet-network/dwallet.js/utils';

const PRIVATE_KEY_SIZE = 32;
const LEGACY_PRIVATE_KEY_SIZE = 64;
export function fromExportedKeypair(keypair: ExportedKeypair): Keypair {
	const secretKey = fromB64(keypair.privateKey);

	switch (keypair.schema) {
		case 'ED25519':
			let pureSecretKey = secretKey;
			if (secretKey.length === LEGACY_PRIVATE_KEY_SIZE) {
				// This is a legacy secret key, we need to strip the public key bytes and only read the first 32 bytes
				pureSecretKey = secretKey.slice(0, PRIVATE_KEY_SIZE);
			}
			return Ed25519Keypair.fromSecretKey(pureSecretKey);
		case 'Secp256k1':
			return Secp256k1Keypair.fromSecretKey(secretKey);
		case 'Secp256r1':
			return Secp256r1Keypair.fromSecretKey(secretKey);
		default:
			throw new Error(`Invalid keypair schema ${keypair.schema}`);
	}
}
