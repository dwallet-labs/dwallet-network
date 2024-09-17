// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

import { injectDappInterface } from './interface-inject';
import { setupMessagesProxy } from './messages-proxy';

injectDappInterface();
setupMessagesProxy();
