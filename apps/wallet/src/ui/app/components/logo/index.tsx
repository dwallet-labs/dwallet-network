// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { API_ENV } from '_src/shared/api-env';
import { IkaCustomRpc, IkaDevnet, IkaLocal, IkaMainnet, IkaTestnet } from '@mysten/icons';

type LogoProps = {
	networkName?: API_ENV;
};

const networkLogos = {
	[API_ENV.mainnet]: IkaMainnet,
	[API_ENV.devNet]: IkaDevnet,
	[API_ENV.testNet]: IkaTestnet,
	[API_ENV.local]: IkaLocal,
	[API_ENV.customRPC]: IkaCustomRpc,
};

const Logo = ({ networkName }: LogoProps) => {
	const LogoComponent = networkName ? networkLogos[networkName] : networkLogos[API_ENV.mainnet];

	return <LogoComponent className="h-7 w-walletLogo text-gray-90" />;
};

export default Logo;
