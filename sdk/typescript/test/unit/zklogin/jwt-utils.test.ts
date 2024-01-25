// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { describe, expect, test } from 'vitest';

import { extractClaimValue } from '../../../src/zklogin/jwt-utils';

describe('jwt-utils', () => {
	test('extracts claim value successfully', () => {
		expect(
			extractClaimValue(
				{ indexMod4: 1, value: 'yJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLC' },
				'iss',
			),
		).toBe('https://accounts.google.com');
	});
});
