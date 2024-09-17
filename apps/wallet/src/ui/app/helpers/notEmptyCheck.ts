// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export default function notEmpty<TValue>(value: TValue | null | undefined): value is TValue {
	if (value === null || value === undefined) return false;
	return true;
}
