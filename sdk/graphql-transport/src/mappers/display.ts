// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { DisplayFieldsResponse } from '@ika-io/ika/client';

export function formatDisplay(object: {
	display?:
		| {
				key: string;
				value?: string | null | undefined;
				error?: string | null | undefined;
		  }[]
		| null;
}) {
	let display: DisplayFieldsResponse = {
		data: null,
		error: null,
	};

	if (object.display) {
		object.display.forEach((displayItem) => {
			if (displayItem.error) {
				display!.error = displayItem.error as never;
			} else if (displayItem.value != null) {
				if (!display!.data) {
					display!.data = {};
				}
				display!.data[displayItem.key] = displayItem.value;
			}
		});
	}

	return display;
}
