// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

pub(crate) use indexer_analytical_store::*;
pub use indexer_store::*;
pub(crate) use indexer_store_v2::*;
pub use pg_indexer_analytical_store::PgIndexerAnalyticalStore;
pub use pg_indexer_store::PgIndexerStore;
pub use pg_indexer_store_v2::PgIndexerStoreV2;

mod indexer_analytical_store;
mod indexer_store;
pub mod indexer_store_v2;
pub mod module_resolver;
pub(crate) mod module_resolver_v2;
mod pg_indexer_analytical_store;
mod pg_indexer_store;
mod pg_indexer_store_v2;
mod pg_partition_manager;
mod query;

pub(crate) mod diesel_macro {
    macro_rules! read_only_blocking {
        ($pool:expr, $query:expr) => {{
            let mut pg_pool_conn = crate::get_pg_pool_connection($pool)?;
            pg_pool_conn
                .build_transaction()
                .read_only()
                .run($query)
                .map_err(|e| IndexerError::PostgresReadError(e.to_string()))
        }};
    }

    macro_rules! transactional_blocking {
        ($pool:expr, $query:expr) => {{
            let mut pg_pool_conn = crate::get_pg_pool_connection($pool)?;
            pg_pool_conn
                .build_transaction()
                .serializable()
                .read_write()
                .run($query)
                .map_err(|e| IndexerError::PostgresWriteError(e.to_string()))
        }};
    }

    macro_rules! transactional_blocking_with_retry {
        ($pool:expr, $query:expr, $max_elapsed:expr) => {{
            let mut backoff = backoff::ExponentialBackoff::default();
            backoff.max_elapsed_time = Some($max_elapsed);

            let result = match backoff::retry(backoff, || {
                let mut pg_pool_conn = crate::get_pg_pool_connection($pool).map_err(|e| {
                    backoff::Error::Transient {
                        err: IndexerError::PostgresWriteError(e.to_string()),
                        retry_after: None,
                    }
                })?;
                pg_pool_conn
                    .build_transaction()
                    .read_write()
                    .run($query)
                    .map_err(|e| {
                        tracing::error!("Error with persisting data into DB: {:?}", e);
                        backoff::Error::Transient {
                            err: IndexerError::PostgresWriteError(e.to_string()),
                            retry_after: None,
                        }
                    })
            }) {
                Ok(v) => Ok(v),
                Err(backoff::Error::Transient { err, .. }) => Err(err),
                Err(backoff::Error::Permanent(err)) => Err(err),
            };

            result
        }};
    }

    pub(crate) use read_only_blocking;
    pub(crate) use transactional_blocking;
    pub(crate) use transactional_blocking_with_retry;
}
