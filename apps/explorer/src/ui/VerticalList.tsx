// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import clsx from 'clsx';
import { type ReactNode } from 'react';

export interface ListItemProps {
	active?: boolean;
	children: ReactNode;
	onClick?(): void;
}

export function ListItem({ active, children, onClick }: ListItemProps) {
	return (
		<li className="list-none">
			<button
				type="button"
				className={clsx(
					'block w-full cursor-pointer rounded-md border px-2.5 py-2 text-left text-body',
					active
						? 'border-gray-40 bg-hero font-semibold text-steel-darker shadow-sm'
						: 'border-transparent bg-steel-dark font-medium text-gray-90',
				)}
				onClick={onClick}
			>
				{children}
			</button>
		</li>
	);
}

export interface VerticalListProps {
	children: ReactNode;
}

export function VerticalList({ children }: VerticalListProps) {
	return <ul className="m-0 flex flex-col gap-1 p-0">{children}</ul>;
}
