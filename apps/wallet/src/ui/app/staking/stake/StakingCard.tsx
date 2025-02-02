// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import BottomMenuLayout, { Content, Menu } from '_app/shared/bottom-menu-layout';
import { Button } from '_app/shared/ButtonUI';
import { Collapsible } from '_app/shared/collapse';
import { Text } from '_app/shared/text';
import Loading from '_components/loading';
import { parseAmount } from '_helpers';
import { useCoinsReFetchingConfig } from '_hooks';
import { Coin } from '_redux/slices/ika-objects/Coin';
import { ampli } from '_src/shared/analytics/ampli';
import {
	DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	DELEGATED_STAKES_QUERY_STALE_TIME,
	MIN_NUMBER_IKA_TO_STAKE,
} from '_src/shared/constants';
import { FEATURES } from '_src/shared/experimentation/features';
import { useFeatureIsOn } from '@growthbook/growthbook-react';
import { useCoinMetadata, useGetDelegatedStake } from '@mysten/core';
import { useIkaClientQuery } from '@mysten/dapp-kit';
import { ArrowLeft16 } from '@mysten/icons';
import type { StakeObject } from '@ika-io/ika/client';
import { NIKA_PER_IKA, IKA_TYPE_ARG } from '@ika-io/ika/utils';
import * as Sentry from '@sentry/react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Formik } from 'formik';
import type { FormikHelpers } from 'formik';
import { useCallback, useMemo } from 'react';
import { toast } from 'react-hot-toast';
import { Navigate, useNavigate, useSearchParams } from 'react-router-dom';

import Alert from '../../components/alert';
import { getSignerOperationErrorMessage } from '../../helpers/errorMessages';
import { useActiveAccount } from '../../hooks/useActiveAccount';
import { useQredoTransaction } from '../../hooks/useQredoTransaction';
import { useSigner } from '../../hooks/useSigner';
import { QredoActionIgnoredByUser } from '../../QredoSigner';
import { getDelegationDataByStakeId } from '../getDelegationByStakeId';
import { getStakeIkaByIkaId } from '../getStakeIkaByIkaId';
import StakeForm from './StakeForm';
import { UnStakeForm } from './UnstakeForm';
import { createStakeTransaction, createUnstakeTransaction } from './utils/transaction';
import { createValidationSchema } from './utils/validation';
import { ValidatorFormDetail } from './ValidatorFormDetail';

const initialValues = {
	amount: '',
};

export type FormValues = typeof initialValues;

