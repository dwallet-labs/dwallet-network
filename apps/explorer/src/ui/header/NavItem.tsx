// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { forwardRef, type ComponentProps, type ReactNode } from 'react';

export interface NavItemProps extends ComponentProps<'button'> {
	beforeIcon?: ReactNode;
	afterIcon?: ReactNode;
	children: ReactNode;
}

export const NavItem = forwardRef<HTMLButtonElement, NavItemProps>(
	({ children, beforeIcon, afterIcon, ...props }, ref) => (
		<button
			ref={ref}
			type="button"
			className="flex h-10 cursor-pointer items-center gap-2 rounded-md border-none bg-transparent px-3.5 py-2 text-heading6 font-medium text-steel-dark outline-none hover:bg-white/10 ui-open:bg-white/10"
			{...props}
		>
			{beforeIcon}
			{children}
			{afterIcon}
		</button>
	),
);
