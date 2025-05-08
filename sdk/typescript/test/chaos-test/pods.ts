import type { KubeConfig, V1Pod } from '@kubernetes/client-node';
import { CoreV1Api } from '@kubernetes/client-node';

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
