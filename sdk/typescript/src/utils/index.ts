// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

export { formatAddress, formatDigest } from './format.js';
export {
	isValidIkaAddress,
	isValidIkaObjectId,
	isValidTransactionDigest,
	normalizeStructTag,
	normalizeIkaAddress,
	normalizeIkaObjectId,
	parseStructTag,
	IKA_ADDRESS_LENGTH,
} from './ika-types.js';

export {
	fromB64,
	toB64,
	fromHEX,
	toHex,
	toHEX,
	fromHex,
	fromBase64,
	toBase64,
	fromBase58,
	toBase58,
} from '@mysten/bcs';
export { isValidIkaNSName, normalizeIkaNSName } from './ikans.js';

export {
	IKA_DECIMALS,
	NIKA_PER_IKA,
	MOVE_STDLIB_ADDRESS,
	IKA_FRAMEWORK_ADDRESS,
	IKA_SYSTEM_ADDRESS,
	IKA_CLOCK_OBJECT_ID,
	IKA_SYSTEM_MODULE_NAME,
	IKA_TYPE_ARG,
	IKA_SYSTEM_STATE_OBJECT_ID,
} from './constants.js';

export { isValidNamedPackage, isValidNamedType } from './move-registry.js';

export { deriveDynamicFieldID } from './dynamic-fields.js';
