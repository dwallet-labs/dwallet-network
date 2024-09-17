// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraClient, PeraObjectResponse } from '@pera-io/pera/client';

// get NFT's owner from RPC.
export const getOwner = async (client: PeraClient, nftId: string): Promise<string | null> => {
	const ownerResponse = await client.getObject({
		id: nftId,
		options: { showOwner: true },
	});
	const owner = ownerResponse.data?.owner;
	return (
		(owner as { AddressOwner: string })?.AddressOwner ||
		(owner as { ObjectOwner: string })?.ObjectOwner ||
		null
	);
};

// get avatar NFT Object from RPC.
export const getAvatar = async (client: PeraClient, avatar: string): Promise<PeraObjectResponse> => {
	return await client.getObject({
		id: avatar,
		options: {
			showDisplay: true,
			showOwner: true,
		},
	});
};
