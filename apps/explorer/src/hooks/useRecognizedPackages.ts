// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useFeatureValue } from '@growthbook/growthbook-react';
import { SUI_FRAMEWORK_ADDRESS, SUI_SYSTEM_ADDRESS } from '@dwallet/dwallet.js/utils';

import { useNetwork } from '~/context';
import { Network } from '~/utils/api/DefaultRpcClient';

const DEFAULT_RECOGNIZED_PACKAGES = [SUI_FRAMEWORK_ADDRESS, SUI_SYSTEM_ADDRESS];

export function useRecognizedPackages() {
	const [network] = useNetwork();

	const recognizedPackages = useFeatureValue('recognized-packages', DEFAULT_RECOGNIZED_PACKAGES);

	// Our recognized package list is currently only available on mainnet
	return network === Network.MAINNET ? recognizedPackages : DEFAULT_RECOGNIZED_PACKAGES;
}
