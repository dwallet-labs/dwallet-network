// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DWalletClient } from '../client';
import type { Keypair } from '../cryptography';

export async function createDWallet(_keypair: Keypair, _client: DWalletClient) {
	return null;
}

export async function createPartialUserSignedMessages(
	_dwalletId: string,
	_dkgOutput: number[],
	_messages: Uint8Array[],
	_hash: 'KECCAK256' | 'SHA256',
	_keypair: Keypair,
	_client: DWalletClient,
) {
	return null;
}

export async function createVirginBoundDWallet(
	_encryptionKey: Uint8Array,
	_encryptionKeyObjId: string,
	_bindToAuthorityId: string,
	_keypair: Keypair,
	_client: DWalletClient,
// ): Promise<CreatedDwallet | null> {
){
	return null;
}