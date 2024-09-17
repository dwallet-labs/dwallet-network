// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { createPeraAddressValidation } from '_components/address-input/validation';
import { type PeraClient } from '@pera-io/pera/client';
import * as Yup from 'yup';

export function createValidationSchema(
	client: PeraClient,
	peraNSEnabled: boolean,
	senderAddress: string,
	objectId: string,
) {
	return Yup.object({
		to: createPeraAddressValidation(client, peraNSEnabled)
			.test(
				'sender-address',
				// eslint-disable-next-line no-template-curly-in-string
				`NFT is owned by this address`,
				(value) => senderAddress !== value,
			)
			.test(
				'nft-sender-address',
				// eslint-disable-next-line no-template-curly-in-string
				`NFT address must be different from receiver address`,
				(value) => objectId !== value,
			),
	});
}
