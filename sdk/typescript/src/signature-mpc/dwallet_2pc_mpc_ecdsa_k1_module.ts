// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DWalletClient } from '../client/index.js';
import type { Keypair } from '../cryptography/index.js';

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
