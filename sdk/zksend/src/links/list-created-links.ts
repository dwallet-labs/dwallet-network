// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@ika-io/ika/bcs';
import type { IkaClient } from '@ika-io/ika/client';
import { IkaGraphQLClient } from '@ika-io/ika/graphql';
import { graphql } from '@ika-io/ika/graphql/schemas/2024.4';
import { fromBase64, normalizeIkaAddress } from '@ika-io/ika/utils';

import { ZkSendLink } from './claim.js';
import type { ZkBagContractOptions } from './zk-bag.js';
import { getContractIds } from './zk-bag.js';

const ListCreatedLinksQuery = graphql(`
	query listCreatedLinks($address: IkaAddress!, $function: String!, $cursor: String) {
		transactionBlocks(
			last: 10
			before: $cursor
			filter: { sentAddress: $address, function: $function }
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
	contract = getContractIds(network),
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
	client?: IkaClient;
	fetch?: typeof fetch;
}) {
	const gqlClient = new IkaGraphQLClient({
		url:
			network === 'testnet'
				? 'https://ika-testnet.mystenlabs.com/graphql'
				: 'https://ika-mainnet.mystenlabs.com/graphql',
		fetch: fetchFn,
	});

	const packageId = normalizeIkaAddress(contract.packageId);

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

				const kind = bcs.SenderSignedData.parse(fromBase64(node.bcs))?.[0]?.intentMessage.value.V1
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
