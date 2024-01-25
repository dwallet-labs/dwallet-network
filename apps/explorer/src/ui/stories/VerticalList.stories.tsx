// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type Meta, type StoryObj } from '@storybook/react';

import { ListItem, VerticalList } from '../VerticalList';

export default {
	component: VerticalList,
} as Meta;

export const Default: StoryObj = {
	render: () => (
		<VerticalList>
			<ListItem>One</ListItem>
			<ListItem active>Two</ListItem>
			<ListItem>Three</ListItem>
			<ListItem>Four</ListItem>
		</VerticalList>
	),
};
