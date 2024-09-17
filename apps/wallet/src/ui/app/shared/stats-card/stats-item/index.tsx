// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import cl from 'clsx';
import { memo } from 'react';
import type { ReactNode } from 'react';

import st from './StatsItem.module.scss';

export type StatsItemProps = {
	className?: string;
	title: string | ReactNode;
	value: string | ReactNode;
};

function StatsItem({ className, title, value }: StatsItemProps) {
	return (
		<div className={cl(className, st.container)}>
			<div className={st.title}>{title}</div>
			<div className={st.value}>{value}</div>
		</div>
	);
}

export default memo(StatsItem);
