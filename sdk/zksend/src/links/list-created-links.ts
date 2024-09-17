// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@pera-io/pera/bcs';
import type { PeraClient } from '@pera-io/pera/client';
import { PeraGraphQLClient } from '@pera-io/pera/graphql';
import { graphql } from '@pera-io/pera/graphql/schemas/2024.4';
import { fromB64, normalizePeraAddress } from '@pera-io/pera/utils';

import { ZkSendLink } from './claim.js';
import type { ZkBagContractOptions } from './zk-bag.js';
import { MAINNET_CONTRACT_IDS } from './zk-bag.js';

const ListCreatedLinksQuery = graphql(`
	query listCreatedLinks($address: PeraAddress!, $function: String!, $cursor: String) {
		transactionBlocks(
			last: 10
			before: $cursor
			filter: { signAddress: $address, function: $function, kind: PROGRAMMABLE_TX }
		) {
			pageInfo {
				startCursor
				hasPreviousPage
			}
			nodes {
				effects {
					timestamp
				}
				digest
				bcs
			}
		}
	}
`);

export async function listCreatedLinks({
	address,
	cursor,
	network,
	contract = MAINNET_CONTRACT_IDS,
	fetch: fetchFn,
	...linkOptions
}: {
	address: string;
	contract?: ZkBagContractOptions;
	cursor?: string;
	network?: 'mainnet' | 'testnet';

	// Link options:
	host?: string;
	path?: string;
	claimApi?: string;
	client?: PeraClient;
	fetch?: typeof fetch;
}) {
	const gqlClient = new PeraGraphQLClient({
		url:
			network === 'testnet'
				? 'https://pera-testnet.mystenlabs.com/graphql'
				: 'https://pera-mainnet.mystenlabs.com/graphql',
		fetch: fetchFn,
	});

	const packageId = normalizePeraAddress(contract.packageId);

	const page = await gqlClient.query({
		query: ListCreatedLinksQuery,
		variables: {
			address,
			cursor,
			function: `${packageId}::zk_bag::new`,
		},
	});

	const transactionBlocks = page.data?.transactionBlocks;

	if (!transactionBlocks || page.errors?.length) {
		throw new Error('Failed to load created links');
	}

	const links = (
		await Promise.all(
			transactionBlocks.nodes.map(async (node) => {
				if (!node.bcs) {
					return null;
				}

				const kind = bcs.SenderSignedData.parse(fromB64(node.bcs))?.[0]?.intentMessage.value.V1
					.kind;

				if (!kind.ProgrammableTransaction) {
					return null;
				}

				const { inputs, commands } = kind.ProgrammableTransaction;

				const fn = commands.find(
					(command) =>
						command.MoveCall?.package === packageId &&
						command.MoveCall.module === 'zk_bag' &&
						command.MoveCall.function === 'new',
				);

				if (!fn?.MoveCall) {
					return null;
				}

				const addressArg = fn.MoveCall.arguments[1];

				if (addressArg.$kind !== 'Input') {
					throw new Error('Invalid address argument');
				}

				const input = inputs[addressArg.Input];

				if (!input.Pure) {
					throw new Error('Expected Address input to be a Pure value');
				}

				const address = bcs.Address.fromBase64(input.Pure.bytes);

				const link = new ZkSendLink({
					network,
					address,
					contract,
					isContractLink: true,
					...linkOptions,
				});

				await link.loadAssets();

				return {
					link,
					claimed: !!link.claimed,
					assets: link.assets!,
					digest: node.digest,
					createdAt: node.effects?.timestamp!,
				};
			}),
		)
	).reverse();

	return {
		cursor: transactionBlocks.pageInfo.startCursor,
		hasNextPage: transactionBlocks.pageInfo.hasPreviousPage,
		links: links.filter((link): link is NonNullable<typeof link> => link !== null),
	};
}
