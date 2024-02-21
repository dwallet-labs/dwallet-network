// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useMemo } from 'react';

import { getNormalizedFunctionParameterTypeDetails } from '../utils';

import type { SuiMoveNormalizedType } from '@mysten/sui.js/client';

export function useFunctionParamsDetails(
	params: SuiMoveNormalizedType[],
	functionTypeArgNames?: string[],
) {
	return useMemo(
		() =>
			params
				.map((aParam) => getNormalizedFunctionParameterTypeDetails(aParam, functionTypeArgNames))
				.filter(({ isTxContext }) => !isTxContext),
		[params, functionTypeArgNames],
	);
}
