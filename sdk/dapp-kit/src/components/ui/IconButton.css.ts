// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { style } from '@vanilla-extract/css';

import { themeVars } from '../../themes/themeContract.js';

export const container = style({
	borderRadius: 9999,
	padding: 8,
	color: themeVars.colors.iconButton,
	backgroundColor: themeVars.backgroundColors.iconButton,
	':hover': {
		backgroundColor: themeVars.backgroundColors.iconButtonHover,
	},
});
