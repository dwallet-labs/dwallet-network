// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { createIkaAddressValidation } from '_components/address-input/validation';
import { createTokenValidation } from '_src/shared/validation';
import { type IkaClient } from '@ika-io/ika/client';
import * as Yup from 'yup';

export function createValidationSchemaStepOne(
	client: IkaClient,
	ikaNSEnabled: boolean,
	...args: Parameters<typeof createTokenValidation>
) {
	return Yup.object({
		to: createIkaAddressValidation(client, ikaNSEnabled),
		amount: createTokenValidation(...args),
	});
}
