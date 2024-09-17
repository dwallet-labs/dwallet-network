// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { decodePeraPrivateKey } from '@pera-io/pera/cryptography/keypair';
import { z } from 'zod';

export const privateKeyValidation = z
	.string()
	.trim()
	.nonempty('Private Key is required.')
	.transform((privateKey, context) => {
		try {
			decodePeraPrivateKey(privateKey);
		} catch (error) {
			context.addIssue({
				code: 'custom',
				message:
					'Invalid Private Key, please use a Bech32 encoded 33-byte string. Learn more: https://github.com/pera-foundation/sips/blob/main/sips/sip-15.md',
			});
			return z.NEVER;
		}
		return privateKey;
	});
