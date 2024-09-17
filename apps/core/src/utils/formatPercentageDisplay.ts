// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// For unavailable %, return '--' else return the APY number
export function formatPercentageDisplay(
	value: number | null,
	nullDisplay = '--',
	isApyApprox = false,
) {
	return value === null ? nullDisplay : `${isApyApprox ? '~' : ''}${value}%`;
}
