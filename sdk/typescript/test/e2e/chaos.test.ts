import {Config, delay, getNetworkDecryptionKeyPublicOutput} from "../../src/dwallet-mpc/globals";
import { beforeEach, describe, it } from 'vitest';
import {KubeConfig, CoreV1Api} from '@kubernetes/client-node';

const createConfigMap = async () => {
    const kc = new KubeConfig();
    kc.loadFromDefault();
    const k8sApi = kc.makeApiClient(CoreV1Api);
    const clientIdentifier = 'my-subdomain';
    await k8sApi.createNamespacedConfigMap();
}

describe('run chain chaos testing', () => {
    it('deploy the network and run an e2e test', async () => {
        const networkDecryptionKeyPublicOutput = await getNetworkDecryptionKeyPublicOutput(conf);
        console.log(`networkDecryptionKeyPublicOutput: ${networkDecryptionKeyPublicOutput}`);
    });
};