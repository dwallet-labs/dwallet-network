// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { expect, it } from 'vitest';

import { setup } from './utils/setup';

it('can fetch protocol config', async () => {
	const toolbox = await setup();
	const config = await toolbox.client.getProtocolConfig();
	expect(config).toBeTypeOf('object');
});
