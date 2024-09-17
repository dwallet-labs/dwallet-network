// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import mitt from 'mitt';

type AccountsEvents = {
	accountsChanged: void;
	accountStatusChanged: { accountID: string };
	activeAccountChanged: { accountID: string };
};

export const accountsEvents = mitt<AccountsEvents>();
