// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type IkaObjectResponse } from '@ika-io/ika/client';

import { useRecognizedPackages } from './useRecognizedPackages';

export function useResolveVideo(object?: IkaObjectResponse | null) {
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
