// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export function toShortTypeString<T extends string | null | undefined>(type?: T): T {
	return type?.replace(/0x0{31,}(\d)/g, '0x$1').replace(/,\b/g, ', ') as T;
}

export function isNumericString(value: string) {
	return /^-?\d+$/.test(value);
}
