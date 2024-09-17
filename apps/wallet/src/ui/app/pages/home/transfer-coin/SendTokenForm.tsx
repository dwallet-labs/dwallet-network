// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useActiveAddress } from '_app/hooks/useActiveAddress';
import BottomMenuLayout, { Content, Menu } from '_app/shared/bottom-menu-layout';
import { Button } from '_app/shared/ButtonUI';
import { Text } from '_app/shared/text';
import { AddressInput } from '_components/address-input';
import Alert from '_components/alert';
import Loading from '_components/loading';
import { parseAmount } from '_helpers';
import { useGetAllCoins } from '_hooks';
import { GAS_SYMBOL } from '_src/ui/app/redux/slices/pera-objects/Coin';
import { InputWithAction } from '_src/ui/app/shared/InputWithAction';
import { CoinFormat, useCoinMetadata, useFormatCoin, usePeraNSEnabled } from '@mysten/core';
import { usePeraClient } from '@mysten/dapp-kit';
import { ArrowRight16 } from '@mysten/icons';
import { type CoinStruct } from '@pera-io/pera/client';
import { isValidPeraNSName, PERA_TYPE_ARG } from '@pera-io/pera/utils';
import { useQuery } from '@tanstack/react-query';
import { Field, Form, Formik, useFormikContext } from 'formik';
import { useEffect, useMemo } from 'react';

import { createTokenTransferTransaction } from './utils/transaction';
import { createValidationSchemaStepOne } from './validation';

const initialValues = {
	to: '',
	amount: '',
	isPayAllPera: false,
	gasBudgetEst: '',
};

export type FormValues = typeof initialValues;

export type SubmitProps = {
	to: string;
	amount: string;
	isPayAllPera: boolean;
	coinIds: string[];
	coins: CoinStruct[];
	gasBudgetEst: string;
};

export type SendTokenFormProps = {
	coinType: string;
	onSubmit: (values: SubmitProps) => void;
	initialAmount: string;
	initialTo: string;
};

function totalBalance(coins: CoinStruct[]): bigint {
	return coins.reduce((partialSum, c) => partialSum + getBalanceFromCoinStruct(c), BigInt(0));
}
function getBalanceFromCoinStruct(coin: CoinStruct): bigint {
	return BigInt(coin.balance);
}

function GasBudgetEstimation({
	coinDecimals,
	coins,
}: {
	coinDecimals: number;
	coins: CoinStruct[];
}) {
	const activeAddress = useActiveAddress();
	const { values, setFieldValue } = useFormikContext<FormValues>();
	const peraNSEnabled = usePeraNSEnabled();

	const client = usePeraClient();
	const { data: gasBudget } = useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: [
			'transaction-gas-budget-estimate',
			{
				to: values.to,
				amount: values.amount,
				coins,
				activeAddress,
				coinDecimals,
			},
		],
		queryFn: async () => {
			if (!values.amount || !values.to || !coins || !activeAddress) {
				return null;
			}

			let to = values.to;
			if (peraNSEnabled && isValidPeraNSName(values.to)) {
				const address = await client.resolveNameServiceAddress({
					name: values.to,
				});
				if (!address) {
					throw new Error('PeraNS name not found.');
				}
				to = address;
			}

			const tx = createTokenTransferTransaction({
				to,
				amount: values.amount,
				coinType: PERA_TYPE_ARG,
				coinDecimals,
				isPayAllPera: values.isPayAllPera,
				coins,
			});

			tx.setSender(activeAddress);
			await tx.build({ client });
			return tx.blockData.gasConfig.budget;
		},
	});

	const [formattedGas] = useFormatCoin(gasBudget, PERA_TYPE_ARG);

	// gasBudgetEstimation should change when the amount above changes
	useEffect(() => {
		setFieldValue('gasBudgetEst', formattedGas, true);
	}, [formattedGas, setFieldValue, values.amount]);

	return (
		<div className="px-2 my-2 flex w-full gap-2 justify-between">
			<div className="flex gap-1">
				<Text variant="body" color="gray-80" weight="medium">
					Estimated Gas Fees
				</Text>
			</div>
			<Text variant="body" color="gray-90" weight="medium">
				{formattedGas ? formattedGas + ' ' + GAS_SYMBOL : '--'}
			</Text>
		</div>
	);
}

