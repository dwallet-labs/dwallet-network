// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { openTransportReplayer, RecordStore } from '@ledgerhq/hw-transport-mocker';
import { expect, test } from 'vitest';

import Sui from '../src/Sui';

test('Sui init', async () => {
	const transport = await openTransportReplayer(RecordStore.fromString(''));
	const pkt = new Sui(transport);
	expect(pkt).not.toBe(undefined);
});
