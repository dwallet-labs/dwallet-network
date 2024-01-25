// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export enum AppType {
	unknown,
	fullscreen,
	popup,
}

export function getFromLocationSearch(search: string) {
	if (/type=popup/.test(window.location.search)) {
		return AppType.popup;
	}
	return AppType.fullscreen;
}
