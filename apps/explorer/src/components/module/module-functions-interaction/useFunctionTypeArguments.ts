// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useMemo } from 'react';

import type { SuiMoveAbilitySet } from '@dwallet/dwallet.js/client';

export function useFunctionTypeArguments(typeArguments: SuiMoveAbilitySet[]) {
	return useMemo(
		() =>
			typeArguments.map(
				(aTypeArgument, index) =>
					`T${index}${
						aTypeArgument.abilities.length ? `: ${aTypeArgument.abilities.join(' + ')}` : ''
					}`,
			),
		[typeArguments],
	);
}
