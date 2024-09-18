// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { describe, expect, test } from 'vitest';

import { isValidPeraNSName, normalizePeraNSName } from '../../../src/utils';

describe('isValidPeraNSName', () => {
	test('valid PeraNS names', () => {
		expect(isValidPeraNSName('example.pera')).toBe(true);
		expect(isValidPeraNSName('EXAMPLE.pera')).toBe(true);
		expect(isValidPeraNSName('@example')).toBe(true);
		expect(isValidPeraNSName('1.example.pera')).toBe(true);
		expect(isValidPeraNSName('1@example')).toBe(true);
		expect(isValidPeraNSName('a.b.c.example.pera')).toBe(true);
		expect(isValidPeraNSName('A.B.c.123@Example')).toBe(true);
		expect(isValidPeraNSName('1-a@1-b')).toBe(true);
		expect(isValidPeraNSName('1-a.1-b.pera')).toBe(true);
		expect(isValidPeraNSName('-@test')).toBe(false);
		expect(isValidPeraNSName('-1@test')).toBe(false);
		expect(isValidPeraNSName('test@-')).toBe(false);
		expect(isValidPeraNSName('test@-1')).toBe(false);
		expect(isValidPeraNSName('test@-a')).toBe(false);
		expect(isValidPeraNSName('test.pera2')).toBe(false);
		expect(isValidPeraNSName('.pera2')).toBe(false);
		expect(isValidPeraNSName('test@')).toBe(false);
		expect(isValidPeraNSName('@@')).toBe(false);
		expect(isValidPeraNSName('@@test')).toBe(false);
		expect(isValidPeraNSName('test@test.test')).toBe(false);
		expect(isValidPeraNSName('@test.test')).toBe(false);
		expect(isValidPeraNSName('#@test')).toBe(false);
		expect(isValidPeraNSName('test@#')).toBe(false);
		expect(isValidPeraNSName('test.#.pera')).toBe(false);
		expect(isValidPeraNSName('#.pera')).toBe(false);
		expect(isValidPeraNSName('@.test.sue')).toBe(false);

		expect(isValidPeraNSName('hello-.pera')).toBe(false);
		expect(isValidPeraNSName('hello--.pera')).toBe(false);
		expect(isValidPeraNSName('hello.-pera')).toBe(false);
		expect(isValidPeraNSName('hello.--pera')).toBe(false);
		expect(isValidPeraNSName('hello.pera-')).toBe(false);
		expect(isValidPeraNSName('hello.pera--')).toBe(false);
		expect(isValidPeraNSName('hello-@pera')).toBe(false);
		expect(isValidPeraNSName('hello--@pera')).toBe(false);
		expect(isValidPeraNSName('hello@-pera')).toBe(false);
		expect(isValidPeraNSName('hello@--pera')).toBe(false);
		expect(isValidPeraNSName('hello@pera-')).toBe(false);
		expect(isValidPeraNSName('hello@pera--')).toBe(false);
		expect(isValidPeraNSName('hello--world@pera')).toBe(false);
	});
});

describe('normalizePeraNSName', () => {
	test('normalize PeraNS names', () => {
		expect(normalizePeraNSName('example.pera')).toMatch('@example');
		expect(normalizePeraNSName('EXAMPLE.pera')).toMatch('@example');
		expect(normalizePeraNSName('@example')).toMatch('@example');
		expect(normalizePeraNSName('1.example.pera')).toMatch('1@example');
		expect(normalizePeraNSName('1@example')).toMatch('1@example');
		expect(normalizePeraNSName('a.b.c.example.pera')).toMatch('a.b.c@example');
		expect(normalizePeraNSName('A.B.c.123@Example')).toMatch('a.b.c.123@example');
		expect(normalizePeraNSName('1-a@1-b')).toMatch('1-a@1-b');
		expect(normalizePeraNSName('1-a.1-b.pera')).toMatch('1-a@1-b');

		expect(normalizePeraNSName('example.pera', 'dot')).toMatch('example.pera');
		expect(normalizePeraNSName('EXAMPLE.pera', 'dot')).toMatch('example.pera');
		expect(normalizePeraNSName('@example', 'dot')).toMatch('example.pera');
		expect(normalizePeraNSName('1.example.pera', 'dot')).toMatch('1.example.pera');
		expect(normalizePeraNSName('1@example', 'dot')).toMatch('1.example.pera');
		expect(normalizePeraNSName('a.b.c.example.pera', 'dot')).toMatch('a.b.c.example.pera');
		expect(normalizePeraNSName('A.B.c.123@Example', 'dot')).toMatch('a.b.c.123.example.pera');
		expect(normalizePeraNSName('1-a@1-b', 'dot')).toMatch('1-a.1-b.pera');
		expect(normalizePeraNSName('1-a.1-b.pera', 'dot')).toMatch('1-a.1-b.pera');

		expect(() => normalizePeraNSName('-@test')).toThrowError('Invalid PeraNS name -@test');
		expect(normalizePeraNSName('1-a@1-b')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(normalizePeraNSName('1-a.1-b.pera')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(() => normalizePeraNSName('-@test')).toThrowError('Invalid PeraNS name -@test');
		expect(() => normalizePeraNSName('-1@test')).toThrowError('Invalid PeraNS name -1@test');
		expect(() => normalizePeraNSName('test@-')).toThrowError('Invalid PeraNS name test@-');
		expect(() => normalizePeraNSName('test@-1')).toThrowError('Invalid PeraNS name test@-1');
		expect(() => normalizePeraNSName('test@-a')).toThrowError('Invalid PeraNS name test@-a');
		expect(() => normalizePeraNSName('test.pera2')).toThrowError('Invalid PeraNS name test.pera2');
		expect(() => normalizePeraNSName('.pera2')).toThrowError('Invalid PeraNS name .pera2');
		expect(() => normalizePeraNSName('test@')).toThrowError('Invalid PeraNS name test@');
		expect(() => normalizePeraNSName('@@')).toThrowError('Invalid PeraNS name @@');
		expect(() => normalizePeraNSName('@@test')).toThrowError('Invalid PeraNS name @@test');
		expect(() => normalizePeraNSName('test@test.test')).toThrowError(
			'Invalid PeraNS name test@test.test',
		);
		expect(() => normalizePeraNSName('@test.test')).toThrowError('Invalid PeraNS name @test.test');
		expect(() => normalizePeraNSName('#@test')).toThrowError('Invalid PeraNS name #@test');
		expect(() => normalizePeraNSName('test@#')).toThrowError('Invalid PeraNS name test@#');
		expect(() => normalizePeraNSName('test.#.pera')).toThrowError(
			'Invalid PeraNS name test.#.pera',
		);
		expect(() => normalizePeraNSName('#.pera')).toThrowError('Invalid PeraNS name #.pera');
		expect(() => normalizePeraNSName('@.test.sue')).toThrowError('Invalid PeraNS name @.test.sue');
	});
});
