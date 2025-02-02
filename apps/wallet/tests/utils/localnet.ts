// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import 'tsconfig-paths/register';

import { requestIkaFromFaucetV0 } from '@ika-io/ika/faucet';
import { Ed25519Keypair } from '@ika-io/ika/keypairs/ed25519';
import * as bip39 from '@scure/bip39';
import { wordlist } from '@scure/bip39/wordlists/english';

export async function generateKeypairFromMnemonic(mnemonic: string) {
	return Ed25519Keypair.deriveKeypair(mnemonic);
}

export async function generateKeypair() {
	const mnemonic = bip39.generateMnemonic(wordlist);
	const keypair = await generateKeypairFromMnemonic(mnemonic);
	return { mnemonic, keypair };
}

const FAUCET_HOST = 'http://127.0.0.1:9123';

export async function requestIkaFromFaucet(recipient: string) {
	await requestIkaFromFaucetV0({ host: FAUCET_HOST, recipient });
}
