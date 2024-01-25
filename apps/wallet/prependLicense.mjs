// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { readFile, writeFile } from 'fs/promises';

const LICENSE = '// Copyright (c) Mysten Labs, Inc.\n// SPDX-License-Identifier: BSD-3-Clause-Clear\n\n';

async function prependLicense(filename) {
	const content = await readFile(filename, 'utf8');
	writeFile(filename, LICENSE + content);
}

prependLicense('src/shared/analytics/ampli/index.ts');
