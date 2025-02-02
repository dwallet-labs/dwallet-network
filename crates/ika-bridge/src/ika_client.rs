// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use async_trait::async_trait;
use core::panic;
use fastcrypto::traits::ToFromBytes;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::Arc;
use std::time::Duration;
use ika_json_rpc_api::BridgeReadApiClient;
use ika_json_rpc_types::DevInspectResults;
use ika_json_rpc_types::{EventFilter, Page, IkaEvent};
use ika_json_rpc_types::{
    EventPage, IkaObjectDataOptions, IkaTransactionBlockResponse,
    IkaTransactionBlockResponseOptions,
};
use ika_sdk::{IkaClient as IkaSdkClient, IkaClientBuilder};
use ika_types::base_types::ObjectRef;
use ika_types::base_types::SequenceNumber;
use ika_types::bridge::BridgeSummary;
use ika_types::bridge::BridgeTreasurySummary;
use ika_types::bridge::MoveTypeCommitteeMember;
use ika_types::bridge::MoveTypeParsedTokenTransferMessage;
use ika_types::gas_coin::GasCoin;
use ika_types::object::Owner;
use ika_types::parse_ika_type_tag;
use ika_types::transaction::Argument;
use ika_types::transaction::CallArg;
use ika_types::transaction::Command;
use ika_types::transaction::ObjectArg;
use ika_types::transaction::ProgrammableTransaction;
use ika_types::transaction::Transaction;
use ika_types::transaction::TransactionKind;
use ika_types::TypeTag;
use ika_types::BRIDGE_PACKAGE_ID;
use ika_types::IKA_BRIDGE_OBJECT_ID;
use ika_types::{
    base_types::{ObjectID, IkaAddress},
    digests::TransactionDigest,
    event::EventID,
    Identifier,
};
use tokio::sync::OnceCell;
use tracing::{error, warn};

use crate::crypto::BridgeAuthorityPublicKey;
use crate::error::{BridgeError, BridgeResult};
use crate::events::IkaBridgeEvent;
use crate::metrics::BridgeMetrics;
use crate::retry_with_max_elapsed_time;
use crate::types::BridgeActionStatus;
use crate::types::ParsedTokenTransferMessage;
use crate::types::{BridgeAction, BridgeAuthority, BridgeCommittee};

pub struct IkaClient<P> {
    inner: P,
    bridge_metrics: Arc<BridgeMetrics>,
}

pub type IkaBridgeClient = IkaClient<IkaSdkClient>;

impl IkaBridgeClient {
    pub async fn new(rpc_url: &str, bridge_metrics: Arc<BridgeMetrics>) -> anyhow::Result<Self> {
        let inner = IkaClientBuilder::default()
            .build(rpc_url)
            .await
            .map_err(|e| {
                anyhow!("Can't establish connection with Ika Rpc {rpc_url}. Error: {e}")
            })?;
        let self_ = Self {
            inner,
            bridge_metrics,
        };
        self_.describe().await?;
        Ok(self_)
    }

    pub fn ika_client(&self) -> &IkaSdkClient {
        &self.inner
    }
}

