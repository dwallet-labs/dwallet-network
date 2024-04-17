// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type SuiEvent } from '@dwallet/dwallet.js/client';

export function getValidatorMoveEvent(validatorsEvent: SuiEvent[], validatorAddress: string) {
	const event = validatorsEvent.find(
		({ parsedJson }) =>
			(parsedJson as { validator_address?: unknown })!.validator_address === validatorAddress,
	);

	return event && event.parsedJson;
}
