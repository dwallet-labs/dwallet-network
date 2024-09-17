// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { PeraObjectResponse } from '@pera-io/pera/client';

export const hasDisplayData = (obj: PeraObjectResponse) => !!obj.data?.display?.data;
