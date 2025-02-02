// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
import type { AwsClientOptions } from './aws-client.js';
import type { AwsKmsSignerOptions } from './aws-kms-signer.js';
import { AwsKmsSigner } from './aws-kms-signer.js';

export { AwsKmsSigner };

export type { AwsKmsSignerOptions, AwsClientOptions };
