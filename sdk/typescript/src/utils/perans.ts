// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

const PERA_NS_NAME_REGEX =
	/^(?!.*(^(?!@)|[-.@])($|[-.@]))(?:[a-z0-9-]{0,63}(?:\.[a-z0-9-]{0,63})*)?@[a-z0-9-]{0,63}$/i;
const PERA_NS_DOMAIN_REGEX = /^(?!.*(^|[-.])($|[-.]))(?:[a-z0-9-]{0,63}\.)+pera$/i;
const MAX_PERA_NS_NAME_LENGTH = 235;

export function isValidPeraNSName(name: string): boolean {
	if (name.length > MAX_PERA_NS_NAME_LENGTH) {
		return false;
	}

	if (name.includes('@')) {
		return PERA_NS_NAME_REGEX.test(name);
	}

	return PERA_NS_DOMAIN_REGEX.test(name);
}

export function normalizePeraNSName(name: string, format: 'at' | 'dot' = 'at'): string {
	const lowerCase = name.toLowerCase();
	let parts;

	if (lowerCase.includes('@')) {
		if (!PERA_NS_NAME_REGEX.test(lowerCase)) {
			throw new Error(`Invalid PeraNS name ${name}`);
		}
		const [labels, domain] = lowerCase.split('@');
		parts = [...(labels ? labels.split('.') : []), domain];
	} else {
		if (!PERA_NS_DOMAIN_REGEX.test(lowerCase)) {
			throw new Error(`Invalid PeraNS name ${name}`);
		}
		parts = lowerCase.split('.').slice(0, -1);
	}

	if (format === 'dot') {
		return `${parts.join('.')}.pera`;
	}

	return `${parts.slice(0, -1).join('.')}@${parts[parts.length - 1]}`;
}
