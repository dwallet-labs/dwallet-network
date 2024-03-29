// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import Overlay from '_components/overlay';
import { useNavigate, useSearchParams } from 'react-router-dom';

import { SelectValidatorCard } from '../validators/SelectValidatorCard';
import StakingCard from './StakingCard';

function StakePage() {
	const [searchParams] = useSearchParams();
	const validatorAddress = searchParams.get('address');
	const unstake = searchParams.get('unstake') === 'true';

	const navigate = useNavigate();
	const stakingTitle = unstake ? 'Unstake DWLT' : 'Stake DWLT';

	return (
		<Overlay
			showModal={true}
			title={validatorAddress ? stakingTitle : 'Select a Validator'}
			closeOverlay={() => navigate('/')}
		>
			{validatorAddress ? <StakingCard /> : <SelectValidatorCard />}
		</Overlay>
	);
}

export default StakePage;
