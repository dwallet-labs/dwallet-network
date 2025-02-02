// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import { formatAddress, isValidIkaNSName } from '@ika-io/ika/utils';

type TxnAddressLinkProps = {
	address: string;
};

export function TxnAddressLink({ address }: TxnAddressLinkProps) {
	return (
		<ExplorerLink
			type={ExplorerLinkType.address}
			address={address}
			title="View on Ika Explorer"
			showIcon={false}
		>
			{isValidIkaNSName(address) ? address : formatAddress(address)}
		</ExplorerLink>
	);
}
