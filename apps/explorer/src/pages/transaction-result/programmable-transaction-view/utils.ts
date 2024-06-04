// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { type SuiArgument } from '@dwallet-network/dwallet.js/client';

export function flattenSuiArguments(data: (SuiArgument | SuiArgument[])[]): string {
	if (!data) {
		return '';
	}

	return data
		.map((value) => {
			if (value === 'GasCoin') {
				return value;
			} else if (Array.isArray(value)) {
				return `[${flattenSuiArguments(value)}]`;
			} else if (value === null) {
				return 'Null';
			} else if (typeof value === 'object') {
				if ('Input' in value) {
					return `Input(${value.Input})`;
				} else if ('Result' in value) {
					return `Result(${value.Result})`;
				} else if ('NestedResult' in value) {
					return `NestedResult(${value.NestedResult[0]}, ${value.NestedResult[1]})`;
				}
			} else {
				throw new Error('Not a correct flattenable data');
			}
			return '';
		})
		.join(', ');
}
