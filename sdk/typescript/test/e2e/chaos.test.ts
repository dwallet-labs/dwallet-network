import fs from 'fs';
import {
	CoreV1Api,
	KubeConfig,
	loadYaml,
	V1ConfigMap,
	V1Namespace,
	V1Pod,
	V1Service,
} from '@kubernetes/client-node';
import Handlebars from 'handlebars';
import { describe, it } from 'vitest';

const createNamespace = async (kc: KubeConfig, namespaceName: string) => {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const namespaceBody: V1Namespace = {
		metadata: {
			name: namespaceName,
		},
	};
	await k8sApi.createNamespace({ body: namespaceBody }).catch((err) => {
		if (err.response?.statusCode !== 409) throw err;
	});
};

const CONFIG_MAP_NAME = 'ika-chaos-test-config';
const createConfigMap = async (kc: KubeConfig, namespaceName: string, numOfValidators: number) => {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const fullNodeYaml = fs.readFileSync(
		'/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/publisher/fullnode.yaml',
		'utf8',
	);
	const validatorsConfig: Record<string, string> = {};

	for (let i = 0; i < numOfValidators; i++) {
		validatorsConfig[`validator${i + 1}_class-groups.key`] = fs.readFileSync(
			`/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/val${i + 1}.beta50.devnet.ika-network.net/key-pairs/class-groups.key`,
			'utf8',
		);
		validatorsConfig[`validator${i + 1}_consensus.key`] = fs.readFileSync(
			`/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/val${i + 1}.beta50.devnet.ika-network.net/key-pairs/consensus.key`,
			'utf8',
		);
		validatorsConfig[`validator${i + 1}_network.key`] = fs.readFileSync(
			`/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/val${i + 1}.beta50.devnet.ika-network.net/key-pairs/network.key`,
			'utf8',
		);
		validatorsConfig[`validator${i + 1}_protocol.key`] = fs.readFileSync(
			`/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/val${i + 1}.beta50.devnet.ika-network.net/key-pairs/protocol.key`,
			'utf8',
		);
		validatorsConfig[`validator${i + 1}.yaml`] = fs.readFileSync(
			`/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/val${i + 1}.beta50.devnet.ika-network.net/validator.yaml`,
			'utf8',
		);
	}

	const configMap: V1ConfigMap = {
		metadata: {
			namespace: namespaceName,
			name: CONFIG_MAP_NAME,
		},
		data: {
			'fullnode.yaml': fullNodeYaml,
			'notifier.key': fs.readFileSync(
				'/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/publisher/sui_config/publisher.key',
				'utf8',
			),
			...validatorsConfig,
		},
	};
	await k8sApi.createNamespacedConfigMap({
		namespace: namespaceName,
		body: configMap,
	});
};

async function createNetworkServices(
	kc: KubeConfig,
	namespaceName: string,
	numOfValidators: number,
) {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const template = fs.readFileSync(
		'/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/service_template.yaml',
		'utf-8',
	);
	const compiled = Handlebars.compile(template);
	for (let i = 0; i < numOfValidators; i++) {
		const serviceBody = loadYaml(compiled({ validatorIndex: i + 1 }));
		await k8sApi.createNamespacedService({
			namespace: namespaceName,
			body: serviceBody as V1Service,
		});
	}
}

