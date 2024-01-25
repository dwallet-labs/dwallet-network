// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { type ComponentProps, type FC } from 'react';

export interface OnrampProvider {
	key: string;
	icon: FC<ComponentProps<'svg'>>;
	name: string;
	checkSupported(): Promise<boolean>;
	getUrl(address: string): Promise<string>;
}
