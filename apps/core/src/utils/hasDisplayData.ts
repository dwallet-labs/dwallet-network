// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { SuiObjectResponse } from '@dwallet/dwallet.js/client';

export const hasDisplayData = (obj: SuiObjectResponse) => !!obj.data?.display?.data;
