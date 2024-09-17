// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { Spinner16 } from '@mysten/icons';
import { cva, type VariantProps } from 'class-variance-authority';

const styles = cva('', {
	variants: {
		color: {
			inherit: 'text-inherit',
			pera: 'text-pera',
		},
	},
});

export type LoadingIndicatorProps = VariantProps<typeof styles>;

const LoadingIndicator = ({ color = 'pera' }: LoadingIndicatorProps) => {
	return <Spinner16 className={styles({ className: 'animate-spin', color })} />;
};

export default LoadingIndicator;
