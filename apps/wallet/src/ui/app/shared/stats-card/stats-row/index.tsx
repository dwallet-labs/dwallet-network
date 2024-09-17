// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { memo } from 'react';
import type { ReactNode } from 'react';

import st from './StatsRow.module.scss';

export type StatsRowProps = {
	children: ReactNode;
};

function StatsRow({ children }: StatsRowProps) {
	return <div className={st.container}>{children}</div>;
}

export default memo(StatsRow);
