// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { forwardRef, type ComponentProps, type ReactNode } from 'react';

import { TextArea } from './controls/TextArea';
import FormField from './FormField';

type TextAreaFieldProps = {
	name: string;
	label: ReactNode;
} & ComponentProps<'textarea'>;

export const TextAreaField = forwardRef<HTMLTextAreaElement, TextAreaFieldProps>(
	({ label, ...props }, forwardedRef) => (
		<FormField name={props.name} label={label}>
			<TextArea {...props} ref={forwardedRef} />
		</FormField>
	),
);
