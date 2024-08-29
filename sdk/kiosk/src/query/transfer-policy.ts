// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { DWalletClient } from '@dwallet-network/dwallet.js/client';
import { isValidSuiAddress } from '@dwallet-network/dwallet.js/utils';

import { bcs } from '../bcs';
import {
	TRANSFER_POLICY_CAP_TYPE,
	TRANSFER_POLICY_CREATED_EVENT,
	TRANSFER_POLICY_TYPE,
	TransferPolicy,
	TransferPolicyCap,
} from '../types';
import { getAllOwnedObjects, parseTransferPolicyCapObject } from '../utils';

/**
 * Searches the `TransferPolicy`-s for the given type. The seach is performed via
 * the `TransferPolicyCreated` event. The policy can either be owned or shared,
 * and the caller needs to filter the results accordingly (ie single owner can not
 * be accessed by anyone but the owner).
 *
 * @param provider
 * @param type
 */
export async function queryTransferPolicy(
	client: DWalletClient,
	type: string,
): Promise<TransferPolicy[]> {
	// console.log('event type: %s', `${TRANSFER_POLICY_CREATED_EVENT}<${type}>`);
	const { data } = await client.queryEvents({
		query: {
			MoveEventType: `${TRANSFER_POLICY_CREATED_EVENT}<${type}>`,
		},
	});

	const search = data.map((event) => event.parsedJson as { id: string });
	const policies = await client.multiGetObjects({
		ids: search.map((policy) => policy.id),
		options: { showBcs: true, showOwner: true },
	});

	return policies
		.filter((policy) => !!policy && 'data' in policy)
		.map(({ data: policy }) => {
			// should never happen; policies are objects and fetched via an event.
			// policies are filtered for null and undefined above.
			if (!policy || !policy.bcs || !('bcsBytes' in policy.bcs)) {
				throw new Error(`Invalid policy: ${policy?.objectId}, expected object, got package`);
			}

			const parsed = bcs.de(TRANSFER_POLICY_TYPE, policy.bcs.bcsBytes, 'base64');

			return {
				id: policy?.objectId,
				type: `${TRANSFER_POLICY_TYPE}<${type}>`,
				owner: policy?.owner!,
				rules: parsed.rules,
				balance: parsed.balance,
			} as TransferPolicy;
		});
}

/**
 * A function to fetch all the user's kiosk Caps
 * And a list of the kiosk address ids.
 * Returns a list of `kioskOwnerCapIds` and `kioskIds`.
 * Extra options allow pagination.
 * @returns TransferPolicyCap Object ID | undefined if not found.
 */
export async function queryTransferPolicyCapsByType(
	client: DWalletClient,
	address: string,
	type: string,
): Promise<TransferPolicyCap[]> {
	if (!isValidSuiAddress(address)) return [];

	const filter = {
		MatchAll: [
			{
				StructType: `${TRANSFER_POLICY_CAP_TYPE}<${type}>`,
			},
		],
	};

	// fetch owned kiosk caps, paginated.
	const data = await getAllOwnedObjects({
		client,
		filter,
		owner: address,
	});

	return data
		.map((item) => parseTransferPolicyCapObject(item))
		.filter((item) => !!item) as TransferPolicyCap[];
}

/**
 * A function to fetch all the user's kiosk Caps
 * And a list of the kiosk address ids.
 * Returns a list of `kioskOwnerCapIds` and `kioskIds`.
 * Extra options allow pagination.
 * @returns TransferPolicyCap Object ID | undefined if not found.
 */
export async function queryOwnedTransferPolicies(
	client: DWalletClient,
	address: string,
): Promise<TransferPolicyCap[] | undefined> {
	if (!isValidSuiAddress(address)) return;

	const filter = {
		MatchAll: [
			{
				MoveModule: {
					module: 'transfer_policy',
					package: '0x2',
				},
			},
		],
	};

	// fetch all owned kiosk caps, paginated.
	const data = await getAllOwnedObjects({ client, owner: address, filter });

	const policies: TransferPolicyCap[] = [];

	for (const item of data) {
		const data = parseTransferPolicyCapObject(item);
		if (data) policies.push(data);
	}

	return policies;
}
