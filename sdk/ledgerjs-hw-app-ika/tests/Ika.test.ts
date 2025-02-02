// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { openTransportReplayer, RecordStore } from '@ledgerhq/hw-transport-mocker';
import { expect, test } from 'vitest';

import Ika from '../src/Ika';

test('Ika init', async () => {
	const transport = await openTransportReplayer(RecordStore.fromString(''));
	const pkt = new Ika(transport);
	expect(pkt).not.toBe(undefined);
});
