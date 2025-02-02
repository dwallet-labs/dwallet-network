// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { IkaMoveObject, IkaObjectData } from '@ika-io/ika/client';

const COIN_TYPE = '0x2::coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;

export const GAS_TYPE_ARG = '0x2::ika::IKA';
export const GAS_SYMBOL = 'IKA';

// TODO use sdk
export class Coin {
	public static isCoin(obj: IkaObjectData) {
		const type = obj?.content?.dataType === 'package' ? 'package' : obj?.content?.type;
		return type?.startsWith(COIN_TYPE) ?? false;
	}

	public static getCoinTypeArg(obj: IkaMoveObject) {
		const res = obj.type.match(COIN_TYPE_ARG_REGEX);
		return res ? res[1] : null;
	}

	public static isIKA(obj: IkaMoveObject) {
		const arg = Coin.getCoinTypeArg(obj);
		return arg ? Coin.getCoinSymbol(arg) === 'IKA' : false;
	}

	public static getCoinSymbol(coinTypeArg: string) {
		return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
	}

	public static getBalance(obj: IkaMoveObject): bigint {
		return BigInt((obj.fields as { balance: string }).balance);
	}

	public static getID(obj: IkaMoveObject): string {
		return (obj.fields as { id: { id: string } }).id.id;
	}

	public static getCoinTypeFromArg(coinTypeArg: string) {
		return `${COIN_TYPE}<${coinTypeArg}>`;
	}
}
