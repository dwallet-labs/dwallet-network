// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { PeraClient } from '@pera-io/pera/client';
import { screen } from '@testing-library/dom';
import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useState } from 'react';

import { PeraClientProvider } from '../../src/components/PeraClientProvider.js';
import { usePeraClient, usePeraClientContext } from '../../src/index.js';

describe('PeraClientProvider', () => {
	it('renders without crashing', () => {
		render(
			<PeraClientProvider>
				<div>Test</div>
			</PeraClientProvider>,
		);
		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	it('provides a PeraClient instance to its children', () => {
		const ChildComponent = () => {
			const client = usePeraClient();
			expect(client).toBeInstanceOf(PeraClient);
			return <div>Test</div>;
		};

		render(
			<PeraClientProvider>
				<ChildComponent />
			</PeraClientProvider>,
		);
	});

	it('can accept pre-configured PeraClients', () => {
		const peraClient = new PeraClient({ url: 'http://localhost:8080' });
		const ChildComponent = () => {
			const client = usePeraClient();
			expect(client).toBeInstanceOf(PeraClient);
			expect(client).toBe(peraClient);
			return <div>Test</div>;
		};

		render(
			<PeraClientProvider networks={{ localnet: peraClient }}>
				<ChildComponent />
			</PeraClientProvider>,
		);

		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	test('can create pera clients with custom options', async () => {
		function NetworkSelector() {
			const ctx = usePeraClientContext();

			return (
				<div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => ctx.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}
		function CustomConfigProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<string>();

			return (
				<PeraClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					createClient={(name, { custom, ...config }) => {
						custom(name);
						return new PeraClient(config);
					}}
				>
					<div>{`selected network: ${selectedNetwork}`}</div>
					<NetworkSelector />
				</PeraClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<CustomConfigProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});

	test('controlled mode', async () => {
		function NetworkSelector(props: { selectNetwork: (network: string) => void }) {
			const ctx = usePeraClientContext();

			return (
				<div>
					<div>{`selected network: ${ctx.network}`}</div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => props.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}

		function ControlledProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<'a' | 'b'>('a');

			return (
				<PeraClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					network={selectedNetwork}
				>
					<NetworkSelector
						selectNetwork={(network) => {
							setSelectedNetwork(network as 'a' | 'b');
						}}
					/>
				</PeraClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<ControlledProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});

	test('onNetworkChange', async () => {
		function NetworkSelector() {
			const ctx = usePeraClientContext();

			return (
				<div>
					<div>{`selected network: ${ctx.network}`}</div>
					{Object.keys(ctx.networks).map((network) => (
						<button key={network} onClick={() => ctx.selectNetwork(network)}>
							{`select ${network}`}
						</button>
					))}
				</div>
			);
		}

		function ControlledProvider() {
			const [selectedNetwork, setSelectedNetwork] = useState<string>('a');

			return (
				<PeraClientProvider
					networks={{
						a: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
						b: {
							url: 'http://localhost:8080',
							custom: setSelectedNetwork,
						},
					}}
					network={selectedNetwork as 'a' | 'b'}
					onNetworkChange={(network) => {
						setSelectedNetwork(network);
					}}
				>
					<NetworkSelector />
				</PeraClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<ControlledProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});
});
