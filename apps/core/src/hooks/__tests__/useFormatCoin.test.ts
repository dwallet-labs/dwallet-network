// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import BigNumber from 'bignumber.js';
import { describe, expect, it } from 'vitest';

import { CoinFormat, formatBalance } from '../useFormatCoin';

const PERA_DECIMALS = 9;

function toNPera(pera: string) {
	return new BigNumber(pera).shiftedBy(PERA_DECIMALS).toString();
}

describe('formatBalance', () => {
	it('formats zero amounts correctly', () => {
		expect(formatBalance('0', 0)).toEqual('0');
		expect(formatBalance('0', PERA_DECIMALS)).toEqual('0');
	});

	it('formats decimal amounts correctly', () => {
		expect(formatBalance('0', PERA_DECIMALS)).toEqual('0');
		expect(formatBalance('0.000', PERA_DECIMALS)).toEqual('0');
	});

	it('formats integer amounts correctly', () => {
		expect(formatBalance(toNPera('1'), PERA_DECIMALS)).toEqual('1');
		expect(formatBalance(toNPera('1.0001'), PERA_DECIMALS)).toEqual('1');
		expect(formatBalance(toNPera('1.1201'), PERA_DECIMALS)).toEqual('1.12');
		expect(formatBalance(toNPera('1.1234'), PERA_DECIMALS)).toEqual('1.12');
		expect(formatBalance(toNPera('1.1239'), PERA_DECIMALS)).toEqual('1.12');

		expect(formatBalance(toNPera('9999.9999'), PERA_DECIMALS)).toEqual('9,999.99');
		// 10k + handling:
		expect(formatBalance(toNPera('10000'), PERA_DECIMALS)).toEqual('10 K');
		expect(formatBalance(toNPera('12345'), PERA_DECIMALS)).toEqual('12.34 K');
		// Millions:
		expect(formatBalance(toNPera('1234000'), PERA_DECIMALS)).toEqual('1.23 M');
		// Billions:
		expect(formatBalance(toNPera('1234000000'), PERA_DECIMALS)).toEqual('1.23 B');
	});

	it('formats integer amounts with full CoinFormat', () => {
		expect(formatBalance(toNPera('1'), PERA_DECIMALS, CoinFormat.FULL)).toEqual('1');
		expect(formatBalance(toNPera('1.123456789'), PERA_DECIMALS, CoinFormat.FULL)).toEqual(
			'1.123456789',
		);
		expect(formatBalance(toNPera('9999.9999'), PERA_DECIMALS, CoinFormat.FULL)).toEqual('9,999.9999');
		expect(formatBalance(toNPera('10000'), PERA_DECIMALS, CoinFormat.FULL)).toEqual('10,000');
		expect(formatBalance(toNPera('12345'), PERA_DECIMALS, CoinFormat.FULL)).toEqual('12,345');
		expect(formatBalance(toNPera('1234000'), PERA_DECIMALS, CoinFormat.FULL)).toEqual('1,234,000');
		expect(formatBalance(toNPera('1234000000'), PERA_DECIMALS, CoinFormat.FULL)).toEqual(
			'1,234,000,000',
		);
	});
});
