// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type IkaWallet } from '_src/dapp-interface/WalletStandardInterface';
import { Transaction } from '@ika-io/ika/transactions';
import { getWallets, ReadonlyWalletAccount, type Wallet } from '@mysten/wallet-standard';
import { useEffect, useState } from 'react';
import ReactDOM from 'react-dom/client';

function getDemoTransaction(address: string) {
	const txb = new Transaction();
	const [coin] = txb.splitCoins(txb.gas, [1]);
	txb.transferObjects([coin], address);
	return txb;
}

function getAccount(account: ReadonlyWalletAccount, useWrongAccount: boolean) {
	if (useWrongAccount && account) {
		const newAccount = new ReadonlyWalletAccount({
			address: '0x00000001',
			chains: account.chains,
			features: account.features,
			publicKey: account.publicKey,
			icon: account.icon,
			label: account.label,
		});
		return newAccount;
	}
	return account;
}

function findIkaWallet(wallets: readonly Wallet[]) {
	return (wallets.find((aWallet) => aWallet.name.includes('Ika Wallet')) ||
		null) as IkaWallet | null;
}

function App() {
	const [ikaWallet, setIkaWallet] = useState<IkaWallet | null>(() =>
		findIkaWallet(getWallets().get()),
	);
	const [error, setError] = useState<string | null>(null);
	const [accounts, setAccounts] = useState<ReadonlyWalletAccount[]>(
		() => ikaWallet?.accounts || [],
	);
	const [useWrongAccounts, setUseWrongAccounts] = useState(false);

	useEffect(() => {
		const walletsApi = getWallets();
		function updateWallets() {
			setIkaWallet(findIkaWallet(walletsApi.get()));
		}
		const unregister1 = walletsApi.on('register', updateWallets);
		const unregister2 = walletsApi.on('unregister', updateWallets);
		return () => {
			unregister1();
			unregister2();
		};
	}, []);
	useEffect(() => {
		if (ikaWallet) {
			return ikaWallet.features['standard:events'].on('change', ({ accounts }) => {
				if (accounts) {
					setAccounts(ikaWallet.accounts);
				}
			});
		}
	}, [ikaWallet]);
	if (!ikaWallet) {
		return <h1>Ika Wallet not found</h1>;
	}
	return (
		<>
			<h1>Ika Wallet is installed. ({ikaWallet.name})</h1>
			{accounts.length ? (
				<ul data-testid="accounts-list">
					{accounts.map((anAccount) => (
						<li key={anAccount.address}>{anAccount.address}</li>
					))}
				</ul>
			) : (
				<button onClick={async () => ikaWallet.features['standard:connect'].connect()}>
					Connect
				</button>
			)}
			<label>
				<input
					type="checkbox"
					checked={useWrongAccounts}
					onChange={() => setUseWrongAccounts((v) => !v)}
				/>
				Use wrong account
			</label>
			<button
				onClick={async () => {
					setError(null);
					const txb = getDemoTransaction(accounts[0]?.address || '0x01');
					try {
						await ikaWallet.features[
							'ika:signAndExecuteTransactionBlock'
						]!.signAndExecuteTransactionBlock({
							transactionBlock: txb,
							account: getAccount(accounts[0], useWrongAccounts),
							chain: 'ika:unknown',
						});
					} catch (e) {
						setError((e as Error).message);
					}
				}}
			>
				Send transaction
			</button>
			<button
				onClick={async () => {
					setError(null);
					const txb = getDemoTransaction(accounts[0]?.address || '0x01');
					try {
						await ikaWallet.features['ika:signTransactionBlock']!.signTransactionBlock({
							transactionBlock: txb,
							account: getAccount(accounts[0], useWrongAccounts),
							chain: 'ika:unknown',
						});
					} catch (e) {
						setError((e as Error).message);
					}
				}}
			>
				Sign transaction
			</button>
			<button
				onClick={async () => {
					setError(null);
					try {
						await ikaWallet.features['ika:signMessage']?.signMessage({
							account: getAccount(accounts[0], useWrongAccounts),
							message: new TextEncoder().encode('Test message'),
						});
					} catch (e) {
						setError((e as Error).message);
					}
				}}
			>
				Sign message
			</button>
			{error ? (
				<div>
					<h6>Error</h6>
					<div>{error}</div>
				</div>
			) : null}
		</>
	);
}

ReactDOM.createRoot(document.getElementById('root')!).render(<App />);
