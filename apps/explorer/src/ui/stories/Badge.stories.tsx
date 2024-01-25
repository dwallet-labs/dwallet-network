// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type Meta, type StoryObj } from '@storybook/react';

import { Badge, type BadgeProps } from '../Badge';

export default {
	component: Badge,
} as Meta;

export const Current: StoryObj<BadgeProps> = {
	render: (props) => <Badge {...props}>Badge</Badge>,
};

export const Success: StoryObj<BadgeProps> = {
	...Current,
	args: { variant: 'success' },
};

export const Failure: StoryObj<BadgeProps> = {
	...Current,
	args: { variant: 'failure' },
};
