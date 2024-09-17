// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { bcs } from '@pera-io/pera/bcs';
import {
	type DryRunTransactionBlockResponse,
	type ExecuteTransactionRequestType,
	type PeraClient,
	type PeraTransactionBlockResponse,
	type PeraTransactionBlockResponseOptions,
} from '@pera-io/pera/client';
import { messageWithIntent } from '@pera-io/pera/cryptography';
import { isTransaction, type Transaction } from '@pera-io/pera/transactions';
import { fromB64, toB64 } from '@pera-io/pera/utils';

export type SignedTransaction = {
	transactionBlockBytes: string;
	signature: string;
};

export type SignedMessage = {
	messageBytes: string;
	signature: string;
};

export abstract class WalletSigner {
	client: PeraClient;

	constructor(client: PeraClient) {
		this.client = client;
	}

	abstract signData(data: Uint8Array, clientIdentifier?: string): Promise<string>;

	abstract getAddress(): Promise<string>;

	async signMessage(
		input: { message: Uint8Array },
		clientIdentifier?: string,
	): Promise<SignedMessage> {
		const signature = await this.signData(
			messageWithIntent('PersonalMessage', bcs.vector(bcs.u8()).serialize(input.message).toBytes()),
		);

		return {
			messageBytes: toB64(input.message),
			signature,
		};
	}

	protected async prepareTransactionBlock(transactionBlock: Uint8Array | Transaction | string) {
		if (isTransaction(transactionBlock)) {
			// If the sender has not yet been set on the transaction, then set it.
			// NOTE: This allows for signing transactions with mis-matched senders, which is important for sponsored transactions.
			transactionBlock.setSenderIfNotSet(await this.getAddress());
			return await transactionBlock.build({
				client: this.client,
			});
		}

		if (typeof transactionBlock === 'string') {
			return fromB64(transactionBlock);
		}

		if (transactionBlock instanceof Uint8Array) {
			return transactionBlock;
		}
		throw new Error('Unknown transaction format');
	}

	async signTransactionBlock(
		input: {
			transactionBlock: Uint8Array | Transaction;
		},
		clientIdentifier?: string,
	): Promise<SignedTransaction> {
		const bytes = await this.prepareTransactionBlock(input.transactionBlock);
		const signature = await this.signData(messageWithIntent('TransactionData', bytes));

		return {
			transactionBlockBytes: toB64(bytes),
			signature,
		};
	}

	async signAndExecuteTransactionBlock(
		input: {
			transactionBlock: Uint8Array | Transaction;
			options?: PeraTransactionBlockResponseOptions;
			requestType?: ExecuteTransactionRequestType;
		},
		clientIdentifier?: string,
	): Promise<PeraTransactionBlockResponse> {
		const bytes = await this.prepareTransactionBlock(input.transactionBlock);
		const signed = await this.signTransactionBlock({
			transactionBlock: bytes,
		});

		return this.client.executeTransactionBlock({
			transactionBlock: bytes,
			signature: signed.signature,
			options: input.options,
			requestType: input.requestType,
		});
	}

	async dryRunTransactionBlock(input: {
		transactionBlock: Transaction | string | Uint8Array;
	}): Promise<DryRunTransactionBlockResponse> {
		return this.client.dryRunTransactionBlock({
			transactionBlock: await this.prepareTransactionBlock(input.transactionBlock),
		});
	}
}
