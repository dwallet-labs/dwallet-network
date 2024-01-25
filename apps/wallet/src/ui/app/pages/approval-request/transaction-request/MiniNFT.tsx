// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import st from './TransactionRequest.module.scss';

export type MiniNFTProps = {
	size?: 'xs' | 'sm';
	url: string;
	name?: string | null;
};

export function MiniNFT({ size = 'sm', url, name }: MiniNFTProps) {
	const sizes = size === 'xs' ? st.nftImageTiny : st.nftImageSmall;
	return <img src={url} className={sizes} alt={name || 'Nft Image'} />;
}
