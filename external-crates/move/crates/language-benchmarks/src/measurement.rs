// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BSD-3-Clause-Clear

use criterion::Criterion;
use criterion_cpu_time::PosixTime;

pub fn cpu_time_measurement() -> Criterion<PosixTime> {
    Criterion::default().with_measurement(PosixTime::UserAndSystemTime)
}

pub fn wall_time_measurement() -> Criterion {
    Criterion::default()
}
