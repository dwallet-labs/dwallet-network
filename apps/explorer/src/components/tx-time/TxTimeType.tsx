// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useTimeAgo } from '@mysten/core';

type Prop = {
	timestamp: number | undefined;
};

export function TxTimeType({ timestamp }: Prop) {
	const timeAgo = useTimeAgo({
		timeFrom: timestamp || null,
		shortedTimeLabel: true,
	});

	return (
		<section>
			<div className="w-20 text-caption">{timeAgo}</div>
		</section>
	);
}
