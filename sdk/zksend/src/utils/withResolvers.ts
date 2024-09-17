// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

export interface Resolvers<T = any> {
	promise: Promise<T>;
	reject: (error: Error) => void;
	resolve: (value: T) => void;
}

export function withResolvers<T = any>(): Resolvers<T> {
	let resolve: (value: T) => void;
	let reject: (error: Error) => void;

	const promise = new Promise<T>((res, rej) => {
		resolve = res;
		reject = rej;
	});

	return { promise, reject: reject!, resolve: resolve! };
}
