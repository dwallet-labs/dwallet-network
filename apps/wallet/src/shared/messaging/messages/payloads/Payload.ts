// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import type { BasePayload } from './BasePayload';
import type { ErrorPayload } from './ErrorPayload';

export type Payload = BasePayload | ErrorPayload;
