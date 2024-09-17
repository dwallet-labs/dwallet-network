// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { PeraMoveObject, PeraObjectData } from '@pera-io/pera/client';

const COIN_TYPE = '0x2::coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;

export const GAS_TYPE_ARG = '0x2::pera::PERA';
export const GAS_SYMBOL = 'PERA';

// TODO use sdk
export class Coin {
	public static isCoin(obj: PeraObjectData) {
		const type = obj?.content?.dataType === 'package' ? 'package' : obj?.content?.type;
		return type?.startsWith(COIN_TYPE) ?? false;
	}

	public static getCoinTypeArg(obj: PeraMoveObject) {
		const res = obj.type.match(COIN_TYPE_ARG_REGEX);
		return res ? res[1] : null;
	}

	public static isPERA(obj: PeraMoveObject) {
		const arg = Coin.getCoinTypeArg(obj);
		return arg ? Coin.getCoinSymbol(arg) === 'PERA' : false;
	}

	public static getCoinSymbol(coinTypeArg: string) {
		return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
	}

	public static getBalance(obj: PeraMoveObject): bigint {
		return BigInt((obj.fields as { balance: string }).balance);
	}

	public static getID(obj: PeraMoveObject): string {
		return (obj.fields as { id: { id: string } }).id.id;
	}

	public static getCoinTypeFromArg(coinTypeArg: string) {
		return `${COIN_TYPE}<${coinTypeArg}>`;
	}
}
