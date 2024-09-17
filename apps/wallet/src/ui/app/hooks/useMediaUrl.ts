// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraParsedData } from '@pera-io/pera/client';
import { useMemo } from 'react';

export const parseIpfsUrl = (ipfsUrl: string) =>
	ipfsUrl.replace(/^ipfs:\/\//, 'https://ipfs.io/ipfs/');

export default function useMediaUrl(objData: PeraParsedData | null) {
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
