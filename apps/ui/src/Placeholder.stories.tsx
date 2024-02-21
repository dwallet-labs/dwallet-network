// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type Meta, type StoryObj } from '@storybook/react';

import { Placeholder } from './Placeholder';

const meta = {
	component: Placeholder,
} satisfies Meta<typeof Placeholder>;

export default meta;

type Story = StoryObj<typeof meta>;

export const VaryingWidthAndHeight: Story = {
	render: () => (
		<div>
			<Placeholder width="120px" height="12px" />
			<br />
			<Placeholder width="90px" height="16px" />
			<br />
			<Placeholder width="59px" height="32px" />
		</div>
	),
};
