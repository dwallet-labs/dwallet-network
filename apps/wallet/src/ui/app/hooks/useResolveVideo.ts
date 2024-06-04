// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type SuiObjectResponse } from '@dwallet-network/dwallet.js/client';

import { useRecognizedPackages } from './useRecognizedPackages';

export function useResolveVideo(object?: SuiObjectResponse | null) {
	const recognizedPackages = useRecognizedPackages();

	if (!object) return null;

	const objectType =
		object.data?.type ??
		(object?.data?.content?.dataType === 'package' ? 'package' : object?.data?.content?.type) ??
		null;
	const isRecognized = objectType && recognizedPackages.includes(objectType.split('::')[0]);

	if (!isRecognized) return null;

	const display = object.data?.display?.data;

	return display?.video_url;
}
