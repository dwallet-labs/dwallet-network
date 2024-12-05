// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import TransportWebHID from '@ledgerhq/hw-transport-webhid';
import TransportWebUSB from '@ledgerhq/hw-transport-webusb';
import IkaLedgerClient from '@mysten/ledgerjs-hw-app-ika';
import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';

import {
	convertErrorToLedgerConnectionFailedError,
	LedgerDeviceNotFoundError,
	LedgerNoTransportMechanismError,
} from './ledgerErrors';

type IkaLedgerClientProviderProps = {
	children: React.ReactNode;
};

type IkaLedgerClientContextValue = {
	ikaLedgerClient: IkaLedgerClient | undefined;
	connectToLedger: (requestPermissionsFirst?: boolean) => Promise<IkaLedgerClient>;
};

const IkaLedgerClientContext = createContext<IkaLedgerClientContextValue | undefined>(undefined);

export function IkaLedgerClientProvider({ children }: IkaLedgerClientProviderProps) {
	const [ikaLedgerClient, setIkaLedgerClient] = useState<IkaLedgerClient>();
	const resetIkaLedgerClient = useCallback(async () => {
		await ikaLedgerClient?.transport.close();
		setIkaLedgerClient(undefined);
	}, [ikaLedgerClient]);

	useEffect(() => {
		// NOTE: The disconnect event is fired when someone physically disconnects
		// their Ledger device in addition to when user's exit out of an application
		ikaLedgerClient?.transport.on('disconnect', resetIkaLedgerClient);
		return () => {
			ikaLedgerClient?.transport.off('disconnect', resetIkaLedgerClient);
		};
	}, [resetIkaLedgerClient, ikaLedgerClient?.transport]);

	const connectToLedger = useCallback(
		async (requestPermissionsFirst = false) => {
			// If we've already connected to a Ledger device, we need
			// to close the connection before we try to re-connect
			await resetIkaLedgerClient();

			const ledgerTransport = requestPermissionsFirst
				? await requestLedgerConnection()
				: await openLedgerConnection();
			const ledgerClient = new IkaLedgerClient(ledgerTransport);
			setIkaLedgerClient(ledgerClient);
			return ledgerClient;
		},
		[resetIkaLedgerClient],
	);
	const contextValue: IkaLedgerClientContextValue = useMemo(() => {
		return {
			ikaLedgerClient,
			connectToLedger,
		};
	}, [connectToLedger, ikaLedgerClient]);

	return (
		<IkaLedgerClientContext.Provider value={contextValue}>
			{children}
		</IkaLedgerClientContext.Provider>
	);
}

export function useIkaLedgerClient() {
	const ikaLedgerClientContext = useContext(IkaLedgerClientContext);
	if (!ikaLedgerClientContext) {
		throw new Error('useIkaLedgerClient must be used within IkaLedgerClientContext');
	}
	return ikaLedgerClientContext;
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
