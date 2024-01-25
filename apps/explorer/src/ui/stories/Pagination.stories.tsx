// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type StoryObj, type Meta } from '@storybook/react';

import { Pagination } from '../Pagination';

export default {
	component: Pagination,
} as Meta;

export const Default: StoryObj<typeof Pagination> = {
	args: {
		hasPrev: true,
		hasNext: true,
		onNext() {},
		onPrev() {},
		onFirst() {},
	},
};
