// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type Meta, type StoryObj } from '@storybook/react';

import { DateCard, type DateCardProps } from '../DateCard';

export default {
	component: DateCard,
} as Meta;

export const defaultAmount: StoryObj<DateCardProps> = {
	args: {
		date: 1667942429177,
	},
};
