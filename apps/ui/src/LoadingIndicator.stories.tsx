// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { Meta, StoryObj } from '@storybook/react';

import { LoadingIndicator } from './LoadingIndicator';

const meta = {
	component: LoadingIndicator,
} satisfies Meta<typeof LoadingIndicator>;

export default meta;

type Story = StoryObj<typeof meta>;

export const Default: Story = {};

export const Text: Story = {
	args: {
		text: 'Loading...',
	},
};
