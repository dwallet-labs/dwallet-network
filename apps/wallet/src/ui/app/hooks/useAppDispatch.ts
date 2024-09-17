// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { AppDispatch } from '_store';
import { useDispatch } from 'react-redux';

export default function useAppDispatch() {
	return useDispatch<AppDispatch>();
}
