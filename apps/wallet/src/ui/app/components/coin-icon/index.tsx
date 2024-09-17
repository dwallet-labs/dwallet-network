// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { ImageIcon } from '_app/shared/image-icon';
import { useCoinMetadata } from '@mysten/core';
import { Pera, Unstaked } from '@mysten/icons';
import { PERA_TYPE_ARG } from '@pera-io/pera/utils';
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
			pera: 'bg-pera',
			peraPrimary2023: 'bg-pera-primaryBlue2023',
		},
	},
	defaultVariants: {
		size: 'md',
		fill: 'peraPrimary2023',
	},
});

function PeraCoin() {
	return (
		<Pera className="flex items-center w-full h-full justify-center text-white p-1.5 text-body rounded-full" />
	);
}

type NonPeraCoinProps = {
	coinType: string;
};

function NonPeraCoin({ coinType }: NonPeraCoinProps) {
	const { data: coinMeta } = useCoinMetadata(coinType);
	const coinMetadataOverrides = useCoinMetadataOverrides();

	return (
		<div className="flex h-full w-full items-center justify-center text-white bg-steel rounded-full">
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
	return (
		<div className={imageStyle(styleProps)}>
			{coinType === PERA_TYPE_ARG ? <PeraCoin /> : <NonPeraCoin coinType={coinType} />}
		</div>
	);
}
