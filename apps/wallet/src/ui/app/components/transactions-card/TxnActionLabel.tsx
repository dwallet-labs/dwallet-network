// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Text } from '_app/shared/text';
import { formatAddress } from '@mysten/sui.js/utils';

type TxnTypeProps = {
	address: string;
	moveCallFnName?: string;
	isTransfer: boolean;
	isSender: boolean;
};

export function TxnTypeLabel({ address, moveCallFnName, isTransfer, isSender }: TxnTypeProps) {
	const transferLabel = isSender ? 'To' : 'From';
	const label = isTransfer ? transferLabel : 'Action';
	const content = isTransfer ? formatAddress(address) : moveCallFnName?.replace(/_/g, ' ');

	return content ? (
		<div className="flex gap-1 break-all capitalize mt-1">
			<Text color="steel-darker" weight="semibold" variant="subtitle">
				{label}:
			</Text>
			<div className="flex-1">
				<Text color="steel-darker" weight="normal" variant="subtitle" mono={isTransfer}>
					{content}
				</Text>
			</div>
		</div>
	) : null;
}
