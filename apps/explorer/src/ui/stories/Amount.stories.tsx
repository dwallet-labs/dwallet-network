// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type Meta, type StoryObj } from '@storybook/react';

import { Amount, type AmountProps } from '../Amount';

export default {
	component: Amount,
} as Meta;

export const Default: StoryObj<AmountProps> = {
	args: {
		amount: 1000,
		symbol: 'DWLT',
	},
};

export const LargeSize: StoryObj<AmountProps> = {
	args: {
		amount: 990000,
		symbol: 'USDC',
		size: 'lg',
	},
};

export const TestCoin: StoryObj<AmountProps> = {
	args: {
		amount: 10000,
		symbol: 'USDC',
		size: 'lg',
	},
};

export const WithoutSymbol: StoryObj<AmountProps> = {
	args: {
		amount: 990000,
	},
};
