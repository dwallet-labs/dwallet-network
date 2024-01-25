// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type Meta, type StoryObj } from '@storybook/react';

import { Toggle } from './Toggle';

const meta = {
	component: Toggle,
} satisfies Meta<typeof Toggle>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Checked: Story = {
	args: {
		checked: true,
	},
};
