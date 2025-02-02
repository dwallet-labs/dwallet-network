// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const IKA_NS_NAME_REGEX =
	/^(?!.*(^(?!@)|[-.@])($|[-.@]))(?:[a-z0-9-]{0,63}(?:\.[a-z0-9-]{0,63})*)?@[a-z0-9-]{0,63}$/i;
const IKA_NS_DOMAIN_REGEX = /^(?!.*(^|[-.])($|[-.]))(?:[a-z0-9-]{0,63}\.)+ika$/i;
const MAX_IKA_NS_NAME_LENGTH = 235;

export function isValidIkaNSName(name: string): boolean {
	if (name.length > MAX_IKA_NS_NAME_LENGTH) {
		return false;
	}

	if (name.includes('@')) {
		return IKA_NS_NAME_REGEX.test(name);
	}

	return IKA_NS_DOMAIN_REGEX.test(name);
}

export function normalizeIkaNSName(name: string, format: 'at' | 'dot' = 'at'): string {
	const lowerCase = name.toLowerCase();
	let parts;

	if (lowerCase.includes('@')) {
		if (!IKA_NS_NAME_REGEX.test(lowerCase)) {
			throw new Error(`Invalid IkaNS name ${name}`);
		}
		const [labels, domain] = lowerCase.split('@');
		parts = [...(labels ? labels.split('.') : []), domain];
	} else {
		if (!IKA_NS_DOMAIN_REGEX.test(lowerCase)) {
			throw new Error(`Invalid IkaNS name ${name}`);
		}
		parts = lowerCase.split('.').slice(0, -1);
	}

	if (format === 'dot') {
		return `${parts.join('.')}.ika`;
	}

	return `${parts.slice(0, -1).join('.')}@${parts[parts.length - 1]}`;
}
