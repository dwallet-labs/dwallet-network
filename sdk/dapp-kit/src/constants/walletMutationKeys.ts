// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { MutationKey } from '@tanstack/react-query';

export const walletMutationKeys = {
	all: { baseScope: 'wallet' },
	connectWallet: formMutationKeyFn('connect-wallet'),
	autoconnectWallet: formMutationKeyFn('autoconnect-wallet'),
	disconnectWallet: formMutationKeyFn('disconnect-wallet'),
	signPersonalMessage: formMutationKeyFn('sign-personal-message'),
	signTransactionBlock: formMutationKeyFn('sign-transaction-block'),
	signAndExecuteTransactionBlock: formMutationKeyFn('sign-and-execute-transaction-block'),
	switchAccount: formMutationKeyFn('switch-account'),
};

function formMutationKeyFn(baseEntity: string) {
	return function mutationKeyFn(additionalKeys: MutationKey = []) {
		return [{ ...walletMutationKeys.all, baseEntity }, ...additionalKeys];
	};
}
