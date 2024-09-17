// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { normalizePeraNSName } from '@pera-io/pera/utils';

import { useResolvePeraNSName as useResolvePeraNSNameCore } from '../../../../../core';

export function useResolvePeraNSName(address?: string) {
	const enableNewPeransFormat = useFeatureIsOn('wallet-enable-new-perans-name-format');
	const { data } = useResolvePeraNSNameCore(address);
	return data ? normalizePeraNSName(data, enableNewPeransFormat ? 'at' : 'dot') : undefined;
}
