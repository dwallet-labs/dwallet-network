// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { QueryClient } from '@tanstack/react-query';

export const queryClient = new QueryClient({
	defaultOptions: {
		queries: {
			// We default the stale time to 5 minutes, which is an arbitrary number selected to
			// strike the balance between stale data and cache hits.
			// Individual queries can override this value based on their caching needs.
			staleTime: 5 * 60 * 1000,
			refetchInterval: false,
			refetchIntervalInBackground: false,
			// TODO: re-enable/remove when api is healthy ===>
			retry: false,
			refetchOnWindowFocus: false,
			refetchOnMount: false,
			//<======
		},
	},
});
