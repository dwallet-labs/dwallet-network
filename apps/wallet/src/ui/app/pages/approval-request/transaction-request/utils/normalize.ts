// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type IkaMoveNormalizedType } from '@ika-io/ika/client';

export interface TypeReference {
	address: string;
	module: string;
	name: string;
	typeArguments: IkaMoveNormalizedType[];
}

export const TX_CONTEXT_TYPE = '0x2::tx_context::TxContext';

/** Takes a normalized move type and returns the address information contained within it */
export function unwrapTypeReference(type: IkaMoveNormalizedType): null | TypeReference {
	if (typeof type === 'object') {
		if ('Struct' in type) {
			return type.Struct;
		}
		if ('Reference' in type) {
			return unwrapTypeReference(type.Reference);
		}
		if ('MutableReference' in type) {
			return unwrapTypeReference(type.MutableReference);
		}
		if ('Vector' in type) {
			return unwrapTypeReference(type.Vector);
		}
	}
	return null;
}
