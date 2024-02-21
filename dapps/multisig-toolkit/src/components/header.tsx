// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { KeyRound } from 'lucide-react';
import { NavLink } from 'react-router-dom';

const links = [
	{ to: '/offline-signer', label: 'Offline Signer' },
	{ to: '/signature-analyzer', label: 'Signature Analyzer' },
	{ to: '/multisig-address', label: 'MultiSig Address' },
	{ to: '/combine-signatures', label: 'Combine MultiSig Signatures' },
	{ to: '/execute-transaction', label: 'Execute Transaction' },
];

export function Header() {
	return (
		<div className="border-b px-8 py-4 flex items-center justify-between">
			<div className="flex items-center gap-2">
				<KeyRound strokeWidth={2} size={18} className="text-primary/80" />
				<h1 className="font-bold text-lg bg-clip-text text-transparent bg-gradient-to-r from-primary to-primary/60">
					Sui MultiSig Toolkit
				</h1>
			</div>

			<div className="flex gap-4">
				{links.map(({ to, label }) => (
					<NavLink
						key={to}
						to={to}
						className={({ isActive }) =>
							isActive
								? 'text-sm font-semibold transition-colors hover:text-primary'
								: 'text-sm font-semibold text-muted-foreground transition-colors hover:text-primary'
						}
					>
						{label}
					</NavLink>
				))}
			</div>
		</div>
	);
}
