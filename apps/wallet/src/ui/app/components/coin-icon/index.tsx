// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ImageIcon } from '_app/shared/image-icon';
import { useCoinMetadata } from '@mysten/core';
import { Ika, Unstaked } from '@mysten/icons';
import { normalizeStructTag, IKA_TYPE_ARG } from '@ika-io/ika/utils';
import { cva, type VariantProps } from 'class-variance-authority';

import { useCoinMetadataOverrides } from '../../hooks/useCoinMetadataOverride';

const imageStyle = cva(['rounded-full flex'], {
	variants: {
		size: {
			sm: 'w-6 h-6',
			md: 'w-7.5 h-7.5',
			lg: 'md:w-10 md:h-10 w-8 h-8',
			xl: 'md:w-31.5 md:h-31.5 w-16 h-16 ',
		},
		fill: {
			ika: 'bg-ika',
			ikaPrimary2023: 'bg-ika-primaryBlue2023',
		},
	},
	defaultVariants: {
		size: 'md',
		fill: 'ikaPrimary2023',
	},
});

function IkaCoin() {
	return (
		<Ika className="flex items-center w-full h-full justify-center text-white p-1.5 text-body rounded-full" />
	);
}

type NonIkaCoinProps = {
	coinType: string;
};

function NonIkaCoin({ coinType }: NonIkaCoinProps) {
	const { data: coinMeta } = useCoinMetadata(coinType);
	const coinMetadataOverrides = useCoinMetadataOverrides();

	return (
		<div className="flex h-full w-full items-center justify-center text-white bg-steel rounded-full overflow-hidden">
			{coinMeta?.iconUrl ? (
				<ImageIcon
					src={coinMetadataOverrides[coinType]?.iconUrl ?? coinMeta.iconUrl}
					label={coinMeta.name || coinType}
					fallback={coinMeta.name || coinType}
					rounded="full"
				/>
			) : (
				<Unstaked />
			)}
		</div>
	);
}

export interface CoinIconProps extends VariantProps<typeof imageStyle> {
	coinType: string;
}

export function CoinIcon({ coinType, ...styleProps }: CoinIconProps) {
	const isIka = coinType
		? normalizeStructTag(coinType) === normalizeStructTag(IKA_TYPE_ARG)
		: false;

	return (
		<div className={imageStyle(styleProps)}>
			{isIka ? <IkaCoin /> : <NonIkaCoin coinType={coinType} />}
		</div>
	);
}
