// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { initGraphQLTada } from 'gql.tada';

import type { introspection } from '../../generated/2024.4/tada-env.js';
import type { CustomScalars } from '../../types.js';

export * from '../../types.js';

export type { FragmentOf, ResultOf, VariablesOf, TadaDocumentNode } from 'gql.tada';
export { readFragment, maskFragments } from 'gql.tada';

export const graphql = initGraphQLTada<{
	introspection: introspection;
	scalars: CustomScalars;
}>();
