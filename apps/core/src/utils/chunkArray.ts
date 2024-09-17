// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export function chunkArray<T>(arr: T[], chunkSize: number) {
	return Array.from({ length: Math.ceil(arr.length / chunkSize) }, (_, i) =>
		arr.slice(i * chunkSize, (i + 1) * chunkSize),
	);
}
