// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { thunkExtras } from '../redux/store/thunk-extras';

export function useBackgroundClient() {
	return thunkExtras.background;
}
