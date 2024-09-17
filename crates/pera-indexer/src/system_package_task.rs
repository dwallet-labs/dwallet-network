// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::time::Duration;

use crate::store::diesel_macro::*;
use diesel::r2d2::R2D2Connection;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use pera_types::SYSTEM_PACKAGE_ADDRESSES;
use tokio_util::sync::CancellationToken;

use crate::{indexer_reader::IndexerReader, schema::epochs};

/// Background task responsible for evicting system packages from the package resolver's cache after
/// detecting an epoch boundary.
pub(crate) struct SystemPackageTask<T: R2D2Connection + 'static> {
    /// Holds the DB connection and also the package resolver to evict packages from.
    reader: IndexerReader<T>,
    /// Signal to cancel the task.
    cancel: CancellationToken,
    /// Interval to sleep for between checks.
    interval: Duration,
}

impl<T: R2D2Connection> SystemPackageTask<T> {
    pub(crate) fn new(
        reader: IndexerReader<T>,
        cancel: CancellationToken,
        interval: Duration,
    ) -> Self {
        Self {
            reader,
            cancel,
            interval,
        }
    }

    pub(crate) async fn run(&self) {
        let mut last_epoch: i64 = 0;
        loop {
            tokio::select! {
                _ = self.cancel.cancelled() => {
                    tracing::info!(
                        "Shutdown signal received, terminating system package eviction task"
                    );
                    return;
                }
                _ = tokio::time::sleep(self.interval) => {
                    let pool = self.reader.get_pool();
                    let next_epoch = match run_query_async!(&pool, move |conn| {
                            epochs::dsl::epochs
                                .select(epochs::dsl::epoch)
                                .order_by(epochs::epoch.desc())
                                .first::<i64>(conn)
                        }) {
                        Ok(epoch) => epoch,
                        Err(e) => {
                            tracing::error!("Failed to fetch latest epoch: {:?}", e);
                            continue;
                        }
                    };

                    if next_epoch > last_epoch {
                        last_epoch = next_epoch;
                        tracing::info!(
                            "Detected epoch boundary, evicting system packages from cache"
                        );
                        self.reader
                            .package_resolver()
                            .package_store()
                            .evict(SYSTEM_PACKAGE_ADDRESSES.iter().copied());
                    }
                }
            }
        }
    }
}
