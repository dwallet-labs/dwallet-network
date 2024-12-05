// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, expect, test } from 'vitest';

import { isValidIkaNSName, normalizeIkaNSName } from '../../../src/utils';

describe('isValidIkaNSName', () => {
	test('valid IkaNS names', () => {
		expect(isValidIkaNSName('example.ika')).toBe(true);
		expect(isValidIkaNSName('EXAMPLE.ika')).toBe(true);
		expect(isValidIkaNSName('@example')).toBe(true);
		expect(isValidIkaNSName('1.example.ika')).toBe(true);
		expect(isValidIkaNSName('1@example')).toBe(true);
		expect(isValidIkaNSName('a.b.c.example.ika')).toBe(true);
		expect(isValidIkaNSName('A.B.c.123@Example')).toBe(true);
		expect(isValidIkaNSName('1-a@1-b')).toBe(true);
		expect(isValidIkaNSName('1-a.1-b.ika')).toBe(true);
		expect(isValidIkaNSName('-@test')).toBe(false);
		expect(isValidIkaNSName('-1@test')).toBe(false);
		expect(isValidIkaNSName('test@-')).toBe(false);
		expect(isValidIkaNSName('test@-1')).toBe(false);
		expect(isValidIkaNSName('test@-a')).toBe(false);
		expect(isValidIkaNSName('test.ika2')).toBe(false);
		expect(isValidIkaNSName('.ika2')).toBe(false);
		expect(isValidIkaNSName('test@')).toBe(false);
		expect(isValidIkaNSName('@@')).toBe(false);
		expect(isValidIkaNSName('@@test')).toBe(false);
		expect(isValidIkaNSName('test@test.test')).toBe(false);
		expect(isValidIkaNSName('@test.test')).toBe(false);
		expect(isValidIkaNSName('#@test')).toBe(false);
		expect(isValidIkaNSName('test@#')).toBe(false);
		expect(isValidIkaNSName('test.#.ika')).toBe(false);
		expect(isValidIkaNSName('#.ika')).toBe(false);
		expect(isValidIkaNSName('@.test.sue')).toBe(false);

		expect(isValidIkaNSName('hello-.ika')).toBe(false);
		expect(isValidIkaNSName('hello--.ika')).toBe(false);
		expect(isValidIkaNSName('hello.-ika')).toBe(false);
		expect(isValidIkaNSName('hello.--ika')).toBe(false);
		expect(isValidIkaNSName('hello.ika-')).toBe(false);
		expect(isValidIkaNSName('hello.ika--')).toBe(false);
		expect(isValidIkaNSName('hello-@ika')).toBe(false);
		expect(isValidIkaNSName('hello--@ika')).toBe(false);
		expect(isValidIkaNSName('hello@-ika')).toBe(false);
		expect(isValidIkaNSName('hello@--ika')).toBe(false);
		expect(isValidIkaNSName('hello@ika-')).toBe(false);
		expect(isValidIkaNSName('hello@ika--')).toBe(false);
		expect(isValidIkaNSName('hello--world@ika')).toBe(false);
	});
});

describe('normalizeIkaNSName', () => {
	test('normalize IkaNS names', () => {
		expect(normalizeIkaNSName('example.ika')).toMatch('@example');
		expect(normalizeIkaNSName('EXAMPLE.ika')).toMatch('@example');
		expect(normalizeIkaNSName('@example')).toMatch('@example');
		expect(normalizeIkaNSName('1.example.ika')).toMatch('1@example');
		expect(normalizeIkaNSName('1@example')).toMatch('1@example');
		expect(normalizeIkaNSName('a.b.c.example.ika')).toMatch('a.b.c@example');
		expect(normalizeIkaNSName('A.B.c.123@Example')).toMatch('a.b.c.123@example');
		expect(normalizeIkaNSName('1-a@1-b')).toMatch('1-a@1-b');
		expect(normalizeIkaNSName('1-a.1-b.ika')).toMatch('1-a@1-b');

		expect(normalizeIkaNSName('example.ika', 'dot')).toMatch('example.ika');
		expect(normalizeIkaNSName('EXAMPLE.ika', 'dot')).toMatch('example.ika');
		expect(normalizeIkaNSName('@example', 'dot')).toMatch('example.ika');
		expect(normalizeIkaNSName('1.example.ika', 'dot')).toMatch('1.example.ika');
		expect(normalizeIkaNSName('1@example', 'dot')).toMatch('1.example.ika');
		expect(normalizeIkaNSName('a.b.c.example.ika', 'dot')).toMatch('a.b.c.example.ika');
		expect(normalizeIkaNSName('A.B.c.123@Example', 'dot')).toMatch('a.b.c.123.example.ika');
		expect(normalizeIkaNSName('1-a@1-b', 'dot')).toMatch('1-a.1-b.ika');
		expect(normalizeIkaNSName('1-a.1-b.ika', 'dot')).toMatch('1-a.1-b.ika');

		expect(() => normalizeIkaNSName('-@test')).toThrowError('Invalid IkaNS name -@test');
		expect(normalizeIkaNSName('1-a@1-b')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(normalizeIkaNSName('1-a.1-b.ika')).toMatchInlineSnapshot('"1-a@1-b"');
		expect(() => normalizeIkaNSName('-@test')).toThrowError('Invalid IkaNS name -@test');
		expect(() => normalizeIkaNSName('-1@test')).toThrowError('Invalid IkaNS name -1@test');
		expect(() => normalizeIkaNSName('test@-')).toThrowError('Invalid IkaNS name test@-');
		expect(() => normalizeIkaNSName('test@-1')).toThrowError('Invalid IkaNS name test@-1');
		expect(() => normalizeIkaNSName('test@-a')).toThrowError('Invalid IkaNS name test@-a');
		expect(() => normalizeIkaNSName('test.ika2')).toThrowError('Invalid IkaNS name test.ika2');
		expect(() => normalizeIkaNSName('.ika2')).toThrowError('Invalid IkaNS name .ika2');
		expect(() => normalizeIkaNSName('test@')).toThrowError('Invalid IkaNS name test@');
		expect(() => normalizeIkaNSName('@@')).toThrowError('Invalid IkaNS name @@');
		expect(() => normalizeIkaNSName('@@test')).toThrowError('Invalid IkaNS name @@test');
		expect(() => normalizeIkaNSName('test@test.test')).toThrowError(
			'Invalid IkaNS name test@test.test',
		);
		expect(() => normalizeIkaNSName('@test.test')).toThrowError('Invalid IkaNS name @test.test');
		expect(() => normalizeIkaNSName('#@test')).toThrowError('Invalid IkaNS name #@test');
		expect(() => normalizeIkaNSName('test@#')).toThrowError('Invalid IkaNS name test@#');
		expect(() => normalizeIkaNSName('test.#.ika')).toThrowError('Invalid IkaNS name test.#.ika');
		expect(() => normalizeIkaNSName('#.ika')).toThrowError('Invalid IkaNS name #.ika');
		expect(() => normalizeIkaNSName('@.test.sue')).toThrowError('Invalid IkaNS name @.test.sue');
	});
});
