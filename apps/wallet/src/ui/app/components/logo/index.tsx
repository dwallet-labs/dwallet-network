// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { API_ENV } from '_src/shared/api-env';
import { PeraCustomRpc, PeraDevnet, PeraLocal, PeraMainnet, PeraTestnet } from '@mysten/icons';

type LogoProps = {
	networkName?: API_ENV;
};

const networkLogos = {
	[API_ENV.mainnet]: PeraMainnet,
	[API_ENV.devNet]: PeraDevnet,
	[API_ENV.testNet]: PeraTestnet,
	[API_ENV.local]: PeraLocal,
	[API_ENV.customRPC]: PeraCustomRpc,
};

const Logo = ({ networkName }: LogoProps) => {
	const LogoComponent = networkName ? networkLogos[networkName] : networkLogos[API_ENV.mainnet];

	return <LogoComponent className="h-7 w-walletLogo text-gray-90" />;
};

export default Logo;
