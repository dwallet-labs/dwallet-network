// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { growthbook } from '_src/ui/app/experimentation/feature-gating';
import * as Sentry from '@sentry/react';

import { getSentryConfig } from '../../../shared/sentry-config';

export default function initSentry() {
	Sentry.init(
		getSentryConfig({
			integrations: [new Sentry.BrowserTracing()],
			tracesSampler: () => {
				return growthbook.getFeatureValue('wallet-sentry-tracing', 0);
			},
		}),
	);
}
