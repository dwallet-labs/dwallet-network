// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { execSync } from 'child_process';
import { mkdtemp } from 'fs/promises';
import { tmpdir } from 'os';
import path from 'path';
import type {
	DevInspectResults,
	PeraObjectChangeCreated,
	PeraObjectChangePublished,
	PeraTransactionBlockResponse,
} from '@pera-io/pera/client';
import { getFullnodeUrl, PeraClient } from '@pera-io/pera/client';
import { FaucetRateLimitError, getFaucetHost, requestPeraFromFaucetV0 } from '@pera-io/pera/faucet';
import { Ed25519Keypair } from '@pera-io/pera/keypairs/ed25519';
import { Transaction } from '@pera-io/pera/transactions';
import tmp from 'tmp';
import { retry } from 'ts-retry-promise';
import { expect } from 'vitest';

import { DeepBookClient } from '../../src/index.js';
import type { PoolSummary } from '../../src/types/index.js';
import { FLOAT_SCALING_FACTOR, NORMALIZED_PERA_COIN_TYPE } from '../../src/utils/index.js';

const DEFAULT_FAUCET_URL = import.meta.env.VITE_FAUCET_URL ?? getFaucetHost('localnet');
const DEFAULT_FULLNODE_URL = import.meta.env.VITE_FULLNODE_URL ?? getFullnodeUrl('localnet');
const PERA_BIN = import.meta.env.VITE_PERA_BIN ?? 'cargo run --bin pera';

export const DEFAULT_TICK_SIZE = 1n * FLOAT_SCALING_FACTOR;
export const DEFAULT_LOT_SIZE = 1n;

export class TestToolbox {
	keypair: Ed25519Keypair;
	client: PeraClient;
	configPath: string;

	constructor(keypair: Ed25519Keypair, client: PeraClient, configPath: string) {
		this.keypair = keypair;
		this.client = client;
		this.configPath = configPath;
	}

	address() {
		return this.keypair.getPublicKey().toPeraAddress();
	}

	public async getActiveValidators() {
		return (await this.client.getLatestPeraSystemState()).activeValidators;
	}
}

export function getClient(): PeraClient {
	return new PeraClient({
		url: DEFAULT_FULLNODE_URL,
	});
}

// TODO: expose these testing utils from @pera-io/pera
export async function setupPeraClient() {
	const keypair = Ed25519Keypair.generate();
	const address = keypair.getPublicKey().toPeraAddress();
	const client = getClient();
	await retry(() => requestPeraFromFaucetV0({ host: DEFAULT_FAUCET_URL, recipient: address }), {
		backoff: 'EXPONENTIAL',
		// overall timeout in 60 seconds
		timeout: 1000 * 60,
		// skip retry if we hit the rate-limit error
		retryIf: (error: any) => !(error instanceof FaucetRateLimitError),
		logger: (msg) => console.warn('Retrying requesting from faucet: ' + msg),
	});

	const tmpDirPath = path.join(tmpdir(), 'config-');
	const tmpDir = await mkdtemp(tmpDirPath);
	const configPath = path.join(tmpDir, 'client.yaml');
	execSync(`${PERA_BIN} client --yes --client.config ${configPath}`, { encoding: 'utf-8' });
	return new TestToolbox(keypair, client, configPath);
}

// TODO: expose these testing utils from @pera-io/pera
export async function publishPackage(packagePath: string, toolbox?: TestToolbox) {
	// TODO: We create a unique publish address per publish, but we really could share one for all publishes.
	if (!toolbox) {
		toolbox = await setupPeraClient();
	}

	// remove all controlled temporary objects on process exit
	tmp.setGracefulCleanup();

	const tmpobj = tmp.dirSync({ unsafeCleanup: true });

	const { modules, dependencies } = JSON.parse(
		execSync(
			`${PERA_BIN} move move --client.config ${toolbox.configPath} build --dump-bytecode-as-base64 --path ${packagePath} --install-dir ${tmpobj.name}`,
			{ encoding: 'utf-8' },
		),
	);
	const tx = new Transaction();
	const cap = tx.publish({
		modules,
		dependencies,
	});

	// Transfer the upgrade capability to the sender so they can upgrade the package later if they want.
	tx.transferObjects([cap], await toolbox.address());

	const { digest } = await toolbox.client.signAndExecuteTransaction({
		transaction: tx,
		signer: toolbox.keypair,
	});

	const publishTxn = await toolbox.client.waitForTransaction({
		digest: digest,
		options: { showObjectChanges: true, showEffects: true },
	});

	expect(publishTxn.effects?.status.status).toEqual('success');

	const packageId = ((publishTxn.objectChanges?.filter(
		(a) => a.type === 'published',
	) as PeraObjectChangePublished[]) ?? [])[0]?.packageId.replace(/^(0x)(0+)/, '0x') as string;

	expect(packageId).toBeTypeOf('string');

	console.info(`Published package ${packageId} from address ${toolbox.address()}}`);

	return { packageId, publishTxn };
}

export async function setupPool(toolbox: TestToolbox): Promise<PoolSummary> {
	const packagePath = __dirname + '/./data/test_coin';
	const { packageId } = await publishPackage(packagePath, toolbox);
	const baseAsset = `${packageId}::test::TEST`;
	const quoteAsset = NORMALIZED_PERA_COIN_TYPE;
	const deepbook = new DeepBookClient(toolbox.client);
	const tx = deepbook.createPool(baseAsset, quoteAsset, DEFAULT_TICK_SIZE, DEFAULT_LOT_SIZE);
	const resp = await executeTransaction(toolbox, tx);
	const event = resp.events?.find((e) => e.type.includes('PoolCreated')) as any;
	return {
		poolId: event.parsedJson.pool_id,
		baseAsset,
		quoteAsset,
	};
}

export async function setupDeepbookAccount(toolbox: TestToolbox): Promise<string> {
	const deepbook = new DeepBookClient(toolbox.client);
	const tx = deepbook.createAccount(toolbox.address());
	const resp = await executeTransaction(toolbox, tx);

	const accountCap = ((resp.objectChanges?.filter(
		(a) => a.type === 'created',
	) as PeraObjectChangeCreated[]) ?? [])[0].objectId;
	return accountCap;
}

export async function executeTransaction(
	toolbox: TestToolbox,
	tx: Transaction,
): Promise<PeraTransactionBlockResponse> {
	const resp = await toolbox.client.signAndExecuteTransaction({
		signer: toolbox.keypair,
		transaction: tx,
		options: {
			showEffects: true,
			showEvents: true,
			showObjectChanges: true,
		},
	});
	expect(resp.effects?.status.status).toEqual('success');
	return resp;
}

export async function devInspectTransaction(
	toolbox: TestToolbox,
	tx: Transaction,
): Promise<DevInspectResults> {
	return await toolbox.client.devInspectTransactionBlock({
		transactionBlock: tx,
		sender: toolbox.address(),
	});
}
