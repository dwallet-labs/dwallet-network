// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useCallback, type MouseEventHandler } from 'react';
import { toast } from 'react-hot-toast';

export type CopyOptions = {
	copySuccessMessage?: string;
};

export function useCopyToClipboard(
	textToCopy: string,
	{ copySuccessMessage = 'Copied' }: CopyOptions = {},
) {
	return useCallback<MouseEventHandler>(
		async (e) => {
			e.stopPropagation();
			e.preventDefault();
			try {
				await navigator.clipboard.writeText(textToCopy);
				toast.success(copySuccessMessage);
			} catch (e) {
				// silence clipboard errors
			}
		},
		[textToCopy, copySuccessMessage],
	);
}
