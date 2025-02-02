// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { CodegenConfig } from '@graphql-codegen/cli';

const header = `
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
/* eslint-disable */
`.trimStart();

const config: CodegenConfig = {
	overwrite: true,
	schema: '../../crates/ika-graphql-rpc/schema.graphql',
	documents: ['src/queries/*.graphql'],
	ignoreNoDocuments: true,
	generates: {
		'src/generated/queries.ts': {
			plugins: [
				{
					add: {
						content: header,
					},
				},
				'typescript',
				'typescript-operations',
				{
					'typed-document-node': {
						documentMode: 'string',
					},
				},
			],
		},
	},
};

export default config;
