// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

const DEFAULT_PRECISION = 2;
export function roundFloat(num: number, precision = DEFAULT_PRECISION) {
	return parseFloat(num.toFixed(precision));
}
