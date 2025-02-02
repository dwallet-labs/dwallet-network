// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaObjectData } from '@ika-io/ika/client';

import useFileExtensionType from './useFileExtensionType';
import useMediaUrl from './useMediaUrl';

export default function useNFTBasicData(nftObj: IkaObjectData | null) {
	const nftObjectID = nftObj?.objectId || null;
	const filePath = useMediaUrl(nftObj?.content || null);
	let objType = null;
	let nftFields = null;
	if (nftObj && nftObj.content?.dataType === 'moveObject') {
		objType = nftObj.content?.type;
		nftFields = nftObj?.content?.dataType === 'moveObject' ? nftObj.content.fields : null;
	}
	const fileExtensionType = useFileExtensionType(filePath || '');
	return {
		nftObjectID,
		filePath,
		nftFields,
		fileExtensionType,
		objType,
	};
}
