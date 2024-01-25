// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { StatsWrapper } from './FormattedStatsAmount';
import { useGetNetworkMetrics } from '~/hooks/useGetNetworkMetrics';

export function Checkpoint() {
	const { data, isPending } = useGetNetworkMetrics();

	return (
		<StatsWrapper
			label="Checkpoint"
			tooltip="The current checkpoint"
			unavailable={isPending}
			size="sm"
			orientation="horizontal"
		>
			{data?.currentCheckpoint ? BigInt(data?.currentCheckpoint).toLocaleString() : null}
		</StatsWrapper>
	);
}
