// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { hello_wasm } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, it } from 'vitest';

import {
	launchDKGSecondRound,
	startFirstDKGSession,
	launchProofMPSession,
} from '../../src/signature-mpc/proof';
import { setup, TestToolbox } from './utils/setup';

function numberArrayToHexString(numbers: number[]): string {
	return numbers
		.map((num) => num.toString(16).padStart(2, '0')) // Convert each number to hex and pad if needed
		.join(''); // Join the hex values into a single string
}

function hexStringToNumberArray(hexString: string): number[] {
	// Ensure the string length is even
	if (hexString.length % 2 !== 0) {
		throw new Error('Invalid hex string. Must have an even length.');
	}

	const result: number[] = [];
	for (let i = 0; i < hexString.length; i += 2) {
		// Take each pair of hex characters and convert them to a number
		const hexPair = hexString.substring(i, i + 2);
		result.push(parseInt(hexPair, 16));
	}

	return result;
}

describe('Test signature mpc', () => {
	let toolbox: TestToolbox;

	beforeAll(async () => {
		toolbox = await setup();
	});

	// it('should create proof MPC Event', async () => {
	// 	await launchProofMPSession(toolbox.keypair, toolbox.client);
	// 	console.log(toolbox.keypair.toPeraAddress());
	// });

	// it('should create dkg MPC Event', async () => {
	// 	// been generated manually
	// 	const user_second_dkg_round_input =
	// 		'IQK00DylC7XlNiIGlgVn3L+6c5Dluy1OQM+0FuLs99Y9HiEDWV2SnvEnjxCgnFMd8mqltVshy00Xs1zQ3ZgSYEZKTl4hAoZap9uY+b+iVQNbA5fOYr+fgBoGfiWNXbbYQGaWgbtYIQKlyz8LEwW5D5q3Rei1JpGtsplNGLVb5X6URAk0QDi2uSECtq5C1qeKQmUGz/OEysrqqe/bvko7sbCT9lNLMEt8ZfshA/nax+kmBtLXoWMXpi3rgnzeRRJdMrHDF/e8AmnPA9xjIQOlpF74CE2oOTRpLdGlekwBHS0Uxq0RzntLBouAffYtUiEDhU5G6/vBA6D4aUUHIInGmt1Gk8Sqe/P2tDYrlNBP3TohAw1SPoRe5+t+Y86Ux47MiXScIyZ1dl2Qz3tzPmOCUPamIQPpoLZ35ZrUAbvzSXa3cNx3XBWU3gW+RevZDhzEbxEmHyEDyINqkQ6h9dDKnx63dzXLXmZfTw6+GMz/+RdUboFFxNchAora6QWdhKVubPDiZt0e0VP5a3BmtrIZ4eVKam98OksZIQNHRTye+XDV09xCZ149x3hsjq3IcLH2OA1Fh+0XlUkuIyEDQ8Fd43HMZ20V1B1zKy0nhnDYyTMhBibdRCrjQAgZBKEhA4lfcBAMg8wsXrisZhoD5OFHBH2KM2ByOF4b8R4ceyNmIQJPZ/qkLya2hIeb6EJfVmo/kp+qY5JsB84eFqDP2491QqBxeJVYUpglYbwZf70Qoc4H+wkZNmaQnP3cO5EYWNBNc1ErMQHMcX2yevYRYivdqtNlhOc5ujQZq1qVgvTuk8RJHDiWoruBj+nJbw/P+scGoycXJibDNSlGwc7an/hyjK5iTit4d33wMGnDd8fAbgl9NWfsFY9CUZMM+15edEARrKfK7i7zgdgOSGwZGDX2An80HA6q63Iz1dI2d7vo/TLSpGTdQD8JOSykWCny5trDdQGlXCCl1c8so+ZyFJ+SCqL09r97lewWSisS/0+9qaMkwBDwlL+tVic3Wd1dt9HG5JJpNKkToC1DMs0/zJhkf+JeULt5LqNSzqSGVqws15g9Tl7aH7+mP7MOjgU/iqUjChyXdePGtmiloM/GCa6NNT1+wO1APQuClj3qGAk0T+XKZssVK5PdyTziP5+lRV5uXT2EJob5sblfeW4moD2TzrQ8aimnoCUESiVqkm8XjMGCMC0wkYZuLOFQdHCbwPu0vZyzqQrPryH5XL+NY4t7G+8U8jwQjQpMpqxIJZmZNcAfU91EfKKbyMn14y9D81VhJgpOf3QqcpBAZM9cZ3LRjfTb0uZeu0zkIJ0XMMkmPvl+3BuyOIWwoHvchvrvTXjQ6Ib/LXpn9xMfkGzNjvJZNgtrjpAkCABEi7T5KEpPYtuC3q+tqXubW88KqRnRAEByOAEAAAAAAAAAAAAAAAAAAE0AAAAAAAAAAAAAAAAAAAAdAgAAAAAAAAAAAAAAAAAABwAAAAAAAAAAAAAAAAAAAC8BAAAAAAAAAAAAAAAAAAD2AwAAAAAAAAAAAAAAAAAALwAAAAAAAAAAAAAAAAAAAAwAAAAAAAAAAAAAAAAAAADtAQAAAAAAAAAAAAAAAAAAbQAAAAAAAAAAAAAAAAAAAJEEAAAAAAAAAAAAAAAAAABmAAAAAAAAAAAAAAAAAAAANwEAAAAAAAAAAAAAAAAAAPkAAAAAAAAAAAAAAAAAAAChAQAAAAAAAAAAAAAAAAAAMwEAAAAAAAAAAAAAAAAAACECQDh9aOUKzi4COApvTWY2MjMGuH3E8Jl5woYBp83wwac=';
	//
	// });

	it('should create dkg MPC Event', async () => {
		console.log(toolbox.keypair.toPeraAddress());
		const firstDKGOutput = await startFirstDKGSession(toolbox.keypair, toolbox.client);
		const a = numberArrayToHexString(firstDKGOutput!);
		// const a =
		// 	'001df78f3b5736463644764f313d47b3c0733cb930f710346db706bcd254fc8f08cf84d4723fc4cd26966a3bcbab2c576aefbbca0348b01e54a9b4988fde14c43517d0cba537131cc097bad5fdd1d86796e9a9c144b4c89e9c705a523c25a9cf830e4e6fb83e08ed77c18024ab2660eb6d8888e0d00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008bcfa77de688a288bbc23439d153759de5e3d4416d1fbf088e615bbc4075597b4b5e433bc393ceadd39126d90da86f04a914d1abe98fe0479b9322cedf84852092cfccc82df3bdc67cdb26139a45a3fedcdc8b54c6d2c43e6c86b4bf2ec16b63fb3db2f9f99b374616f50895deea4e62c68026a700000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000037237c2d9edbfdeae60ec990c5afffae8239568839daab863dbab5bb64da077b74a7f3140ba1b97510246442ac3a46a2bbc09462c2ce7ba6345a07f0884bc0fb9e90ffa418f2e00480af64375ac4b24ef09d8c848c7f0641af61d0b2aafe219ac41a9b5ae4e670bbb50949c98aedcb7d660980530100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000005f8eb4ba8e6b747bb7180363a91ffd661ffeaae5969868d06198ad7d34d348d5dce52a6035dfc0de055178b0781a4e7de7686a48898851977228465fb9679961111d90b4e98d391d6db41ae0c12b02be2d81bab71ba18364a88d9a121acd589c372d5b6de6a86b4d45ec6c7b87b23e7048be708f00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002103d6bc7b6e845778b4cc902f4b70d09d7a3c79b9a1e05c8f04679535b9ffb64d5f';
		console.log(a);
		// const b = hexStringToNumberArray(a);
		// let publicKeyShareAndProof = hello_wasm(Uint8Array.from(b));
		// console.log(publicKeyShareAndProof);
		// const c =
		// 	'2103321bcffc09b5ff5551c980cb6c395b1fd4e53400c261ccc128039cda5b2d093e21022c1b0902ad3675486f0433b14f4a40f25dea6abf26a5def9596ccac6113374a6210361e49a7ed62e0fc5ff75c7d8701ab812928ba6b87bb0217ef42581980ae5bf242102b704dabe11de10387179150c47a523a5b814389029a76fa0f31c66ed560636b82102a5ecdc14c43360372e8996adf121cc37cca2472ea80c90a0cb8fc9d1843c1f3b21039ef990aedb8ec78a68662ece816b580824d5f68b35df1473c7fb4427b70835232103ad0ec044b655175fe4e0cba81c45852961b4e0b9b1ab6bac2de8bdb65ec3f82d2103028f9946fcee0ffaefd2d0d7e950b8e022b669f804a77cf622b7c7edb7bd2bf8210293b6d2271c4cc7899a7f638d11cf2671463e681ec61c70a4c2653b72037fc15821032363c2b1d91b24adb7d76242ee79317812a6b0c021a5a7c4dd7c59231a63cf5e210327e4c227016476eb9ad8fd6820deac4897a867fb08ee295f43a04e9817784fcb210226f77422df07372b6a4c488dfe199902316646554a69fdb18eefd549d9fd5a8821020cc395b26cae3cd46add52a19f9d912f22a77f35df31f54d7515760909096a9321036d8722b0997bff66504b7566b8cdf95d459df56c0ff0a1facf7990fc6361e3342102210cca8c0f47e6e77904d8086b2eb4e78279edfddc59651229c04366936e03c82102a9564157333cb03358acc313c8c51111404a871d05dc3861c8cb7892f955011611154f9b00829a13cd689d1785008550abbb76e4b70ad684bb5072d572227fa235b9fe64050e2316998a35fa55a65450be834bfc760e948c7d30ba68598bae1d79eca7df2e4f5a45e72ad65d3d512c675e9aa982cb757950b0a1524cebfacb26571fa7a00ec41dc96788b20c84ef523bbbd17e3d0720446486c75d0b5d7a21723e45feac49673295ebdd541df7df76fcfac605912884831e53b4f35e2bb1431630256552dfd15be002dcdc87388c5f01a1f8a5aba402e79cd7e7a681dd7b127d8db5152b8bc749cf0524d207fe8a6210100952228a32dc19142b15f8836a0a6628434435c9229021517b5f0a10edeed798349d91b90e53e77c40cd149b6055f54931391f56c98f053299d1db47e29cb5ea706d7ca0a7e88cff09c6efd838b96d3061c7172cfdffc3389b279895bd1aad1ff993c9172926dcf411e040602bee4653700ca140de475c487a81fa7b3806e0b95563d8e3db46cf66b0a6d10b67300422932763b1c070d23c124f22d0aa6ed8c77f9d2f79a3726af1b13f45c3602e87e09937ce91dd14f1ce8c3d9d12413e26e4016629d0e08797636ff1ed0b5ae9289b7127c33bbb1098fb5286e917a85af814d52c4481c24c2dda90f30ecb8b3d8e2c7073a3c4791620d288801e239d5bf5c93c98b7dc5167a0c0dc1fb0cf0ecb0c81c0bf32576479039e69409a8fe58f1fff82ff9cc2848e2af396423a1d059ea00f000000000000000000000000000000430000000000000000000000000000008300000000000000000000000000000048000000000000000000000000000000500000000000000000000000000000004500000000000000000000000000000050010000000000000000000000000000c702000000000000000000000000000023000000000000000000000000000000e5010000000000000000000000000000c0010000000000000000000000000000500100000000000000000000000000000e000000000000000000000000000000b4000000000000000000000000000000950100000000000000000000000000001c0000000000000000000000000000002102a03b96fe1fe53966c08650cd833dbe796b243b798c29176671d0fe0e69dd3679';
		// // console.log(publicKeyShareAndProof);
		// let d = hexStringToNumberArray(c);
		// await launchDKGSecondRound(
		// 	toolbox.keypair,
		// 	toolbox.client,
		// 	Uint8Array.from(d),
		// 	Uint8Array.from(b),
		// );
	});
});
