// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { PeraObjectChange } from '@pera-io/pera/client';

import { ObjectLink } from '../ObjectLink';
import { PreviewCard } from '../PreviewCard';

const objectTypes: Record<string, Record<string, string>> = {
	published: {
		title: 'Published',
		classes: 'text-green-800 bg-green-200',
	},
	created: {
		title: 'Created',
		classes: 'text-green-800 bg-green-200',
	},
	wrapped: {
		title: 'Wrapped',
		classes: 'text-gray-900 bg-gray-100',
	},
	mutated: {
		title: 'Mutated',
		classes: 'text-yellow-800 bg-yellow-50',
	},
	deleted: {
		title: 'Deleted',
		classes: 'text-red-800 bg-red-50',
	},
	transferred: {},
};

// SPDX-License-Identifier: BSD-3-Clause-Clear
export function ObjectChanges({ objects }: { objects: PeraObjectChange[] }) {
	return (
		<div className="grid grid-cols-1 gap-5">
			{objects.map((object, index) => (
				<ChangedObject key={index} object={object} />
			))}
		</div>
	);
}

function ChangedObject({ object }: { object: PeraObjectChange }) {
	const objectType = objectTypes[object.type];

	return (
		<PreviewCard.Root>
			<PreviewCard.Body>
				<>
					<span className={`${objectType?.classes} px-2 py-0.5 rounded`}>{objectType?.title}</span>
					<div className="flex gap-3 items-center break-words my-2">
						Type:{' '}
						<ObjectLink
							type={'objectType' in object ? object.objectType : ''}
							className="break-words"
						/>
					</div>

					<label className="flex gap-3 items-center flex-wrap break-words">
						Object ID: <ObjectLink object={object} />
					</label>
				</>
			</PreviewCard.Body>

			<PreviewCard.Footer owner={'owner' in object ? object.owner : undefined} />
		</PreviewCard.Root>
	);
}
