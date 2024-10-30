// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { hello_wasm } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, it } from 'vitest';

import {
	launchDKGSecondRound,
	launchProofMPSession,
	startFirstDKGSession,
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
		// const firstDKGOutput = await startFirstDKGSession(toolbox.keypair, toolbox.client);
		// let a = numberArrayToHexString(firstDKGOutput?.output!);
		// console.log(a);

		let firstRoundOutputHex =
			'00259094f07a67106b0eda9e76cf398646344667c733e3e389c70fd7b6e13a8de979a6451ddf2a19d104b190addb3213cda35268e97e8c80762d23e660b100506e0e227d847d4aca983f556c524557c5977eb042852a8f43a3b046ae1aaa671ad81bb553f6267a56122ad59d14e464651f961d34ed000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001053f0849b87830522abba7b216db33e7f2d1a6c67f8f19dd3ca56177dad22e96b992cbfed9d4ddb3f63a89fe6fa791601e0901edc12c77b4f874154c34762e84bf2d15878c53b8facbf832aee00e1afbdbdfe7821dbcd136a8542f67fe9361617acc9adc8017be087fae985dfae4b007b44ed9380000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006df33403d2a45cb57ecf5c8fc2b9f7a34bd3ab99a055939148c4731374937e180e2d2a0f8c69bff7bf05abb2978514e457be4986a097149e60e7f4cac963b420cb51b1a16e21a860c6edbb289025302ebabbce9f3a63f1c31ef68c0fc6a38f6019cec61516ae49b5498c12206924d1a987bea334010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001eb55e4e4fb111867627afef21c1eae3df026aa616478715f5463452cf00b7d778a85808765b810b82c5844a5c6f863847aa7c7561b6ad5a8b489d4e086a70a9e5d599247389530fc259109b96f8cb5c9b2cba9a82a61125afa042f377c5bdbdb271a7a10ec349610ac58fe5752713e47d585781f010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000021036f5258f30bb1212e4b8784a143dc133f7a48e0f496ea7fbc9fe23d65e530126b';
		let firstRoundOutput = hexStringToNumberArray(firstRoundOutputHex);
		let sessionId = '0x2bf4ff23c519de44874533a155895633e99488073c02d3d38da97848ab2aed3c';
		let dwalletCapId = '0xb0513bc2aa7fee08a72a10cbd6a66c4fa91aba132621bc3e21861129f11a9128';

		let publicKeyShareAndProof = hello_wasm(Uint8Array.from(firstRoundOutput), sessionId.slice(2)!);
		console.log(publicKeyShareAndProof);
		let b = new Uint8Array(firstRoundOutput);
		await launchDKGSecondRound(
			toolbox.keypair,
			toolbox.client,
			publicKeyShareAndProof,
			firstRoundOutput,
			dwalletCapId,
			sessionId,
		);
	});
});
