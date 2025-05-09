import { CoreV1Api, KubeConfig, V1Namespace } from '@kubernetes/client-node';
import { beforeAll, describe, it } from 'vitest';

import { createConfigMap } from './config-map';
import { NAMESPACE_NAME } from './globals';
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

describe('chaos tests', () => {
	beforeAll(() => {
		require('dotenv').config();
	});

	it('deploy the ika network from the current directory to the local kubernetes cluster', async () => {
		const kc = new KubeConfig();
		kc.loadFromDefault();
		await createNamespace(kc, NAMESPACE_NAME);
		await createConfigMap(kc, NAMESPACE_NAME, Number(process.env.VALIDATOR_NUM));
		await createPods(kc, NAMESPACE_NAME, Number(process.env.VALIDATOR_NUM));
		await createNetworkServices(kc, NAMESPACE_NAME);
	});
});
