// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { IkaObjectResponse } from '@ika-io/ika/client';

export const hasDisplayData = (obj: IkaObjectResponse) => !!obj.data?.display?.data;
