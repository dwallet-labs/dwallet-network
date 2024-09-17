// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use axum::http::HeaderName;

pub static VERSION_HEADER: HeaderName = HeaderName::from_static("x-pera-rpc-version");
pub static LIMITS_HEADER: HeaderName = HeaderName::from_static("x-pera-rpc-show-usage");
