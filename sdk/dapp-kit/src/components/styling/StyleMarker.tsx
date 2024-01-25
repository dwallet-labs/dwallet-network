// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Slot } from '@radix-ui/react-slot';
import type { ComponentPropsWithoutRef, ElementRef, ReactNode } from 'react';
import { forwardRef } from 'react';

import { styleDataAttribute } from '../../constants/styleDataAttribute.js';

import './StyleMarker.css.js';

type StyleMarker = {
	children: ReactNode;
};

export const StyleMarker = forwardRef<
	ElementRef<typeof Slot>,
	ComponentPropsWithoutRef<typeof Slot>
>(({ children, ...props }, forwardedRef) => (
	<Slot ref={forwardedRef} {...props} {...styleDataAttribute}>
		{children}
	</Slot>
));
StyleMarker.displayName = 'StyleMarker';
