// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! A mock implementation of Sui JSON-RPC client.

use crate::error::{BridgeError, BridgeResult};
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use sui_json_rpc_types::SuiTransactionBlockResponse;
use sui_json_rpc_types::{EventFilter, EventPage, SuiEvent};
use sui_types::base_types::ObjectID;
use sui_types::base_types::ObjectRef;
use sui_types::digests::TransactionDigest;
use sui_types::event::EventID;
use sui_types::gas_coin::GasCoin;
use sui_types::object::Owner;
use sui_types::transaction::Transaction;
use sui_types::Identifier;

use crate::sui_client::SuiClientInner;
use crate::types::BridgeCommittee;

/// Mock client used in test environments.
#[allow(clippy::type_complexity)]
#[derive(Clone, Debug)]
pub struct SuiMockClient {
    // the top two fields do not change during tests so we don't need them to be Arc<Mutex>>
    chain_identifier: String,
    latest_checkpoint_sequence_number: u64,
    events: Arc<Mutex<HashMap<(ObjectID, Identifier, EventID), EventPage>>>,
    past_event_query_params: Arc<Mutex<VecDeque<(ObjectID, Identifier, EventID)>>>,
    events_by_tx_digest: Arc<Mutex<HashMap<TransactionDigest, Vec<SuiEvent>>>>,
    transaction_responses:
        Arc<Mutex<HashMap<TransactionDigest, BridgeResult<SuiTransactionBlockResponse>>>>,
    get_object_info: Arc<Mutex<HashMap<ObjectID, (GasCoin, ObjectRef, Owner)>>>,

    requested_transactions_tx: tokio::sync::broadcast::Sender<TransactionDigest>,
}

impl SuiMockClient {
    pub fn default() -> Self {
        Self {
            chain_identifier: "".to_string(),
            latest_checkpoint_sequence_number: 0,
            events: Default::default(),
            past_event_query_params: Default::default(),
            events_by_tx_digest: Default::default(),
            transaction_responses: Default::default(),
            get_object_info: Default::default(),
            requested_transactions_tx: tokio::sync::broadcast::channel(10000).0,
        }
    }

    pub fn add_event_response(
        &self,
        package: ObjectID,
        module: Identifier,
        cursor: EventID,
        events: EventPage,
    ) {
        self.events
            .lock()
            .unwrap()
            .insert((package, module, cursor), events);
    }

    pub fn pop_front_past_event_query_params(&self) -> Option<(ObjectID, Identifier, EventID)> {
        self.past_event_query_params.lock().unwrap().pop_front()
    }

    pub fn add_events_by_tx_digest(&self, tx_digest: TransactionDigest, events: Vec<SuiEvent>) {
        self.events_by_tx_digest
            .lock()
            .unwrap()
            .insert(tx_digest, events);
    }

    pub fn add_transaction_response(
        &self,
        tx_digest: TransactionDigest,
        response: BridgeResult<SuiTransactionBlockResponse>,
    ) {
        self.transaction_responses
            .lock()
            .unwrap()
            .insert(tx_digest, response);
    }

    pub fn add_gas_object_info(&self, gas_coin: GasCoin, object_ref: ObjectRef, owner: Owner) {
        self.get_object_info
            .lock()
            .unwrap()
            .insert(object_ref.0, (gas_coin, object_ref, owner));
    }

    pub fn subscribe_to_requested_transactions(
        &self,
    ) -> tokio::sync::broadcast::Receiver<TransactionDigest> {
        self.requested_transactions_tx.subscribe()
    }
}

#[async_trait]
impl SuiClientInner for SuiMockClient {
    type Error = sui_sdk::error::Error;

    // Unwraps in this function: We assume the responses are pre-populated
    // by the test before calling into this function.
    async fn query_events(
        &self,
        query: EventFilter,
        cursor: EventID,
    ) -> Result<EventPage, Self::Error> {
        let events = self.events.lock().unwrap();
        match query {
            EventFilter::MoveEventModule { package, module } => {
                self.past_event_query_params.lock().unwrap().push_back((
                    package,
                    module.clone(),
                    cursor,
                ));
                Ok(events
                    .get(&(package, module.clone(), cursor))
                    .cloned()
                    .unwrap_or_else(|| {
                        panic!(
                            "No preset events found for package: {:?}, module: {:?}, cursor: {:?}",
                            package, module, cursor
                        )
                    }))
            }
            _ => unimplemented!(),
        }
    }

    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<Vec<SuiEvent>, Self::Error> {
        Ok(self
            .events_by_tx_digest
            .lock()
            .unwrap()
            .get(&tx_digest)
            .cloned()
            .unwrap_or_else(|| panic!("No preset events found for tx_digest: {:?}", tx_digest)))
    }

    async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
        Ok(self.chain_identifier.clone())
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, Self::Error> {
        Ok(self.latest_checkpoint_sequence_number)
    }

    async fn get_bridge_committee(&self) -> Result<BridgeCommittee, Self::Error> {
        unimplemented!()
    }

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<SuiTransactionBlockResponse, BridgeError> {
        self.requested_transactions_tx.send(*tx.digest()).unwrap();
        Ok(self
            .transaction_responses
            .lock()
            .unwrap()
            .get(tx.digest())
            .cloned()
            .unwrap_or_else(|| panic!("No preset transaction response found for tx: {:?}", tx))?)
    }

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        self.get_object_info
            .lock()
            .unwrap()
            .get(&gas_object_id)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "No preset gas object info found for gas_object_id: {:?}",
                    gas_object_id
                )
            })
    }
}
