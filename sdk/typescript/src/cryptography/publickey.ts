// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { toB64 } from '@mysten/bcs';
import { blake2b } from '@noble/hashes/blake2b';
import { bytesToHex } from '@noble/hashes/utils';

import { bcs } from '../bcs/index.js';
import { normalizePeraAddress, PERA_ADDRESS_LENGTH } from '../utils/pera-types.js';
import type { IntentScope } from './intent.js';
import { messageWithIntent } from './intent.js';

/**
 * Value to be converted into public key.
 */
export type PublicKeyInitData = string | Uint8Array | Iterable<number>;

export function bytesEqual(a: Uint8Array, b: Uint8Array) {
	if (a === b) return true;

	if (a.length !== b.length) {
		return false;
	}

	for (let i = 0; i < a.length; i++) {
		if (a[i] !== b[i]) {
			return false;
		}
	}
	return true;
}

/**
 * A public key
 */
export abstract class PublicKey {
	/**
	 * Checks if two public keys are equal
	 */
	equals(publicKey: PublicKey) {
		return bytesEqual(this.toRawBytes(), publicKey.toRawBytes());
	}

	/**
	 * Return the base-64 representation of the public key
	 */
	toBase64() {
		return toB64(this.toRawBytes());
	}

	toString(): never {
		throw new Error(
			'`toString` is not implemented on public keys. Use `toBase64()` or `toRawBytes()` instead.',
		);
	}

	/**
	 * Return the Pera representation of the public key encoded in
	 * base-64. A Pera public key is formed by the concatenation
	 * of the scheme flag with the raw bytes of the public key
	 */
	toPeraPublicKey(): string {
		const bytes = this.toPeraBytes();
		return toB64(bytes);
	}

	verifyWithIntent(
		bytes: Uint8Array,
		signature: Uint8Array | string,
		intent: IntentScope,
	): Promise<boolean> {
		const intentMessage = messageWithIntent(intent, bytes);
		const digest = blake2b(intentMessage, { dkLen: 32 });

		return this.verify(digest, signature);
	}

	/**
	 * Verifies that the signature is valid for for the provided PersonalMessage
	 */
	verifyPersonalMessage(message: Uint8Array, signature: Uint8Array | string): Promise<boolean> {
		return this.verifyWithIntent(
			bcs.vector(bcs.u8()).serialize(message).toBytes(),
			signature,
			'PersonalMessage',
		);
	}

	/**
	 * Verifies that the signature is valid for for the provided Transaction
	 */
	verifyTransaction(transaction: Uint8Array, signature: Uint8Array | string): Promise<boolean> {
		return this.verifyWithIntent(transaction, signature, 'TransactionData');
	}

	/**
	 * Returns the bytes representation of the public key
	 * prefixed with the signature scheme flag
	 */
	toPeraBytes(): Uint8Array {
		const rawBytes = this.toRawBytes();
		const peraBytes = new Uint8Array(rawBytes.length + 1);
		peraBytes.set([this.flag()]);
		peraBytes.set(rawBytes, 1);

		return peraBytes;
	}

	/**
	 * Return the Pera address associated with this Ed25519 public key
	 */
	toPeraAddress(): string {
		// Each hex char represents half a byte, hence hex address doubles the length
		return normalizePeraAddress(
			bytesToHex(blake2b(this.toPeraBytes(), { dkLen: 32 })).slice(0, PERA_ADDRESS_LENGTH * 2),
		);
	}

	/**
	 * Return the byte array representation of the public key
	 */
	abstract toRawBytes(): Uint8Array;

	/**
	 * Return signature scheme flag of the public key
	 */
	abstract flag(): number;

	/**
	 * Verifies that the signature is valid for for the provided message
	 */
	abstract verify(data: Uint8Array, signature: Uint8Array | string): Promise<boolean>;
}
