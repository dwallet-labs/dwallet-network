// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

// eslint-disable-next-line @typescript-eslint/no-unused-vars
import type { Runtime } from 'webextension-polyfill';

declare module 'webextension-polyfill' {
	declare namespace Runtime {
		declare interface MessageSender {
			// Chrome API has origin since v80 https://developer.chrome.com/docs/extensions/reference/runtime/#type-MessageSender
			// Not sure why it's not in the polyfill
			origin?: string;
		}
	}
}
