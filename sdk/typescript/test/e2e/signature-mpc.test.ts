// Copyright (c) dWallet Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { hello_wasm } from '@dwallet-network/signature-mpc-wasm';
import { beforeAll, describe, it } from 'vitest';

import {
	launchDKGSecondRound,
	launchDKGSession,
	launchProofMPSession,
} from '../../src/signature-mpc/proof';
import { setup, TestToolbox } from './utils/setup';

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
		const firstDKGOutput = await launchDKGSession(toolbox.keypair, toolbox.client);
		let publicKeyShareAndProof = hello_wasm(firstDKGOutput);
		await launchDKGSecondRound(
			toolbox.keypair,
			toolbox.client,
			publicKeyShareAndProof,
			firstDKGOutput!,
		);
	});
});
