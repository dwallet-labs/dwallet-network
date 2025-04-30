import fs from 'fs';
import { CoreV1Api, KubeConfig, V1ConfigMap, V1Namespace } from '@kubernetes/client-node';
import { beforeEach, describe, it } from 'vitest';

const namespaceName = 'testush';

const createConfigMap = async () => {
	const kc = new KubeConfig();
	kc.loadFromDefault();
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const namespaceBody: V1Namespace = {
		metadata: {
			name: namespaceName,
		},
	};
	await k8sApi.createNamespace({ body: namespaceBody }).catch((err) => {
		if (err.response?.statusCode !== 409) throw err;
	});

	const clientIdentifier = 'my-subdomain';
	const yourYamlString = fs.readFileSync(
		'/Users/itaylevy/code/dwallet-network/sdk/typescript/test/e2e/beta50.devnet.ika-network.net/publisher/fullnode.yaml',
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
