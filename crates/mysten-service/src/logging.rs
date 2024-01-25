// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use telemetry_subscribers::TelemetryConfig;

pub fn init() -> telemetry_subscribers::TelemetryGuards {
    let (guard, _handle) = TelemetryConfig::new().with_env().init();
    guard
}