// Set the initial gasEstimation from initial amount
// base on the input amount field update the gasEstimation value
// Separating the gasEstimation from the formik context to access the input amount value and update the gasEstimation value
export function SendTokenForm({
	coinType,
	onSubmit,
	initialAmount = '',
	initialTo = '',
}: SendTokenFormProps) {
	const client = usePeraClient();
	const activeAddress = useActiveAddress();
	// Get all coins of the type
	const { data: coinsData, isPending: coinsIsPending } = useGetAllCoins(coinType, activeAddress!);

	const { data: peraCoinsData, isPending: peraCoinsIsPending } = useGetAllCoins(
		PERA_TYPE_ARG,
		activeAddress!,
	);

	const peraCoins = peraCoinsData;
	const coins = coinsData;
	const coinBalance = totalBalance(coins || []);
	const peraBalance = totalBalance(peraCoins || []);

	const coinMetadata = useCoinMetadata(coinType);
	const coinDecimals = coinMetadata.data?.decimals ?? 0;

	const [tokenBalance, symbol, queryResult] = useFormatCoin(coinBalance, coinType, CoinFormat.FULL);
	const peraNSEnabled = usePeraNSEnabled();

	const validationSchemaStepOne = useMemo(
		() => createValidationSchemaStepOne(client, peraNSEnabled, coinBalance, symbol, coinDecimals),
		[client, coinBalance, symbol, coinDecimals, peraNSEnabled],
	);

	// remove the comma from the token balance
	const formattedTokenBalance = tokenBalance.replace(/,/g, '');
	const initAmountBig = parseAmount(initialAmount, coinDecimals);

	return (
		<Loading
			loading={
				queryResult.isPending || coinMetadata.isPending || peraCoinsIsPending || coinsIsPending
			}
		>
			<Formik
				initialValues={{
					amount: initialAmount,
					to: initialTo,
					isPayAllPera:
						!!initAmountBig && initAmountBig === coinBalance && coinType === PERA_TYPE_ARG,
					gasBudgetEst: '',
				}}
				validationSchema={validationSchemaStepOne}
				enableReinitialize
				validateOnMount
				validateOnChange
				onSubmit={async ({ to, amount, isPayAllPera, gasBudgetEst }: FormValues) => {
					if (!coins || !peraCoins) return;
					const coinsIDs = [...coins]
						.sort((a, b) => Number(b.balance) - Number(a.balance))
						.map(({ coinObjectId }) => coinObjectId);

					if (peraNSEnabled && isValidPeraNSName(to)) {
						const address = await client.resolveNameServiceAddress({
							name: to,
						});
						if (!address) {
							throw new Error('PeraNS name not found.');
						}
						to = address;
					}

					const data = {
						to,
						amount,
						isPayAllPera,
						coins,
						coinIds: coinsIDs,
						gasBudgetEst,
					};
					onSubmit(data);
				}}
			>
				{({ isValid, isSubmitting, setFieldValue, values, submitForm, validateField }) => {
					const newPayPeraAll =
						parseAmount(values.amount, coinDecimals) === coinBalance && coinType === PERA_TYPE_ARG;
					if (values.isPayAllPera !== newPayPeraAll) {
						setFieldValue('isPayAllPera', newPayPeraAll);
					}

					const hasEnoughBalance =
						values.isPayAllPera ||
						peraBalance >
							parseAmount(values.gasBudgetEst, coinDecimals) +
								parseAmount(coinType === PERA_TYPE_ARG ? values.amount : '0', coinDecimals);

					return (
						<BottomMenuLayout>
							<Content>
								<Form autoComplete="off" noValidate>
									<div className="w-full flex flex-col flex-grow">
										<div className="px-2 mb-2.5">
											<Text variant="caption" color="steel" weight="semibold">
												Select Coin Amount to Send
											</Text>
										</div>

										<InputWithAction
											data-testid="coin-amount-input"
											type="numberInput"
											name="amount"
											placeholder="0.00"
											prefix={values.isPayAllPera ? '~ ' : ''}
											actionText="Max"
											suffix={` ${symbol}`}
											actionType="button"
											allowNegative={false}
											decimals
											rounded="lg"
											dark
											onActionClicked={async () => {
												// using await to make sure the value is set before the validation
												await setFieldValue('amount', formattedTokenBalance);
												validateField('amount');
											}}
											actionDisabled={
												parseAmount(values?.amount, coinDecimals) === coinBalance ||
												queryResult.isPending ||
												!coinBalance
											}
										/>
									</div>
									{!hasEnoughBalance && isValid ? (
										<div className="mt-3">
											<Alert>Insufficient PERA to cover transaction</Alert>
										</div>
									) : null}

									{coins ? <GasBudgetEstimation coinDecimals={coinDecimals} coins={coins} /> : null}

									<div className="w-full flex gap-2.5 flex-col mt-7.5">
										<div className="px-2 tracking-wider">
											<Text variant="caption" color="steel" weight="semibold">
												Enter Recipient Address
											</Text>
										</div>
										<div className="w-full flex relative items-center flex-col">
											<Field component={AddressInput} name="to" placeholder="Enter Address" />
										</div>
									</div>
								</Form>
							</Content>
							<Menu stuckClass="sendCoin-cta" className="w-full px-0 pb-0 mx-0 gap-2.5">
								<Button
									type="submit"
									onClick={submitForm}
									variant="primary"
									loading={isSubmitting}
									disabled={
										!isValid || isSubmitting || !hasEnoughBalance || values.gasBudgetEst === ''
									}
									size="tall"
									text="Review"
									after={<ArrowRight16 />}
								/>
							</Menu>
						</BottomMenuLayout>
					);
				}}
			</Formik>
		</Loading>
	);
}
