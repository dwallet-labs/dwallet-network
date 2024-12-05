// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaParsedData } from '@ika-io/ika/client';
import { useMemo } from 'react';

export const parseIpfsUrl = (ipfsUrl: string) =>
	ipfsUrl.replace(/^ipfs:\/\//, 'https://ipfs.io/ipfs/');

export default function useMediaUrl(objData: IkaParsedData | null) {
	const { fields } =
		((objData?.dataType === 'moveObject' && objData) as {
			fields: { url?: string; metadata?: { fields: { url: string } } };
		}) || {};
	return useMemo(() => {
		if (fields) {
			const mediaUrl = fields.url || fields.metadata?.fields.url;
			if (typeof mediaUrl === 'string') {
				return parseIpfsUrl(mediaUrl);
			}
		}
		return null;
	}, [fields]);
}
