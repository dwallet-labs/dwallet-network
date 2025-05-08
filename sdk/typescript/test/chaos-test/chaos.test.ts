import fs from 'fs';
import path from 'path';
import { CoreV1Api, KubeConfig, V1ConfigMap, V1Namespace } from '@kubernetes/client-node';
import { describe, it } from 'vitest';

import { CONFIG_MAP_NAME, NAMESPACE_NAME, NETWORK_SERVICE_NAME } from './globals';
import { createNetworkServices } from './network-service';
import { createPods } from './pods';

const createNamespace = async (kc: KubeConfig, namespaceName: string) => {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const namespaceBody: V1Namespace = {
		metadata: {
			name: namespaceName,
		},
	};
	await k8sApi.createNamespace({ body: namespaceBody });
};

export async function createConfigMap(
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

describe('run chain chaos testing', () => {
	it('create and deploy the config map', async () => {
		const kc = new KubeConfig();
		kc.loadFromDefault();
		await createNamespace(kc, NAMESPACE_NAME);
		await createConfigMap(kc, NAMESPACE_NAME, 4);
		await createPods(kc, NAMESPACE_NAME, 4);
		await createNetworkServices(kc, NAMESPACE_NAME);
	});
});