function StakingCard() {
	const coinType = IKA_TYPE_ARG;
	const activeAccount = useActiveAccount();
	const accountAddress = activeAccount?.address;
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();
	const { data: ikaBalance, isPending: loadingIkaBalances } = useIkaClientQuery(
		'getBalance',
		{ coinType: IKA_TYPE_ARG, owner: accountAddress! },
		{ refetchInterval, staleTime, enabled: !!accountAddress },
	);
	const coinBalance = BigInt(ikaBalance?.totalBalance || 0);
	const [searchParams] = useSearchParams();
	const validatorAddress = searchParams.get('address');
	const stakeIkaIdParams = searchParams.get('staked');
	const unstake = searchParams.get('unstake') === 'true';
	const { data: allDelegation, isPending } = useGetDelegatedStake({
		address: accountAddress || '',
		staleTime: DELEGATED_STAKES_QUERY_STALE_TIME,
		refetchInterval: DELEGATED_STAKES_QUERY_REFETCH_INTERVAL,
	});
	const effectsOnlySharedTransactions = useFeatureIsOn(
		FEATURES.WALLET_EFFECTS_ONLY_SHARED_TRANSACTION as string,
	);

	const { data: system, isPending: validatorsisPending } =
		useIkaClientQuery('getLatestIkaSystemState');

	const totalTokenBalance = useMemo(() => {
		if (!allDelegation) return 0n;
		// return only the total amount of tokens staked for a specific stakeId
		return getStakeIkaByIkaId(allDelegation, stakeIkaIdParams);
	}, [allDelegation, stakeIkaIdParams]);

	const stakeData = useMemo(() => {
		if (!allDelegation || !stakeIkaIdParams) return null;
		// return delegation data for a specific stakeId
		return getDelegationDataByStakeId(allDelegation, stakeIkaIdParams);
	}, [allDelegation, stakeIkaIdParams]);

	const coinSymbol = useMemo(() => (coinType && Coin.getCoinSymbol(coinType)) || '', [coinType]);

	const ikaEarned =
		(stakeData as Extract<StakeObject, { estimatedReward: string }>)?.estimatedReward || '0';

	const { data: metadata } = useCoinMetadata(coinType);
	const coinDecimals = metadata?.decimals ?? 0;
	// set minimum stake amount to 1 IKA
	const minimumStake = parseAmount(MIN_NUMBER_IKA_TO_STAKE.toString(), coinDecimals);

	const validationSchema = useMemo(
		() => createValidationSchema(coinBalance, coinSymbol, coinDecimals, unstake, minimumStake),
		[coinBalance, coinSymbol, coinDecimals, unstake, minimumStake],
	);

	const queryClient = useQueryClient();
	const delegationId = useMemo(() => {
		if (!stakeData || stakeData.status === 'Pending') return null;
		return stakeData.stakedIkaId;
	}, [stakeData]);

	const navigate = useNavigate();
	const signer = useSigner(activeAccount);
	const { clientIdentifier, notificationModal } = useQredoTransaction();

	const stakeToken = useMutation({
		mutationFn: async ({
			tokenTypeArg,
			amount,
			validatorAddress,
		}: {
			tokenTypeArg: string;
			amount: bigint;
			validatorAddress: string;
		}) => {
			if (!validatorAddress || !amount || !tokenTypeArg || !signer) {
				throw new Error('Failed, missing required field');
			}

			const sentryTransaction = Sentry.startTransaction({
				name: 'stake',
			});
			try {
				const transactionBlock = createStakeTransaction(amount, validatorAddress);
				return await signer.signAndExecuteTransactionBlock(
					{
						transactionBlock,
						requestType: effectsOnlySharedTransactions
							? 'WaitForEffectsCert'
							: 'WaitForLocalExecution',
						options: {
							showInput: true,
							showEffects: true,
							showEvents: true,
						},
					},
					clientIdentifier,
				);
			} finally {
				sentryTransaction.finish();
			}
		},
		onSuccess: (_, { amount, validatorAddress }) => {
			ampli.stakedIka({
				stakedAmount: Number(amount / NIKA_PER_IKA),
				validatorAddress: validatorAddress,
			});
		},
	});

	const unStakeToken = useMutation({
		mutationFn: async ({ stakedIkaId }: { stakedIkaId: string }) => {
			if (!stakedIkaId || !signer) {
				throw new Error('Failed, missing required field.');
			}

			const sentryTransaction = Sentry.startTransaction({
				name: 'stake',
			});
			try {
				const transactionBlock = createUnstakeTransaction(stakedIkaId);
				return await signer.signAndExecuteTransactionBlock(
					{
						transactionBlock,
						requestType: effectsOnlySharedTransactions
							? 'WaitForEffectsCert'
							: 'WaitForLocalExecution',
						options: {
							showInput: true,
							showEffects: true,
							showEvents: true,
						},
					},
					clientIdentifier,
				);
			} finally {
				sentryTransaction.finish();
			}
		},
		onSuccess: () => {
			ampli.unstakedIka({
				validatorAddress: validatorAddress!,
			});
		},
	});

	const onHandleSubmit = useCallback(
		async ({ amount }: FormValues, { resetForm }: FormikHelpers<FormValues>) => {
			if (coinType === null || validatorAddress === null) {
				return;
			}
			try {
				const bigIntAmount = parseAmount(amount, coinDecimals);
				let response;
				let txDigest;
				if (unstake) {
					// check for delegation data
					if (!stakeData || !stakeIkaIdParams || stakeData.status === 'Pending') {
						return;
					}
					response = await unStakeToken.mutateAsync({
						stakedIkaId: stakeIkaIdParams,
					});

					txDigest = response.digest;
				} else {
					response = await stakeToken.mutateAsync({
						amount: bigIntAmount,
						tokenTypeArg: coinType,
						validatorAddress: validatorAddress,
					});
					txDigest = response.digest;
				}

				// Invalidate the react query for system state and validator
				Promise.all([
					queryClient.invalidateQueries({
						queryKey: ['system', 'state'],
					}),
					queryClient.invalidateQueries({
						queryKey: ['delegated-stakes'],
					}),
				]);
				resetForm();

				navigate(
					`/receipt?${new URLSearchParams({
						txdigest: txDigest,
						from: 'tokens',
					}).toString()}`,
					response?.transaction
						? {
								state: {
									response,
								},
							}
						: undefined,
				);
			} catch (error) {
				if (error instanceof QredoActionIgnoredByUser) {
					navigate('/');
				} else {
					toast.error(
						<div className="max-w-xs overflow-hidden flex flex-col">
							<strong>{unstake ? 'Unstake' : 'Stake'} failed</strong>
							<small className="text-ellipsis overflow-hidden">
								{getSignerOperationErrorMessage(error)}
							</small>
						</div>,
					);
				}
			}
		},
		[
			coinType,
			validatorAddress,
			coinDecimals,
			unstake,
			queryClient,
			navigate,
			stakeData,
			stakeIkaIdParams,
			unStakeToken,
			stakeToken,
		],
	);

	if (!coinType || !validatorAddress || (!validatorsisPending && !system)) {
		return <Navigate to="/" replace={true} />;
	}
	return (
		<div className="flex flex-col flex-nowrap flex-grow w-full">
			<Loading loading={isPending || validatorsisPending || loadingIkaBalances}>
				<Formik
					initialValues={initialValues}
					validationSchema={validationSchema}
					onSubmit={onHandleSubmit}
					validateOnMount
				>
					{({ isSubmitting, isValid, submitForm, errors, touched }) => (
						<BottomMenuLayout>
							<Content>
								<div className="mb-4">
									<ValidatorFormDetail validatorAddress={validatorAddress} unstake={unstake} />
								</div>

								{unstake ? (
									<UnStakeForm
										stakedIkaId={stakeIkaIdParams!}
										coinBalance={totalTokenBalance}
										coinType={coinType}
										stakingReward={ikaEarned}
										epoch={Number(system?.epoch || 0)}
									/>
								) : (
									<StakeForm
										validatorAddress={validatorAddress}
										coinBalance={coinBalance}
										coinType={coinType}
										epoch={system?.epoch}
									/>
								)}

								{(unstake || touched.amount) && errors.amount ? (
									<div className="mt-2 flex flex-col flex-nowrap">
										<Alert>{errors.amount}</Alert>
									</div>
								) : null}

								{!unstake && (
									<div className="flex-1 mt-7.5">
										<Collapsible title="Staking Rewards" defaultOpen>
											<Text variant="pSubtitle" color="steel-dark" weight="normal">
												Staked IKA starts counting as validator’s stake at the end of the Epoch in
												which it was staked. Rewards are earned separately for each Epoch and become
												available at the end of each Epoch.
											</Text>
										</Collapsible>
									</div>
								)}
							</Content>

							<Menu stuckClass="staked-cta" className="w-full px-0 pb-0 mx-0">
								<Button
									size="tall"
									variant="secondary"
									to="/stake"
									disabled={isSubmitting}
									before={<ArrowLeft16 />}
									text="Back"
								/>
								<Button
									size="tall"
									variant="primary"
									onClick={submitForm}
									disabled={!isValid || isSubmitting || (unstake && !delegationId)}
									loading={isSubmitting}
									text={unstake ? 'Unstake Now' : 'Stake Now'}
								/>
							</Menu>
						</BottomMenuLayout>
					)}
				</Formik>
			</Loading>
			{notificationModal}
		</div>
	);
}

export default StakingCard;
