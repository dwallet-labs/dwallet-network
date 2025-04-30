import fs from 'fs';
import { CoreV1Api, KubeConfig, V1ConfigMap } from '@kubernetes/client-node';
import { beforeEach, describe, it } from 'vitest';

const namespaceName = 'test';

const createConfigMap = async () => {
	const kc = new KubeConfig();
	kc.loadFromDefault();
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const clientIdentifier = 'my-subdomain';
	const yourYamlString = fs.readFileSync(
		'./beta50.devnet.ika-network.net/publisher/fullnode.yaml',
		'utf8',
	);
	const configMap: V1ConfigMap = {
		metadata: {
			namespace: namespaceName,
			name: 'ika-chaos-test-config',
		},
		data: {
			'fullnode.yaml': yourYamlString,
		},
	};
	await k8sApi.createNamespacedConfigMap({
		namespace: namespaceName,
		body: configMap,
	});
};

describe('run chain chaos testing', () => {
	it('create and deploy the config map', async () => {
		await createConfigMap();
	});
});
