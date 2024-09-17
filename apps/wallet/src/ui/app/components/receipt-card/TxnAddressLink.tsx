// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';
import { formatAddress, isValidPeraNSName } from '@pera-io/pera/utils';

type TxnAddressLinkProps = {
	address: string;
};

export function TxnAddressLink({ address }: TxnAddressLinkProps) {
	return (
		<ExplorerLink
			type={ExplorerLinkType.address}
			address={address}
			title="View on Pera Explorer"
			showIcon={false}
		>
			{isValidPeraNSName(address) ? address : formatAddress(address)}
		</ExplorerLink>
	);
}
