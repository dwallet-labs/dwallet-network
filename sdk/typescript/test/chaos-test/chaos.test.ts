import { CoreV1Api, KubeConfig, V1ConfigMap, V1Namespace, V1Pod } from '@kubernetes/client-node';
import fs from 'fs';
import path from 'path';
import { describe, it } from 'vitest';

const CONFIG_MAP_NAME = 'ika-chaos-test-config';
const NETWORK_SERVICE_NAME = 'ika-dns-service';
const NAMESPACE_NAME = 'ika';

const createNamespace = async (kc: KubeConfig, namespaceName: string) => {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const namespaceBody: V1Namespace = {
		metadata: {
			name: namespaceName,
		},
	};
	await k8sApi.createNamespace({ body: namespaceBody });
};

async function createConfigMap(
	kc: KubeConfig,
	namespaceName: string,
	numOfValidators: number,
): Promise<V1ConfigMap> {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const fullNodeYaml = fs.readFileSync(
		path.resolve(
			process.cwd(),
			`${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/publisher/fullnode.yaml`,
		),
		'utf8',
	);
	const validatorsConfig: Record<string, string> = {};
	for (let i = 0; i < numOfValidators; i++) {
		validatorsConfig[`validator${i + 1}_class-groups.key`] = fs.readFileSync(
			path.resolve(
				process.cwd(),
				`${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/val${i + 1}.${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/key-pairs/class-groups.key`,
			),
			'utf8',
		);
		validatorsConfig[`validator${i + 1}_consensus.key`] = fs.readFileSync(
			`${process.cwd()}/${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/val${i + 1}.${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/key-pairs/consensus.key`,
			'utf8',
		);
		validatorsConfig[`validator${i + 1}_network.key`] = fs.readFileSync(
			`${process.cwd()}/${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/val${i + 1}.${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/key-pairs/network.key`,
			'utf8',
		);
		validatorsConfig[`validator${i + 1}_protocol.key`] = fs.readFileSync(
			`${process.cwd()}/${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/val${i + 1}.${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/key-pairs/protocol.key`,
			'utf8',
		);
		validatorsConfig[`validator${i + 1}.yaml`] = fs.readFileSync(
			`${process.cwd()}/${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/val${i + 1}.${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/validator.yaml`,
			'utf-8',
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
				`${process.cwd()}/${NETWORK_SERVICE_NAME}.${NAMESPACE_NAME}.svc.cluster.local/publisher/sui_config/publisher.key`,
				'utf8',
			),
			...validatorsConfig,
		},
	};
	return await k8sApi.createNamespacedConfigMap({
		namespace: namespaceName,
		body: configMap,
	});
}

async function createNetworkServices(kc: KubeConfig, namespaceName: string) {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	await k8sApi.createNamespacedService({
		namespace: namespaceName,
		body: {
			metadata: {
				name: 'ika-dns-service',
			},
			spec: {
				clusterIP: 'None',
				ports: [
					{
						name: 'tx-interface',
						protocol: 'TCP',
						port: 8080,
						targetPort: 8080,
					},
					{
						name: 'p2p-sync',
						protocol: 'UDP',
						port: 8084,
						targetPort: 8084,
					},
					{
						name: 'metrics',
						protocol: 'TCP',
						port: 9184,
						targetPort: 9184,
					},
					{
						name: 'admin',
						protocol: 'TCP',
						port: 1337,
						targetPort: 1337,
					},
				],
				selector: {
					app: 'validator',
				},
				sessionAffinity: 'None',
				ipFamilies: ['IPv4'],
				ipFamilyPolicy: 'SingleStack',
				internalTrafficPolicy: 'Cluster',
			},
		},
	});
}

async function createPods(kc: KubeConfig, namespaceName: string, numOfValidators: number) {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	for (let i = 0; i < numOfValidators; i++) {
		const pod: V1Pod = {
			metadata: {
				name: `ika-val-${i + 1}`,
				namespace: namespaceName,
				labels: {
					app: 'validator',
				},
			},
			spec: {
				hostname: `val${i + 1}`,
				subdomain: 'ika-dns-service',
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
						image: 'ika:devnet-v0.0.6-arm64',
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
	const fullnodePod = {
		metadata: {
			name: `ika-fullnode`,
			namespace: namespaceName,
		},
		spec: {
			hostname: 'fullnode',
			subdomain: 'ika-dns-service',
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
					image: 'ika:devnet-v0.0.6-arm64',
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
		console.log(`Creating namespace: ${NAMESPACE_NAME}`);
		await createNamespace(kc, NAMESPACE_NAME);
		const configMap = await createConfigMap(kc, NAMESPACE_NAME, 4);
		console.log(`ConfigMap created: ${configMap}`);
		await createPods(kc, NAMESPACE_NAME, 4);
		await createNetworkServices(kc, NAMESPACE_NAME);
	});
});