async function createPods(kc: KubeConfig, namespaceName: string, numOfValidators: number) {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	for (let i = 0; i < numOfValidators; i++) {
		const pod: V1Pod = {
			metadata: {
				name: `ika-new-devnet-ika-val-${i + 1}`,
				namespace: namespaceName,
			},
			spec: {
				containers: [
					{
						env: [
							{
								name: 'RUST_LOG',
								value: 'off,ika_node=info,ika_core=info',
							},
							{
								name: 'RUST_MIN_STACK',
								value: '16777216',
							},
						],
						command: ['/opt/ika/bin/ika-node', '--config-path', '/opt/ika/config/validator.yaml'],
						name: 'ika-node',
						image:
							'us-docker.pkg.dev/common-449616/ika-common-containers/ika-node:devnet-v0.0.6-arm64',
						volumeMounts: [
							{
								name: 'config-vol',
								mountPath: '/opt/ika/key-pairs/class-groups.key',
								subPath: 'class-groups.key',
							},
							{
								name: 'config-vol',
								mountPath: '/opt/ika/key-pairs/consensus.key',
								subPath: 'consensus.key',
							},
							{
								name: 'config-vol',
								mountPath: '/opt/ika/key-pairs/network.key',
								subPath: 'network.key',
							},
							{
								name: 'config-vol',
								mountPath: '/opt/ika/key-pairs/protocol.key',
								subPath: 'protocol.key',
							},
							{
								name: 'config-vol',
								mountPath: '/opt/ika/config/validator.yaml',
								subPath: 'validator.yaml',
							},
						],
					},
				],
				volumes: [
					{
						name: 'config-vol',
						configMap: {
							name: CONFIG_MAP_NAME,
							items: [
								{
									key: `validator${i + 1}_class-groups.key`,
									path: 'class-groups.key',
								},
								{
									key: `validator${i + 1}_consensus.key`,
									path: 'consensus.key',
								},
								{
									key: `validator${i + 1}_network.key`,
									path: 'network.key',
								},
								{
									key: `validator${i + 1}_protocol.key`,
									path: 'protocol.key',
								},
								{
									key: `validator${i + 1}.yaml`,
									path: 'validator.yaml',
								},
							],
						},
					},
				],
				restartPolicy: 'Always',
			},
		};
		await k8sApi.createNamespacedPod({
			namespace: namespaceName,
			body: pod,
		});
	}
	let fullnodePod = {
		metadata: {
			name: `ika-fullnode`,
			namespace: namespaceName,
		},
		spec: {
			containers: [
				{
					env: [
						{
							name: 'RUST_LOG',
							value: 'off,ika_node=info,ika_core=info',
						},
						{
							name: 'RUST_MIN_STACK',
							value: '16777216',
						},
					],
					command: ['/opt/ika/bin/ika-node', '--config-path', '/opt/ika/config/fullnode.yaml'],
					name: 'ika-node',
					image:
						'us-docker.pkg.dev/common-449616/ika-common-containers/ika-node:devnet-v0.0.6-arm64',
					volumeMounts: [
						{
							name: 'config-vol',
							mountPath: '/opt/ika/key-pairs/notifier.key',
							subPath: 'notifier.key',
						},
						{
							name: 'config-vol',
							mountPath: '/opt/ika/config/fullnode.yaml',
							subPath: 'fullnode.yaml',
						},
					],
				},
			],
			volumes: [
				{
					name: 'config-vol',
					configMap: {
						name: CONFIG_MAP_NAME,
						items: [
							{
								key: `notifier.key`,
								path: 'notifier.key',
							},
							{
								key: `fullnode.yaml`,
								path: 'fullnode.yaml',
							},
						],
					},
				},
			],
			restartPolicy: 'Always',
		},
	};
	await k8sApi.createNamespacedPod({
		namespace: namespaceName,
		body: fullnodePod,
	});
}

describe('run chain chaos testing', () => {
	it('create and deploy the config map', async () => {
		const kc = new KubeConfig();
		kc.loadFromDefault();
		const namespaceName = generateUniqueNamespace();
		console.log(`Creating namespace: ${namespaceName}`);
		await createNamespace(kc, namespaceName);
		await createConfigMap(kc, namespaceName, 4);
		await createPods(kc, namespaceName, 4);
		await createNetworkServices(kc, namespaceName, 4);
	});
});

function generateUniqueNamespace(prefix = 'chaos-test'): string {
	const timestamp = Date.now().toString();
	return `${prefix}-${timestamp}`;
}
