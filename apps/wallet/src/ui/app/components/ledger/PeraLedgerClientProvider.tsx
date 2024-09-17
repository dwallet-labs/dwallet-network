// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import TransportWebHID from '@ledgerhq/hw-transport-webhid';
import TransportWebUSB from '@ledgerhq/hw-transport-webusb';
import PeraLedgerClient from '@mysten/ledgerjs-hw-app-pera';
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';

import {
	convertErrorToLedgerConnectionFailedError,
	LedgerDeviceNotFoundError,
	LedgerNoTransportMechanismError,
} from './ledgerErrors';

type PeraLedgerClientProviderProps = {
	children: React.ReactNode;
};

type PeraLedgerClientContextValue = {
	peraLedgerClient: PeraLedgerClient | undefined;
	connectToLedger: (requestPermissionsFirst?: boolean) => Promise<PeraLedgerClient>;
};

const PeraLedgerClientContext = createContext<PeraLedgerClientContextValue | undefined>(undefined);

export function PeraLedgerClientProvider({ children }: PeraLedgerClientProviderProps) {
	const [peraLedgerClient, setPeraLedgerClient] = useState<PeraLedgerClient>();
	const resetPeraLedgerClient = useCallback(async () => {
		await peraLedgerClient?.transport.close();
		setPeraLedgerClient(undefined);
	}, [peraLedgerClient]);

	useEffect(() => {
		// NOTE: The disconnect event is fired when someone physically disconnects
		// their Ledger device in addition to when user's exit out of an application
		peraLedgerClient?.transport.on('disconnect', resetPeraLedgerClient);
		return () => {
			peraLedgerClient?.transport.off('disconnect', resetPeraLedgerClient);
		};
	}, [resetPeraLedgerClient, peraLedgerClient?.transport]);

	const connectToLedger = useCallback(
		async (requestPermissionsFirst = false) => {
			// If we've already connected to a Ledger device, we need
			// to close the connection before we try to re-connect
			await resetPeraLedgerClient();

			const ledgerTransport = requestPermissionsFirst
				? await requestLedgerConnection()
				: await openLedgerConnection();
			const ledgerClient = new PeraLedgerClient(ledgerTransport);
			setPeraLedgerClient(ledgerClient);
			return ledgerClient;
		},
		[resetPeraLedgerClient],
	);
	const contextValue: PeraLedgerClientContextValue = useMemo(() => {
		return {
			peraLedgerClient,
			connectToLedger,
		};
	}, [connectToLedger, peraLedgerClient]);

	return (
		<PeraLedgerClientContext.Provider value={contextValue}>
			{children}
		</PeraLedgerClientContext.Provider>
	);
}

export function usePeraLedgerClient() {
	const peraLedgerClientContext = useContext(PeraLedgerClientContext);
	if (!peraLedgerClientContext) {
		throw new Error('usePeraLedgerClient must be used within PeraLedgerClientContext');
	}
	return peraLedgerClientContext;
}

async function requestLedgerConnection() {
	const ledgerTransportClass = await getLedgerTransportClass();
	try {
		return await ledgerTransportClass.request();
	} catch (error) {
		throw convertErrorToLedgerConnectionFailedError(error);
	}
}

async function openLedgerConnection() {
	const ledgerTransportClass = await getLedgerTransportClass();
	let ledgerTransport: TransportWebHID | TransportWebUSB | null | undefined;

	try {
		ledgerTransport = await ledgerTransportClass.openConnected();
	} catch (error) {
		throw convertErrorToLedgerConnectionFailedError(error);
	}
	if (!ledgerTransport) {
		throw new LedgerDeviceNotFoundError(
			"The user doesn't have a Ledger device connected to their machine",
		);
	}
	return ledgerTransport;
}

async function getLedgerTransportClass() {
	if (await TransportWebHID.isSupported()) {
		return TransportWebHID;
	} else if (await TransportWebUSB.isSupported()) {
		return TransportWebUSB;
	}
	throw new LedgerNoTransportMechanismError(
		"There are no supported transport mechanisms to connect to the user's Ledger device",
	);
}
