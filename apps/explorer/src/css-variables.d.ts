// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// eslint-disable-next-line react/no-typos
import 'react';

declare module 'react' {
	interface CSSProperties {
		[key: `--${string}`]: string | number | null;
	}
}
