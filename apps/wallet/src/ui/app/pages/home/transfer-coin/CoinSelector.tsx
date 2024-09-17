// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ActiveCoinsCard } from '_components/active-coins-card';
import Overlay from '_components/overlay';
import { useUnlockedGuard } from '_src/ui/app/hooks/useUnlockedGuard';
import { PERA_TYPE_ARG } from '@pera-io/pera/utils';
import { useNavigate, useSearchParams } from 'react-router-dom';

function CoinsSelectorPage() {
	const [searchParams] = useSearchParams();
	const coinType = searchParams.get('type') || PERA_TYPE_ARG;
	const navigate = useNavigate();

	if (useUnlockedGuard()) {
		return null;
	}

	return (
		<Overlay
			showModal={true}
			title="Select Coin"
			closeOverlay={() =>
				navigate(
					`/send?${new URLSearchParams({
						type: coinType,
					}).toString()}`,
				)
			}
		>
			<ActiveCoinsCard activeCoinType={coinType} showActiveCoin={false} />
		</Overlay>
	);
}

export default CoinsSelectorPage;
