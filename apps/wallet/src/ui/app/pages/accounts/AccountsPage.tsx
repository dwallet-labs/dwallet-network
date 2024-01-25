// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Outlet } from 'react-router-dom';

import { Toaster } from '../../shared/toaster';

export function AccountsPage() {
	return (
		<>
			<Outlet />
			<Toaster />
		</>
	);
}
