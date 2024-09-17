// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { execSync } from 'child_process';
import { mkdtemp } from 'fs/promises';
import { tmpdir } from 'os';
import path from 'path';
import type {
	DevInspectResults,
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

import type { KioskClient } from '../../src/index.js';
import { KioskTransaction } from '../../src/index.js';

//@ts-ignore-next-line
const DEFAULT_FAUCET_URL = import.meta.env.VITE_FAUCET_URL ?? getFaucetHost('localnet');
//@ts-ignore-next-line
const DEFAULT_FULLNODE_URL = import.meta.env.VITE_FULLNODE_URL ?? getFullnodeUrl('localnet');
//@ts-ignore-next-line
const PERA_BIN = import.meta.env.VITE_PERA_BIN ?? 'cargo run --bin pera';

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
			`${PERA_BIN} move --client.config ${toolbox.configPath} build --dump-bytecode-as-base64 --path ${packagePath} --install-dir ${tmpobj.name}`,
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

export async function publishExtensionsPackage(toolbox: TestToolbox): Promise<string> {
	const packagePath = __dirname + '/../../../../kiosk';
	const { packageId } = await publishPackage(packagePath, toolbox);

	return packageId;
}

export async function publishHeroPackage(toolbox: TestToolbox): Promise<string> {
	const packagePath = __dirname + '/./data/hero';
	const { packageId } = await publishPackage(packagePath, toolbox);

	return packageId;
}

export function print(item: any) {
	console.dir(item, { depth: null });
}

export async function mintHero(toolbox: TestToolbox, packageId: string): Promise<string> {
	const tx = new Transaction();
	const hero = tx.moveCall({
		target: `${packageId}::hero::mint_hero`,
	});
	tx.transferObjects([hero], await toolbox.address());

	const res = await executeTransaction(toolbox, tx);

	return getCreatedObjectIdByType(res, 'hero::Hero');
}

export async function mintVillain(toolbox: TestToolbox, packageId: string): Promise<string> {
	const tx = new Transaction();
	const hero = tx.moveCall({
		target: `${packageId}::hero::mint_villain`,
	});
	tx.transferObjects([hero], await toolbox.address());

	const res = await executeTransaction(toolbox, tx);

	return getCreatedObjectIdByType(res, 'hero::Villain');
}

// create a non-personal kiosk.
export async function createKiosk(toolbox: TestToolbox, kioskClient: KioskClient) {
	const tx = new Transaction();

	new KioskTransaction({ transaction: tx, kioskClient }).createAndShare(toolbox.address());

	await executeTransaction(toolbox, tx);
}

// Create a personal Kiosk.
export async function createPersonalKiosk(toolbox: TestToolbox, kioskClient: KioskClient) {
	const tx = new Transaction();
	new KioskTransaction({ transaction: tx, kioskClient }).createPersonal().finalize();

	await executeTransaction(toolbox, tx);
}

function getCreatedObjectIdByType(res: PeraTransactionBlockResponse, type: string): string {
	return res.objectChanges?.filter(
		(x) => x.type === 'created' && x.objectType.endsWith(type),
		//@ts-ignore-next-line
	)[0].objectId;
}

export async function getPublisherObject(toolbox: TestToolbox): Promise<string> {
	let res = await toolbox.client.getOwnedObjects({
		filter: {
			StructType: '0x2::package::Publisher',
		},
		owner: toolbox.address(),
	});

	let publisherObj = res.data[0].data?.objectId;
	expect(publisherObj).not.toBeUndefined();

	return publisherObj ?? '';
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
