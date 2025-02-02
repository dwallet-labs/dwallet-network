// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BigNumber from 'bignumber.js';
import { describe, expect, it } from 'vitest';

import { CoinFormat, formatBalance } from '../useFormatCoin';

const IKA_DECIMALS = 9;

function toNIka(ika: string) {
	return new BigNumber(ika).shiftedBy(IKA_DECIMALS).toString();
}

describe('formatBalance', () => {
	it('formats zero amounts correctly', () => {
		expect(formatBalance('0', 0)).toEqual('0');
		expect(formatBalance('0', IKA_DECIMALS)).toEqual('0');
	});

	it('formats decimal amounts correctly', () => {
		expect(formatBalance('0', IKA_DECIMALS)).toEqual('0');
		expect(formatBalance('0.000', IKA_DECIMALS)).toEqual('0');
	});

	it('formats integer amounts correctly', () => {
		expect(formatBalance(toNIka('1'), IKA_DECIMALS)).toEqual('1');
		expect(formatBalance(toNIka('1.0001'), IKA_DECIMALS)).toEqual('1');
		expect(formatBalance(toNIka('1.1201'), IKA_DECIMALS)).toEqual('1.12');
		expect(formatBalance(toNIka('1.1234'), IKA_DECIMALS)).toEqual('1.12');
		expect(formatBalance(toNIka('1.1239'), IKA_DECIMALS)).toEqual('1.12');

		expect(formatBalance(toNIka('9999.9999'), IKA_DECIMALS)).toEqual('9,999.99');
		// 10k + handling:
		expect(formatBalance(toNIka('10000'), IKA_DECIMALS)).toEqual('10 K');
		expect(formatBalance(toNIka('12345'), IKA_DECIMALS)).toEqual('12.34 K');
		// Millions:
		expect(formatBalance(toNIka('1234000'), IKA_DECIMALS)).toEqual('1.23 M');
		// Billions:
		expect(formatBalance(toNIka('1234000000'), IKA_DECIMALS)).toEqual('1.23 B');
	});

	it('formats integer amounts with full CoinFormat', () => {
		expect(formatBalance(toNIka('1'), IKA_DECIMALS, CoinFormat.FULL)).toEqual('1');
		expect(formatBalance(toNIka('1.123456789'), IKA_DECIMALS, CoinFormat.FULL)).toEqual(
			'1.123456789',
		);
		expect(formatBalance(toNIka('9999.9999'), IKA_DECIMALS, CoinFormat.FULL)).toEqual('9,999.9999');
		expect(formatBalance(toNIka('10000'), IKA_DECIMALS, CoinFormat.FULL)).toEqual('10,000');
		expect(formatBalance(toNIka('12345'), IKA_DECIMALS, CoinFormat.FULL)).toEqual('12,345');
		expect(formatBalance(toNIka('1234000'), IKA_DECIMALS, CoinFormat.FULL)).toEqual('1,234,000');
		expect(formatBalance(toNIka('1234000000'), IKA_DECIMALS, CoinFormat.FULL)).toEqual(
			'1,234,000,000',
		);
	});
});
