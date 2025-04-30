import fs from 'fs';
import { CoreV1Api, KubeConfig, V1ConfigMap, V1Namespace } from '@kubernetes/client-node';
import { describe, it } from 'vitest';
import Handlebars from 'handlebars';

const createConfigMap = async (kc: KubeConfig, namespaceName: string, numOfValidators: number) => {
	const k8sApi = kc.makeApiClient(CoreV1Api);
	const namespaceBody: V1Namespace = {
		metadata: {
			name: namespaceName,
		},
	};
	await k8sApi.createNamespace({ body: namespaceBody }).catch((err) => {
		if (err.response?.statusCode !== 409) throw err;
	});
	const yourYamlString = fs.readFileSync(
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
			name: 'ika-chaos-test-config',
		},
		data: {
			'fullnode.yaml': yourYamlString,
			...validatorsConfig,
		},
	};
	await k8sApi.createNamespacedConfigMap({
		namespace: namespaceName,
		body: configMap,
	});
};

async function createNetworkServices() {

}

describe('run chain chaos testing', () => {
	it('create and deploy the config map', async () => {
		const kc = new KubeConfig();
		kc.loadFromDefault();
		const namespaceName = generateUniqueNamespace();
		console.log(`Creating namespace: ${namespaceName}`);
		await createConfigMap(kc, namespaceName, 4);
	});
});

function generateUniqueNamespace(prefix = 'chaos-test'): string {
	const timestamp = Date.now().toString();
	return `${prefix}-${timestamp}`;
}
