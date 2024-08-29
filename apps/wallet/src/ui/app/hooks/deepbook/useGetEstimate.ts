// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { useLotSize } from '_app/hooks/deepbook/useLotSize';
import { useActiveAccount } from '_app/hooks/useActiveAccount';
import { type WalletSigner } from '_app/WalletSigner';
import { DEEPBOOK_KEY, WALLET_FEES_PERCENTAGE } from '_pages/swap/constants';
import { useDeepBookContext } from '_shared/deepBook/context';
import { useSuiClient } from '@mysten/dapp-kit';
import { type DeepBookClient } from '@mysten/deepbook';
import { TransactionBlock } from '@dwallet-network/dwallet.js/builder';
import { type CoinStruct, type DWalletClient } from '@dwallet-network/dwallet.js/client';
import { useQuery, useQueryClient } from '@tanstack/react-query';
import BigNumber from 'bignumber.js';

const MAX_COINS_PER_REQUEST = 10;
const ESTIMATE_RETRY_COUNT = 3;

async function getCoinsByBalance({
	coinType,
	balance,
	suiClient,
	address,
}: {
	coinType: string;
	balance: string;
	suiClient: DWalletClient;
	address: string;
}) {
	let cursor: string | undefined | null = null;
	let currentBalance = 0n;
	let hasNextPage = true;
	const coins = [];
	const bigIntBalance = BigInt(new BigNumber(balance).integerValue(BigNumber.ROUND_UP).toString());

	while (currentBalance < bigIntBalance && hasNextPage) {
		const { data, nextCursor } = await suiClient.getCoins({
			owner: address,
			coinType,
			cursor,
			limit: MAX_COINS_PER_REQUEST,
		});

		if (!data || !data.length) {
			break;
		}

		for (const coin of data) {
			currentBalance += BigInt(coin.balance);
			coins.push(coin);

			if (currentBalance >= bigIntBalance) {
				break;
			}
		}

		cursor = nextCursor;
		hasNextPage = !!nextCursor;
	}

	if (!coins.length) {
		throw new Error('No coins found in balance');
	}

	return coins;
}

function formatBalance(balance: string, lotSize: number) {
	const balanceBigNumber = new BigNumber(balance);
	const remainder = balanceBigNumber.mod(lotSize);

	if (remainder.isEqualTo(0)) {
		return balanceBigNumber.toString();
	}

	const roundedDownBalance = balanceBigNumber.minus(remainder);
	return roundedDownBalance.abs().toString();
}

function getWalletFee(balance: string) {
	return new BigNumber(balance)
		.times(WALLET_FEES_PERCENTAGE / 100)
		.integerValue(BigNumber.ROUND_DOWN)
		.toString();
}

function getBalanceAndWalletFees(balance: string, totalBalance: string, conversionRate: number) {
	const bigNumberTotalBalance = new BigNumber(totalBalance).shiftedBy(conversionRate);
	const bigNumberBalance = new BigNumber(balance);
	const walletFees = getWalletFee(bigNumberBalance.toString());
	const balanceAndWalletFees = bigNumberBalance.plus(walletFees);

	if (balanceAndWalletFees.isGreaterThan(bigNumberTotalBalance)) {
		/**
		 * If the balance + wallet fees is greater than the total balance, we need to
		 * recalculate the balance and wallet fees.
		 */
		const remainingBalance = bigNumberBalance.minus(walletFees).toString();
		const newWalletFee = getWalletFee(remainingBalance);

		return {
			actualBalance: remainingBalance,
			actualWalletFee: newWalletFee,
		};
	}

	return {
		actualBalance: bigNumberBalance.toString(),
		actualWalletFee: walletFees,
	};
}

