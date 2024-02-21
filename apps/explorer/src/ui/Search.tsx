// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import { Search16 } from '@mysten/icons';
import { Text, Combobox, ComboboxInput, ComboboxList } from '@mysten/ui';

export type SearchResult = {
	id: string;
	label: string;
	type: string;
};

export interface SearchProps {
	onChange: (value: string) => void;
	onSelectResult?: (result: SearchResult) => void;
	placeholder?: string;
	isLoading: boolean;
	options?: SearchResult[];
	queryValue: string;
}

export function Search({
	onChange,
	onSelectResult,
	placeholder,
	options = [],
	isLoading = false,
	queryValue,
}: SearchProps) {
	return (
		<Combobox value={queryValue} onValueChange={onChange}>
			<div className="relative flex h-10 items-center">
				<div className="absolute left-0 ml-3 block items-center text-2xl text-hero-darkest/80">
					<Search16 />
				</div>

				<ComboboxInput
					className="w-full rounded border border-transparent bg-steel-dark pl-10 font-mono text-body font-medium leading-9 text-hero-darkest/80 outline-none placeholder:text-sm placeholder:text-hero-darkest/40 hover:bg-gray-55 focus:bg-gray-55"
					placeholder={placeholder}
				/>
			</div>

			<ComboboxList
				isLoading={isLoading}
				onSelect={({ item }) => {
					onSelectResult?.(item);
				}}
				options={options.map((item) => ({
					item,
					value: `${item.type}/${item.id}`,
					label: item.label,
					after: (
						<Text variant="caption/medium" color="steel">
							{item.type}
						</Text>
					),
				}))}
			/>
		</Combobox>
	);
}
