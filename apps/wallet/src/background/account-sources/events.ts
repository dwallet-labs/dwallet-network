// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import mitt from 'mitt';

type AccountSourcesEvents = {
	accountSourcesChanged: void;
	accountSourceStatusUpdated: { accountSourceID: string };
};

export const accountSourcesEvents = mitt<AccountSourcesEvents>();
