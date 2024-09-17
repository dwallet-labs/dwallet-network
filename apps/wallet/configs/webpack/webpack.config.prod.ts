// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { Configuration } from 'webpack';
import { merge } from 'webpack-merge';

import configCommon from './webpack.config.common';

const configProd: Configuration = {
	mode: 'production',
	devtool: 'source-map',
};

async function getConfig() {
	return merge(await configCommon(), configProd);
}

export default getConfig;
