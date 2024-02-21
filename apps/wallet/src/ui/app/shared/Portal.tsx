// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { useEffect, useState } from 'react';
import { createPortal } from 'react-dom';

type PortalProps = {
	children: React.ReactNode;
	containerId: string;
};

export function Portal({ children, containerId }: PortalProps) {
	const [hasMounted, setHasMounted] = useState(false);

	useEffect(() => {
		setHasMounted(true);
	}, []);

	if (!hasMounted) {
		return null;
	}

	return createPortal(children, document.getElementById(containerId)!);
}
