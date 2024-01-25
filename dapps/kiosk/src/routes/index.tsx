// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { createBrowserRouter } from 'react-router-dom';

import Root from '../Root';
import Home from './Home';
import SingleKiosk from './SingleKiosk';

export const router = createBrowserRouter([
	{
		path: '/',
		element: <Root />,
		children: [
			{
				path: '',
				element: <Home />,
			},
			{
				path: '/kiosk/:id',
				element: <SingleKiosk />,
			},
		],
	},
]);