async function getPlaceMarketOrderTxn({
	deepBookClient,
	poolId,
	accountCapId,
	address,
	isAsk,
	baseBalance,
	quoteBalance,
	quoteCoins,
	walletFeeAddress,
	totalBaseBalance,
	totalQuoteBalance,
	baseConversionRate,
	quoteConversionRate,
	lotSize,
}: {
	deepBookClient: DeepBookClient;
	poolId: string;
	accountCapId: string;
	address: string;
	isAsk: boolean;
	baseBalance: string;
	quoteBalance: string;
	baseCoins: CoinStruct[];
	quoteCoins: CoinStruct[];
	walletFeeAddress: string;
	totalBaseBalance: string;
	totalQuoteBalance: string;
	baseConversionRate: number;
	quoteConversionRate: number;
	lotSize: string;
}) {
	const txb = new TransactionBlock();
	const accountCap = accountCapId || deepBookClient.createAccountCap(txb);

	let walletFeeCoin;
	let txnResult;

	if (isAsk) {
		const { actualBalance, actualWalletFee } = getBalanceAndWalletFees(
			baseBalance,
			totalBaseBalance,
			baseConversionRate,
		);

		const actualBalanceFormatted = formatBalance(actualBalance, parseInt(lotSize));

		const swapCoin = txb.splitCoins(txb.gas, [actualBalanceFormatted]);
		walletFeeCoin = txb.splitCoins(txb.gas, [actualWalletFee]);
		txnResult = await deepBookClient.placeMarketOrder(
			accountCap,
			poolId,
			BigInt(actualBalanceFormatted),
			'ask',
			swapCoin,
			undefined,
			undefined,
			address,
			txb,
		);
	} else {
		const primaryCoinInput = txb.object(quoteCoins[0].coinObjectId);
		const restCoins = quoteCoins.slice(1);

		if (restCoins.length) {
			txb.mergeCoins(
				primaryCoinInput,
				restCoins.map((coin) => txb.object(coin.coinObjectId)),
			);
		}

		const { actualBalance, actualWalletFee } = getBalanceAndWalletFees(
			quoteBalance,
			totalQuoteBalance,
			quoteConversionRate,
		);

		const [swapCoin, walletCoin] = txb.splitCoins(primaryCoinInput, [
			actualBalance,
			actualWalletFee,
		]);

		txnResult = await deepBookClient.swapExactQuoteForBase(
			poolId,
			swapCoin,
			BigInt(actualBalance),
			address,
			undefined,
			txb,
		);

		walletFeeCoin = walletCoin;
	}

	if (!accountCapId) {
		txnResult.transferObjects([accountCap], address);
	}

	if (walletFeeCoin) txnResult.transferObjects([walletFeeCoin], walletFeeAddress);

	return txnResult;
}

export function useGetEstimate({
	accountCapId,
	signer,
	coinType,
	poolId,
	baseBalance,
	quoteBalance,
	isAsk,
	totalBaseBalance,
	totalQuoteBalance,
	baseConversionRate,
	quoteConversionRate,
}: {
	accountCapId: string;
	signer: WalletSigner | null;
	coinType: string;
	poolId: string;
	baseBalance: string;
	quoteBalance: string;
	isAsk: boolean;
	totalBaseBalance: string;
	totalQuoteBalance: string;
	baseConversionRate: number;
	quoteConversionRate: number;
}) {
	const walletFeeAddress = useDeepBookContext().walletFeeAddress;
	const queryClient = useQueryClient();
	const suiClient = useSuiClient();
	const activeAccount = useActiveAccount();
	const activeAddress = activeAccount?.address;
	const deepBookClient = useDeepBookContext().client;
	const lotSize = useLotSize(poolId);

	return useQuery({
		// eslint-disable-next-line @tanstack/query/exhaustive-deps
		queryKey: [
			DEEPBOOK_KEY,
			'get-estimate',
			poolId,
			accountCapId,
			coinType,
			activeAddress,
			baseBalance,
			quoteBalance,
			isAsk,
			totalBaseBalance,
			totalQuoteBalance,
			baseConversionRate,
			quoteConversionRate,
			lotSize,
		],
		queryFn: async () => {
			const [baseCoins, quoteCoins] = await Promise.all([
				getCoinsByBalance({
					coinType,
					balance: baseBalance,
					suiClient,
					address: activeAddress!,
				}),
				getCoinsByBalance({
					coinType,
					balance: quoteBalance,
					suiClient,
					address: activeAddress!,
				}),
			]);

			if ((isAsk && !baseCoins.length) || (!isAsk && !quoteCoins.length)) {
				throw new Error('No coins found in balance');
			}

			const txn = await getPlaceMarketOrderTxn({
				deepBookClient,
				poolId,
				accountCapId,
				address: activeAddress!,
				isAsk,
				baseCoins,
				quoteCoins,
				baseBalance,
				quoteBalance,
				walletFeeAddress,
				totalBaseBalance,
				totalQuoteBalance,
				baseConversionRate,
				quoteConversionRate,
				lotSize,
			});

			if (!accountCapId) {
				await queryClient.invalidateQueries({ queryKey: ['get-owned-objects'] });
			}

			const dryRunResponse = await signer!.dryRunTransactionBlock({ transactionBlock: txn });

			return {
				txn,
				dryRunResponse,
			};
		},
		enabled:
			!!baseBalance &&
			baseBalance !== '0' &&
			!!quoteBalance &&
			quoteBalance !== '0' &&
			!!signer &&
			!!activeAddress,
		retry: ESTIMATE_RETRY_COUNT,
	});
}
