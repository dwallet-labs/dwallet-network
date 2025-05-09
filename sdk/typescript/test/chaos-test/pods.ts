import type { KubeConfig, V1Pod } from '@kubernetes/client-node';
import { CoreV1Api } from '@kubernetes/client-node';
import { NETWORK_SERVICE_NAME } from './globals';

export async function createPods(kc: KubeConfig, namespaceName: string, numOfValidators: number) {
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
				subdomain: NETWORK_SERVICE_NAME,
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
						image: process.env.DOCKER_TAG,
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
			subdomain: NETWORK_SERVICE_NAME,
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
					image: process.env.DOCKER_TAG,
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
