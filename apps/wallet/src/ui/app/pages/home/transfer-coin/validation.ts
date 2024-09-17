// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { createPeraAddressValidation } from '_components/address-input/validation';
import { createTokenValidation } from '_src/shared/validation';
import { type PeraClient } from '@pera-io/pera/client';
import * as Yup from 'yup';

export function createValidationSchemaStepOne(
	client: PeraClient,
	peraNSEnabled: boolean,
	...args: Parameters<typeof createTokenValidation>
) {
	return Yup.object({
		to: createPeraAddressValidation(client, peraNSEnabled),
		amount: createTokenValidation(...args),
	});
}
