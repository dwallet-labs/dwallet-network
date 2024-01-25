// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import Browser from 'webextension-polyfill';

export function injectDappInterface() {
	const script = document.createElement('script');
	script.setAttribute('src', Browser.runtime.getURL('dapp-interface.js'));
	const container = document.head || document.documentElement;
	container.insertBefore(script, container.firstElementChild);
	container.removeChild(script);
}