impl<P> IkaClient<P>
where
    P: IkaClientInner,
{
    pub fn new_for_testing(inner: P) -> Self {
        Self {
            inner,
            bridge_metrics: Arc::new(BridgeMetrics::new_for_testing()),
        }
    }

    // TODO assert chain identifier
    async fn describe(&self) -> anyhow::Result<()> {
        let chain_id = self.inner.get_chain_identifier().await?;
        let block_number = self.inner.get_latest_checkpoint_sequence_number().await?;
        tracing::info!(
            "IkaClient is connected to chain {chain_id}, current block number: {block_number}"
        );
        Ok(())
    }

    /// Get the mutable bridge object arg on chain.
    // We retry a few times in case of errors. If it fails eventually, we panic.
    // In general it's safe to call in the beginning of the program.
    // After the first call, the result is cached since the value should never change.
    pub async fn get_mutable_bridge_object_arg_must_succeed(&self) -> ObjectArg {
        static ARG: OnceCell<ObjectArg> = OnceCell::const_new();
        *ARG.get_or_init(|| async move {
            let Ok(Ok(bridge_object_arg)) = retry_with_max_elapsed_time!(
                self.inner.get_mutable_bridge_object_arg(),
                Duration::from_secs(30)
            ) else {
                panic!("Failed to get bridge object arg after retries");
            };
            bridge_object_arg
        })
        .await
    }

    /// Query emitted Events that are defined in the given Move Module.
    pub async fn query_events_by_module(
        &self,
        package: ObjectID,
        module: Identifier,
        // cursor is exclusive
        cursor: Option<EventID>,
    ) -> BridgeResult<Page<IkaEvent, EventID>> {
        let filter = EventFilter::MoveEventModule {
            package,
            module: module.clone(),
        };
        let events = self.inner.query_events(filter.clone(), cursor).await?;

        // Safeguard check that all events are emitted from requested package and module
        assert!(events
            .data
            .iter()
            .all(|event| event.type_.address.as_ref() == package.as_ref()
                && event.type_.module == module));
        Ok(events)
    }

    /// Returns BridgeAction from a Ika Transaction with transaction hash
    /// and the event index. If event is declared in an unrecognized
    /// package, return error.
    pub async fn get_bridge_action_by_tx_digest_and_event_idx_maybe(
        &self,
        tx_digest: &TransactionDigest,
        event_idx: u16,
    ) -> BridgeResult<BridgeAction> {
        let events = self.inner.get_events_by_tx_digest(*tx_digest).await?;
        let event = events
            .get(event_idx as usize)
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;
        if event.type_.address.as_ref() != BRIDGE_PACKAGE_ID.as_ref() {
            return Err(BridgeError::BridgeEventInUnrecognizedIkaPackage);
        }
        let bridge_event = IkaBridgeEvent::try_from_ika_event(event)?
            .ok_or(BridgeError::NoBridgeEventsInTxPosition)?;

        bridge_event
            .try_into_bridge_action(*tx_digest, event_idx)
            .ok_or(BridgeError::BridgeEventNotActionable)
    }

    pub async fn get_bridge_summary(&self) -> BridgeResult<BridgeSummary> {
        self.inner
            .get_bridge_summary()
            .await
            .map_err(|e| BridgeError::InternalError(format!("Can't get bridge committee: {e}")))
    }

    pub async fn is_bridge_paused(&self) -> BridgeResult<bool> {
        self.get_bridge_summary()
            .await
            .map(|summary| summary.is_frozen)
    }

    pub async fn get_treasury_summary(&self) -> BridgeResult<BridgeTreasurySummary> {
        Ok(self.get_bridge_summary().await?.treasury)
    }

    pub async fn get_token_id_map(&self) -> BridgeResult<HashMap<u8, TypeTag>> {
        self.get_bridge_summary()
            .await?
            .treasury
            .id_token_type_map
            .into_iter()
            .map(|(id, name)| {
                parse_ika_type_tag(&format!("0x{name}"))
                    .map(|name| (id, name))
                    .map_err(|e| {
                        BridgeError::InternalError(format!(
                            "Failed to retrieve token id mapping: {e}, type name: {name}"
                        ))
                    })
            })
            .collect()
    }

    pub async fn get_notional_values(&self) -> BridgeResult<HashMap<u8, u64>> {
        let bridge_summary = self.get_bridge_summary().await?;
        bridge_summary
            .treasury
            .id_token_type_map
            .iter()
            .map(|(id, type_name)| {
                bridge_summary
                    .treasury
                    .supported_tokens
                    .iter()
                    .find_map(|(tn, metadata)| {
                        if type_name == tn {
                            Some((*id, metadata.notional_value))
                        } else {
                            None
                        }
                    })
                    .ok_or(BridgeError::InternalError(
                        "Error encountered when retrieving token notional values.".into(),
                    ))
            })
            .collect()
    }

    pub async fn get_bridge_committee(&self) -> BridgeResult<BridgeCommittee> {
        let bridge_summary =
            self.inner.get_bridge_summary().await.map_err(|e| {
                BridgeError::InternalError(format!("Can't get bridge committee: {e}"))
            })?;
        let move_type_bridge_committee = bridge_summary.committee;

        let mut authorities = vec![];
        // TODO: move this to MoveTypeBridgeCommittee
        for (_, member) in move_type_bridge_committee.members {
            let MoveTypeCommitteeMember {
                ika_address,
                bridge_pubkey_bytes,
                voting_power,
                http_rest_url,
                blocklisted,
            } = member;
            let pubkey = BridgeAuthorityPublicKey::from_bytes(&bridge_pubkey_bytes)?;
            let base_url = from_utf8(&http_rest_url).unwrap_or_else(|_e| {
                warn!(
                    "Bridge authority address: {}, pubkey: {:?} has invalid http url: {:?}",
                    ika_address, bridge_pubkey_bytes, http_rest_url
                );
                ""
            });
            authorities.push(BridgeAuthority {
                ika_address,
                pubkey,
                voting_power,
                base_url: base_url.into(),
                is_blocklisted: blocklisted,
            });
        }
        BridgeCommittee::new(authorities)
    }

    pub async fn get_chain_identifier(&self) -> BridgeResult<String> {
        Ok(self.inner.get_chain_identifier().await?)
    }

    pub async fn get_reference_gas_price_until_success(&self) -> u64 {
        loop {
            let Ok(Ok(rgp)) = retry_with_max_elapsed_time!(
                self.inner.get_reference_gas_price(),
                Duration::from_secs(30)
            ) else {
                self.bridge_metrics
                    .ika_rpc_errors
                    .with_label_values(&["get_reference_gas_price"])
                    .inc();
                error!("Failed to get reference gas price");
                continue;
            };
            return rgp;
        }
    }

    pub async fn get_latest_checkpoint_sequence_number(&self) -> BridgeResult<u64> {
        Ok(self.inner.get_latest_checkpoint_sequence_number().await?)
    }

    pub async fn execute_transaction_block_with_effects(
        &self,
        tx: ika_types::transaction::Transaction,
    ) -> BridgeResult<IkaTransactionBlockResponse> {
        self.inner.execute_transaction_block_with_effects(tx).await
    }

    // TODO: this function is very slow (seconds) in tests, we need to optimize it
    pub async fn get_token_transfer_action_onchain_status_until_success(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> BridgeActionStatus {
        loop {
            let bridge_object_arg = self.get_mutable_bridge_object_arg_must_succeed().await;
            let Ok(Ok(status)) = retry_with_max_elapsed_time!(
                self.inner.get_token_transfer_action_onchain_status(
                    bridge_object_arg,
                    source_chain_id,
                    seq_number
                ),
                Duration::from_secs(30)
            ) else {
                self.bridge_metrics
                    .ika_rpc_errors
                    .with_label_values(&["get_token_transfer_action_onchain_status"])
                    .inc();
                error!(
                    source_chain_id,
                    seq_number, "Failed to get token transfer action onchain status"
                );
                continue;
            };
            return status;
        }
    }

    pub async fn get_token_transfer_action_onchain_signatures_until_success(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Option<Vec<Vec<u8>>> {
        loop {
            let bridge_object_arg = self.get_mutable_bridge_object_arg_must_succeed().await;
            let Ok(Ok(sigs)) = retry_with_max_elapsed_time!(
                self.inner.get_token_transfer_action_onchain_signatures(
                    bridge_object_arg,
                    source_chain_id,
                    seq_number
                ),
                Duration::from_secs(30)
            ) else {
                self.bridge_metrics
                    .ika_rpc_errors
                    .with_label_values(&["get_token_transfer_action_onchain_signatures"])
                    .inc();
                error!(
                    source_chain_id,
                    seq_number, "Failed to get token transfer action onchain signatures"
                );
                continue;
            };
            return sigs;
        }
    }

    pub async fn get_parsed_token_transfer_message(
        &self,
        source_chain_id: u8,
        seq_number: u64,
    ) -> BridgeResult<Option<ParsedTokenTransferMessage>> {
        let bridge_object_arg = self.get_mutable_bridge_object_arg_must_succeed().await;
        let message = self
            .inner
            .get_parsed_token_transfer_message(bridge_object_arg, source_chain_id, seq_number)
            .await?;
        Ok(match message {
            Some(payload) => Some(ParsedTokenTransferMessage::try_from(payload)?),
            None => None,
        })
    }

    pub async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        self.inner
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await
    }
}

/// Use a trait to abstract over the IkaSDKClient and IkaMockClient for testing.
#[async_trait]
pub trait IkaClientInner: Send + Sync {
    type Error: Into<anyhow::Error> + Send + Sync + std::error::Error + 'static;
    async fn query_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
    ) -> Result<EventPage, Self::Error>;

    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<Vec<IkaEvent>, Self::Error>;

    async fn get_chain_identifier(&self) -> Result<String, Self::Error>;

    async fn get_reference_gas_price(&self) -> Result<u64, Self::Error>;

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, Self::Error>;

    async fn get_mutable_bridge_object_arg(&self) -> Result<ObjectArg, Self::Error>;

    async fn get_bridge_summary(&self) -> Result<BridgeSummary, Self::Error>;

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<IkaTransactionBlockResponse, BridgeError>;

    async fn get_token_transfer_action_onchain_status(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError>;

    async fn get_token_transfer_action_onchain_signatures(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, BridgeError>;

    async fn get_parsed_token_transfer_message(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeParsedTokenTransferMessage>, BridgeError>;

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner);
}

#[async_trait]
impl IkaClientInner for IkaSdkClient {
    type Error = ika_sdk::error::Error;

    async fn query_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
    ) -> Result<EventPage, Self::Error> {
        self.event_api()
            .query_events(query, cursor, None, false)
            .await
    }

    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<Vec<IkaEvent>, Self::Error> {
        self.event_api().get_events(tx_digest).await
    }

    async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
        self.read_api().get_chain_identifier().await
    }

    async fn get_reference_gas_price(&self) -> Result<u64, Self::Error> {
        self.governance_api().get_reference_gas_price().await
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, Self::Error> {
        self.read_api()
            .get_latest_checkpoint_sequence_number()
            .await
    }

    async fn get_mutable_bridge_object_arg(&self) -> Result<ObjectArg, Self::Error> {
        let initial_shared_version = self
            .http()
            .get_bridge_object_initial_shared_version()
            .await?;
        Ok(ObjectArg::SharedObject {
            id: IKA_BRIDGE_OBJECT_ID,
            initial_shared_version: SequenceNumber::from_u64(initial_shared_version),
            mutable: true,
        })
    }

    async fn get_bridge_summary(&self) -> Result<BridgeSummary, Self::Error> {
        self.http().get_latest_bridge().await.map_err(|e| e.into())
    }

    async fn get_token_transfer_action_onchain_status(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<BridgeActionStatus, BridgeError> {
        dev_inspect_bridge::<u8>(
            self,
            bridge_object_arg,
            source_chain_id,
            seq_number,
            "get_token_transfer_action_status",
        )
        .await
        .and_then(|status_byte| BridgeActionStatus::try_from(status_byte).map_err(Into::into))
    }

    async fn get_token_transfer_action_onchain_signatures(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<Vec<Vec<u8>>>, BridgeError> {
        dev_inspect_bridge::<Option<Vec<Vec<u8>>>>(
            self,
            bridge_object_arg,
            source_chain_id,
            seq_number,
            "get_token_transfer_action_signatures",
        )
        .await
    }

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<IkaTransactionBlockResponse, BridgeError> {
        match self.quorum_driver_api().execute_transaction_block(
            tx,
            IkaTransactionBlockResponseOptions::new().with_effects().with_events(),
            Some(ika_types::quorum_driver_types::ExecuteTransactionRequestType::WaitForEffectsCert),
        ).await {
            Ok(response) => Ok(response),
            Err(e) => return Err(BridgeError::IkaTxFailureGeneric(e.to_string())),
        }
    }

    async fn get_parsed_token_transfer_message(
        &self,
        bridge_object_arg: ObjectArg,
        source_chain_id: u8,
        seq_number: u64,
    ) -> Result<Option<MoveTypeParsedTokenTransferMessage>, BridgeError> {
        dev_inspect_bridge::<Option<MoveTypeParsedTokenTransferMessage>>(
            self,
            bridge_object_arg,
            source_chain_id,
            seq_number,
            "get_parsed_token_transfer_message",
        )
        .await
    }

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        loop {
            match self
                .read_api()
                .get_object_with_options(
                    gas_object_id,
                    IkaObjectDataOptions::default().with_owner().with_content(),
                )
                .await
                .map(|resp| resp.data)
            {
                Ok(Some(gas_obj)) => {
                    let owner = gas_obj.owner.expect("Owner is requested");
                    let gas_coin = GasCoin::try_from(&gas_obj)
                        .unwrap_or_else(|err| panic!("{} is not a gas coin: {err}", gas_object_id));
                    return (gas_coin, gas_obj.object_ref(), owner);
                }
                other => {
                    warn!("Can't get gas object: {:?}: {:?}", gas_object_id, other);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }
}

/// Helper function to dev-inspect `bridge::{function_name}` function
/// with bridge object arg, source chain id, seq number as param
/// and parse the return value as `T`.
async fn dev_inspect_bridge<T>(
    ika_client: &IkaSdkClient,
    bridge_object_arg: ObjectArg,
    source_chain_id: u8,
    seq_number: u64,
    function_name: &str,
) -> Result<T, BridgeError>
where
    T: DeserializeOwned,
{
    let pt = ProgrammableTransaction {
        inputs: vec![
            CallArg::Object(bridge_object_arg),
            CallArg::Pure(bcs::to_bytes(&source_chain_id).unwrap()),
            CallArg::Pure(bcs::to_bytes(&seq_number).unwrap()),
        ],
        commands: vec![Command::move_call(
            BRIDGE_PACKAGE_ID,
            Identifier::new("bridge").unwrap(),
            Identifier::new(function_name).unwrap(),
            vec![],
            vec![Argument::Input(0), Argument::Input(1), Argument::Input(2)],
        )],
    };
    let kind = TransactionKind::programmable(pt);
    let resp = ika_client
        .read_api()
        .dev_inspect_transaction_block(IkaAddress::ZERO, kind, None, None, None)
        .await?;
    let DevInspectResults {
        results, effects, ..
    } = resp;
    let Some(results) = results else {
        return Err(BridgeError::Generic(format!(
            "No results returned for '{}', effects: {:?}",
            function_name, effects
        )));
    };
    let return_values = &results
        .first()
        .ok_or(BridgeError::Generic(format!(
            "No return values for '{}', results: {:?}",
            function_name, results
        )))?
        .return_values;
    let (value_bytes, _type_tag) = return_values.first().ok_or(BridgeError::Generic(format!(
        "No first return value for '{}', results: {:?}",
        function_name, results
    )))?;
    bcs::from_bytes::<T>(value_bytes).map_err(|e| {
        BridgeError::Generic(format!(
            "Failed to parse return value for '{}', error: {:?}, results: {:?}",
            function_name, e, results
        ))
    })
}

#[cfg(test)]
mod tests {
    use crate::crypto::BridgeAuthorityKeyPair;
    use crate::e2e_tests::test_utils::TestClusterWrapperBuilder;
    use crate::{
        events::{EmittedIkaToEthTokenBridgeV1, MoveTokenDepositedEvent},
        ika_mock_client::IkaMockClient,
        test_utils::{
            approve_action_with_validator_secrets, bridge_token, get_test_eth_to_ika_bridge_action,
            get_test_ika_to_eth_bridge_action,
        },
        types::IkaToEthBridgeAction,
    };
    use ethers::types::Address as EthAddress;
    use move_core_types::account_address::AccountAddress;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use ika_types::bridge::{BridgeChainId, TOKEN_ID_IKA, TOKEN_ID_USDC};
    use ika_types::crypto::get_key_pair;

    use super::*;
    use crate::events::{init_all_struct_tags, IkaToEthTokenBridgeV1};

    #[tokio::test]
    async fn get_bridge_action_by_tx_digest_and_event_idx_maybe() {
        // Note: for random events generated in this test, we only care about
        // tx_digest and event_seq, so it's ok that package and module does
        // not match the query parameters.
        telemetry_subscribers::init_for_testing();
        let mock_client = IkaMockClient::default();
        let ika_client = IkaClient::new_for_testing(mock_client.clone());
        let tx_digest = TransactionDigest::random();

        // Ensure all struct tags are inited
        init_all_struct_tags();

        let sanitized_event_1 = EmittedIkaToEthTokenBridgeV1 {
            nonce: 1,
            ika_chain_id: BridgeChainId::IkaTestnet,
            ika_address: IkaAddress::random_for_testing_only(),
            eth_chain_id: BridgeChainId::EthSepolia,
            eth_address: EthAddress::random(),
            token_id: TOKEN_ID_IKA,
            amount_ika_adjusted: 100,
        };
        let emitted_event_1 = MoveTokenDepositedEvent {
            seq_num: sanitized_event_1.nonce,
            source_chain: sanitized_event_1.ika_chain_id as u8,
            sender_address: sanitized_event_1.ika_address.to_vec(),
            target_chain: sanitized_event_1.eth_chain_id as u8,
            target_address: sanitized_event_1.eth_address.as_bytes().to_vec(),
            token_type: sanitized_event_1.token_id,
            amount_ika_adjusted: sanitized_event_1.amount_ika_adjusted,
        };

        let mut ika_event_1 = IkaEvent::random_for_testing();
        ika_event_1.type_ = IkaToEthTokenBridgeV1.get().unwrap().clone();
        ika_event_1.bcs = bcs::to_bytes(&emitted_event_1).unwrap();

        #[derive(Serialize, Deserialize)]
        struct RandomStruct {}

        let event_2: RandomStruct = RandomStruct {};
        // undeclared struct tag
        let mut ika_event_2 = IkaEvent::random_for_testing();
        ika_event_2.type_ = IkaToEthTokenBridgeV1.get().unwrap().clone();
        ika_event_2.type_.module = Identifier::from_str("unrecognized_module").unwrap();
        ika_event_2.bcs = bcs::to_bytes(&event_2).unwrap();

        // Event 3 is defined in non-bridge package
        let mut ika_event_3 = ika_event_1.clone();
        ika_event_3.type_.address = AccountAddress::random();

        mock_client.add_events_by_tx_digest(
            tx_digest,
            vec![
                ika_event_1.clone(),
                ika_event_2.clone(),
                ika_event_1.clone(),
                ika_event_3.clone(),
            ],
        );
        let expected_action_1 = BridgeAction::IkaToEthBridgeAction(IkaToEthBridgeAction {
            ika_tx_digest: tx_digest,
            ika_tx_event_index: 0,
            ika_bridge_event: sanitized_event_1.clone(),
        });
        assert_eq!(
            ika_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 0)
                .await
                .unwrap(),
            expected_action_1,
        );
        let expected_action_2 = BridgeAction::IkaToEthBridgeAction(IkaToEthBridgeAction {
            ika_tx_digest: tx_digest,
            ika_tx_event_index: 2,
            ika_bridge_event: sanitized_event_1.clone(),
        });
        assert_eq!(
            ika_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 2)
                .await
                .unwrap(),
            expected_action_2,
        );
        assert!(matches!(
            ika_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 1)
                .await
                .unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition
        ),);
        assert!(matches!(
            ika_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 3)
                .await
                .unwrap_err(),
            BridgeError::BridgeEventInUnrecognizedIkaPackage
        ),);
        assert!(matches!(
            ika_client
                .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 4)
                .await
                .unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition
        ),);

        // if the StructTag matches with unparsable bcs, it returns an error
        ika_event_2.type_ = IkaToEthTokenBridgeV1.get().unwrap().clone();
        mock_client.add_events_by_tx_digest(tx_digest, vec![ika_event_2]);
        ika_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, 2)
            .await
            .unwrap_err();
    }

    // Test get_action_onchain_status.
    // Use validator secrets to bridge USDC from Ethereum initially.
    // TODO: we need an e2e test for this with published solidity contract and committee with BridgeNodes
    #[tokio::test(flavor = "multi_thread", worker_threads = 8)]
    async fn test_get_action_onchain_status_for_ika_to_eth_transfer() {
        telemetry_subscribers::init_for_testing();
        let mut bridge_keys = vec![];
        for _ in 0..=3 {
            let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
            bridge_keys.push(kp);
        }
        let mut test_cluster = TestClusterWrapperBuilder::new()
            .with_bridge_authority_keys(bridge_keys)
            .with_deploy_tokens(true)
            .build()
            .await;

        let bridge_metrics = Arc::new(BridgeMetrics::new_for_testing());
        let ika_client =
            IkaClient::new(&test_cluster.inner.fullnode_handle.rpc_url, bridge_metrics)
                .await
                .unwrap();
        let bridge_authority_keys = test_cluster.authority_keys_clone();

        // Wait until committee is set up
        test_cluster
            .trigger_reconfiguration_if_not_yet_and_assert_bridge_committee_initialized()
            .await;
        let context = &mut test_cluster.inner.wallet;
        let sender = context.active_address().unwrap();
        let usdc_amount = 5000000;
        let bridge_object_arg = ika_client
            .get_mutable_bridge_object_arg_must_succeed()
            .await;
        let id_token_map = ika_client.get_token_id_map().await.unwrap();

        // 1. Create a Eth -> Ika Transfer (recipient is sender address), approve with validator secrets and assert its status to be Claimed
        let action = get_test_eth_to_ika_bridge_action(None, Some(usdc_amount), Some(sender), None);
        let usdc_object_ref = approve_action_with_validator_secrets(
            context,
            bridge_object_arg,
            action.clone(),
            &bridge_authority_keys,
            Some(sender),
            &id_token_map,
        )
        .await
        .unwrap();

        let status = ika_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        assert_eq!(status, BridgeActionStatus::Claimed);

        // 2. Create a Ika -> Eth Transfer, approve with validator secrets and assert its status to be Approved
        // We need to actually send tokens to bridge to initialize the record.
        let eth_recv_address = EthAddress::random();
        let bridge_event = bridge_token(
            context,
            eth_recv_address,
            usdc_object_ref,
            id_token_map.get(&TOKEN_ID_USDC).unwrap().clone(),
            bridge_object_arg,
        )
        .await;
        assert_eq!(bridge_event.nonce, 0);
        assert_eq!(bridge_event.ika_chain_id, BridgeChainId::IkaCustom);
        assert_eq!(bridge_event.eth_chain_id, BridgeChainId::EthCustom);
        assert_eq!(bridge_event.eth_address, eth_recv_address);
        assert_eq!(bridge_event.ika_address, sender);
        assert_eq!(bridge_event.token_id, TOKEN_ID_USDC);
        assert_eq!(bridge_event.amount_ika_adjusted, usdc_amount);

        let action = get_test_ika_to_eth_bridge_action(
            None,
            None,
            Some(bridge_event.nonce),
            Some(bridge_event.amount_ika_adjusted),
            Some(bridge_event.ika_address),
            Some(bridge_event.eth_address),
            Some(TOKEN_ID_USDC),
        );
        let status = ika_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        // At this point, the record is created and the status is Pending
        assert_eq!(status, BridgeActionStatus::Pending);

        // Approve it and assert its status to be Approved
        approve_action_with_validator_secrets(
            context,
            bridge_object_arg,
            action.clone(),
            &bridge_authority_keys,
            None,
            &id_token_map,
        )
        .await;

        let status = ika_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        assert_eq!(status, BridgeActionStatus::Approved);

        // 3. Create a random action and assert its status as NotFound
        let action =
            get_test_ika_to_eth_bridge_action(None, None, Some(100), None, None, None, None);
        let status = ika_client
            .inner
            .get_token_transfer_action_onchain_status(
                bridge_object_arg,
                action.chain_id() as u8,
                action.seq_number(),
            )
            .await
            .unwrap();
        assert_eq!(status, BridgeActionStatus::NotFound);
    }
}
