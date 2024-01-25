// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ImageIcon, type ImageIconProps } from '../ImageIcon';

import type { Meta, StoryObj } from '@storybook/react';

export default {
	component: ImageIcon,
} as Meta;

export const extraLargeImage: StoryObj<ImageIconProps> = {
	args: {
		src: 'https://ipfs.io/ipfs/QmZPWWy5Si54R3d26toaqRiqvCH7HkGdXkxwUgCm2oKKM2?filename=img-sq-01.png',
		alt: 'Blockdaemon',
		size: 'xl',
	},
};

export const largeIconNoImage: StoryObj<ImageIconProps> = {
	args: {
		src: null,
		fallback: 'dWallet',
		label: 'dWallet',
		size: 'lg',
	},
};

export const smallIconImage: StoryObj<ImageIconProps> = {
	args: {
		src: 'https://ipfs.io/ipfs/QmZPWWy5Si54R3d26toaqRiqvCH7HkGdXkxwUgCm2oKKM2?filename=img-sq-01.png',
		label: 'dWallet',
		size: 'sm',
		fallback: 'dWallet',
	},
};
