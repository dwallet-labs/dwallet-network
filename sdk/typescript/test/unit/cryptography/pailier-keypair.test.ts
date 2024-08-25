// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { describe, expect, it } from 'vitest';

import { Ed25519Keypair } from '../../../src/keypairs/ed25519';
import { generatePaillierKeyPairFromSuiKeyPair } from '../../../src/signature-mpc/utils';

describe('Derive a paillier keypair from a Sui keypair', () => {
	it('should derive a paillier keypair from a Sui keypair', () => {
		let keypair = Ed25519Keypair.generate();
		const [encryptionKey, decryptionKey] = generatePaillierKeyPairFromSuiKeyPair(keypair);
		console.log({ encryptionKey, decryptionKey });
	});

	it('should generate the same Paillier key pair when using the same Sui keypair twice', () => {
		let keypair = Ed25519Keypair.generate();
		const [firstEncryptionKey, firstDecryptionKey] = generatePaillierKeyPairFromSuiKeyPair(keypair);
		const [secondEncryptionKey, secondDecryptionKey] =
			generatePaillierKeyPairFromSuiKeyPair(keypair);
		expect(firstEncryptionKey).toEqual(secondEncryptionKey);
		expect(firstDecryptionKey).toEqual(secondDecryptionKey);
	});

	it('should generate different Paillier key pairs when using the different Sui keypair', () => {
		let keypair1 = Ed25519Keypair.generate();
		let keypair2 = Ed25519Keypair.generate();
		const [firstEncryptionKey, firstDecryptionKey] =
			generatePaillierKeyPairFromSuiKeyPair(keypair1);
		const [secondEncryptionKey, secondDecryptionKey] =
			generatePaillierKeyPairFromSuiKeyPair(keypair2);
		expect(firstEncryptionKey).not.toEqual(secondEncryptionKey);
		expect(firstDecryptionKey).not.toEqual(secondDecryptionKey);
	});
});
