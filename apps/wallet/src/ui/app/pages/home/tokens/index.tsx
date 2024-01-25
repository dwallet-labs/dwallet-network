// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Route, Routes } from 'react-router-dom';

import TokenDetailsPage from './TokenDetailsPage';
import TokenDetails from './TokensDetails';

function TokensPage() {
	return (
		<Routes>
			<Route path="/" element={<TokenDetails />} />
			<Route path="/details" element={<TokenDetailsPage />} />
		</Routes>
	);
}

export default TokensPage;
