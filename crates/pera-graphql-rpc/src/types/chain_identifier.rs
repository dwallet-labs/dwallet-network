// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::{
    data::{Db, DbConnection, QueryExecutor},
    error::Error,
};
use async_graphql::*;
use diesel::QueryDsl;
use pera_indexer::schema::chain_identifier;
use pera_types::{
    digests::ChainIdentifier as NativeChainIdentifier, messages_checkpoint::CheckpointDigest,
};

pub(crate) struct ChainIdentifier;

impl ChainIdentifier {
    /// Query the Chain Identifier from the DB.
    pub(crate) async fn query(db: &Db) -> Result<NativeChainIdentifier, Error> {
        use chain_identifier::dsl;

        let digest_bytes = db
            .execute(move |conn| {
                conn.first(move || dsl::chain_identifier.select(dsl::checkpoint_digest))
            })
            .await
            .map_err(|e| Error::Internal(format!("Failed to fetch genesis digest: {e}")))?;

        Self::from_bytes(digest_bytes)
    }

    /// Treat `bytes` as a checkpoint digest and extract a chain identifier from it.
    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Result<NativeChainIdentifier, Error> {
        let genesis_digest = CheckpointDigest::try_from(bytes)
            .map_err(|e| Error::Internal(format!("Failed to deserialize genesis digest: {e}")))?;
        Ok(NativeChainIdentifier::from(genesis_digest))
    }
}
