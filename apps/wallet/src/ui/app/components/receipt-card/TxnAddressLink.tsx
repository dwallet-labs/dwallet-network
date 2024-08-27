// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import { isSuiNSName } from '@mysten/core';
import { formatAddress } from '@dwallet-network/dwallet.js/utils';

type TxnAddressLinkProps = {
	address: string;
};

export function TxnAddressLink({ address }: TxnAddressLinkProps) {
	return (
		<ExplorerLink
			type={ExplorerLinkType.address}
			address={address}
			title="View on dWallet Explorer"
			showIcon={false}
		>
			{isSuiNSName(address) ? address : formatAddress(address)}
		</ExplorerLink>
	);
}
