// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { IkaClient } from '@ika-io/ika/client';
import { screen } from '@testing-library/dom';
import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { useState } from 'react';

import { IkaClientProvider } from '../../src/components/IkaClientProvider.js';
import { useIkaClient, useIkaClientContext } from '../../src/index.js';

describe('IkaClientProvider', () => {
	it('renders without crashing', () => {
		render(
			<IkaClientProvider>
				<div>Test</div>
			</IkaClientProvider>,
		);
		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	it('provides a IkaClient instance to its children', () => {
		const ChildComponent = () => {
			const client = useIkaClient();
			expect(client).toBeInstanceOf(IkaClient);
			return <div>Test</div>;
		};

		render(
			<IkaClientProvider>
				<ChildComponent />
			</IkaClientProvider>,
		);
	});

	it('can accept pre-configured IkaClients', () => {
		const ikaClient = new IkaClient({ url: 'http://localhost:8080' });
		const ChildComponent = () => {
			const client = useIkaClient();
			expect(client).toBeInstanceOf(IkaClient);
			expect(client).toBe(ikaClient);
			return <div>Test</div>;
		};

		render(
			<IkaClientProvider networks={{ localnet: ikaClient }}>
				<ChildComponent />
			</IkaClientProvider>,
		);

		expect(screen.getByText('Test')).toBeInTheDocument();
	});

	test('can create ika clients with custom options', async () => {
		function NetworkSelector() {
			const ctx = useIkaClientContext();

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
				<IkaClientProvider
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
						return new IkaClient(config);
					}}
				>
					<div>{`selected network: ${selectedNetwork}`}</div>
					<NetworkSelector />
				</IkaClientProvider>
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
			const ctx = useIkaClientContext();

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
				<IkaClientProvider
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
				</IkaClientProvider>
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
			const ctx = useIkaClientContext();

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
				<IkaClientProvider
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
				</IkaClientProvider>
			);
		}

		const user = userEvent.setup();

		render(<ControlledProvider />);

		expect(screen.getByText('selected network: a')).toBeInTheDocument();

		await user.click(screen.getByText('select b'));

		expect(screen.getByText('selected network: b')).toBeInTheDocument();
	});
});
