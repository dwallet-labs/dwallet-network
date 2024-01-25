// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import React from 'react';
import ReactDOM from 'react-dom/client';

import { EnokiFlowProvider } from '../src/react.tsx';
import { App } from './App.tsx';

ReactDOM.createRoot(document.getElementById('root')!).render(
	<React.StrictMode>
		<EnokiFlowProvider apiKey="enoki_apikey_ec23ee0a581fca24263243bc89f77bdf">
			{/* <EnokiFlowProvider apiUrl="http://localhost:3081/api/sdk" apiKey="enoki_apikey_dev"> */}
			<App />
		</EnokiFlowProvider>
	</React.StrictMode>,
);
