// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

/// Increment an IntGauge metric, and decrement it when the scope ends.
/// metrics must be an Arc containing a struct containing the field $field.
#[macro_export]
macro_rules! scoped_counter {
    ($metrics: expr, $field: ident) => {{
        let metrics = $metrics.clone();
        metrics.$field.inc();
        ::scopeguard::guard(metrics, |metrics| {
            metrics.$field.dec();
        })
    }};
}
