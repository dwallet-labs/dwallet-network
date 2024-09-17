// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::{
    error::Error,
    types::{address::Address, pera_address::PeraAddress, validator::Validator},
};
use diesel::PgConnection;
use pera_indexer::db::ConnectionPoolConfig;
use pera_indexer::{apis::GovernanceReadApi, indexer_reader::IndexerReader};
use pera_json_rpc_types::Stake as RpcStakedPera;
use pera_types::{
    governance::StakedPera as NativeStakedPera,
    pera_system_state::pera_system_state_summary::PeraSystemStateSummary as NativePeraSystemStateSummary,
};
use std::{collections::BTreeMap, time::Duration};

pub(crate) struct PgManager {
    pub inner: IndexerReader<PgConnection>,
}

impl PgManager {
    pub(crate) fn new(inner: IndexerReader<PgConnection>) -> Self {
        Self { inner }
    }

    /// Create a new underlying reader, which is used by this type as well as other data providers.
    pub(crate) fn reader_with_config(
        db_url: impl Into<String>,
        pool_size: u32,
        timeout_ms: u64,
    ) -> Result<IndexerReader<PgConnection>, Error> {
        let mut config = ConnectionPoolConfig::default();
        config.set_pool_size(pool_size);
        config.set_statement_timeout(Duration::from_millis(timeout_ms));
        IndexerReader::<PgConnection>::new_with_config(db_url, config)
            .map_err(|e| Error::Internal(format!("Failed to create reader: {e}")))
    }
}

/// Implement methods to be used by graphql resolvers
impl PgManager {
    /// If no epoch was requested or if the epoch requested is in progress,
    /// returns the latest pera system state.
    pub(crate) async fn fetch_pera_system_state(
        &self,
        epoch_id: Option<u64>,
    ) -> Result<NativePeraSystemStateSummary, Error> {
        let latest_pera_system_state = self
            .inner
            .spawn_blocking(move |this| this.get_latest_pera_system_state())
            .await?;

        if let Some(epoch_id) = epoch_id {
            if epoch_id == latest_pera_system_state.epoch {
                Ok(latest_pera_system_state)
            } else {
                Ok(self
                    .inner
                    .spawn_blocking(move |this| this.get_epoch_pera_system_state(Some(epoch_id)))
                    .await?)
            }
        } else {
            Ok(latest_pera_system_state)
        }
    }

    /// Make a request to the RPC for its representations of the staked pera we parsed out of the
    /// object.  Used to implement fields that are implemented in JSON-RPC but not GraphQL (yet).
    pub(crate) async fn fetch_rpc_staked_pera(
        &self,
        stake: NativeStakedPera,
    ) -> Result<RpcStakedPera, Error> {
        let governance_api = GovernanceReadApi::new(self.inner.clone());

        let mut delegated_stakes = governance_api
            .get_delegated_stakes(vec![stake])
            .await
            .map_err(|e| Error::Internal(format!("Error fetching delegated stake. {e}")))?;

        let Some(mut delegated_stake) = delegated_stakes.pop() else {
            return Err(Error::Internal(
                "Error fetching delegated stake. No pools returned.".to_string(),
            ));
        };

        let Some(stake) = delegated_stake.stakes.pop() else {
            return Err(Error::Internal(
                "Error fetching delegated stake. No stake in pool.".to_string(),
            ));
        };

        Ok(stake)
    }
}

/// `checkpoint_viewed_at` represents the checkpoint sequence number at which the set of
/// `PeraValidatorSummary` was queried for. Each `Validator` will inherit this checkpoint, so that
/// when viewing the `Validator`'s state, it will be as if it was read at the same checkpoint.
pub(crate) fn convert_to_validators(
    system_state_at_requested_epoch: NativePeraSystemStateSummary,
    checkpoint_viewed_at: u64,
    requested_for_epoch: u64,
) -> Vec<Validator> {
    let at_risk = BTreeMap::from_iter(system_state_at_requested_epoch.at_risk_validators);
    let reports = BTreeMap::from_iter(system_state_at_requested_epoch.validator_report_records);

    system_state_at_requested_epoch
        .active_validators
        .into_iter()
        .map(move |validator_summary| {
            let at_risk = at_risk.get(&validator_summary.pera_address).copied();
            let report_records = reports.get(&validator_summary.pera_address).map(|addrs| {
                addrs
                    .iter()
                    .cloned()
                    .map(|a| Address {
                        address: PeraAddress::from(a),
                        checkpoint_viewed_at,
                    })
                    .collect()
            });

            Validator {
                validator_summary,
                at_risk,
                report_records,
                checkpoint_viewed_at,
                requested_for_epoch,
            }
        })
        .collect()
}
