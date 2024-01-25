// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Outlet } from 'react-router-dom';

import { Header } from '@/components/header';
import { Warning } from '@/components/warning';

export function Root() {
	return (
		<div>
			<Header />
			<div className="container py-8">
				<Outlet />
			</div>
			<Warning />
		</div>
	);
}
