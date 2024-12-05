// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { normalizeIkaNSName } from '@ika-io/ika/utils';

import { useResolveIkaNSName as useResolveIkaNSNameCore } from '../../../../../core';

export function useResolveIkaNSName(address?: string) {
	const enableNewIkansFormat = useFeatureIsOn('wallet-enable-new-ikans-name-format');
	const { data } = useResolveIkaNSNameCore(address);
	return data ? normalizeIkaNSName(data, enableNewIkansFormat ? 'at' : 'dot') : undefined;
}
