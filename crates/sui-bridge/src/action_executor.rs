// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//! BridgeActionExecutor receives BridgeActions (from BridgeOrchestrator),
//! collects bridge authority signatures and submit signatures on chain.

use mysten_metrics::spawn_logged_monitored_task;
use shared_crypto::intent::{Intent, IntentMessage};
use sui_json_rpc_types::{
    SuiExecutionStatus, SuiTransactionBlockEffects, SuiTransactionBlockEffectsAPI,
};
use sui_types::{
    base_types::{ObjectID, ObjectRef, SuiAddress},
    committee::VALIDITY_THRESHOLD,
    crypto::{Signature, SuiKeyPair},
    digests::TransactionDigest,
    gas_coin::GasCoin,
    object::Owner,
    programmable_transaction_builder::ProgrammableTransactionBuilder,
    transaction::{Transaction, TransactionData},
};

use crate::{
    client::bridge_authority_aggregator::BridgeAuthorityAggregator,
    error::BridgeError,
    storage::BridgeOrchestratorTables,
    sui_client::{SuiClient, SuiClientInner},
    types::{BridgeAction, VerifiedCertifiedBridgeAction},
};
use std::sync::Arc;
use tracing::{error, info, warn};

pub const CHANNEL_SIZE: usize = 1000;

// delay schedule: at most 16 times including the initial attempt
// 0.1s, 0.2s, 0.4s, 0.8s, 1.6s, 3.2s, 6.4s, 12.8s, 25.6s, 51.2s, 102.4s, 204.8s, 409.6s, 819.2s, 1638.4s
pub const MAX_SIGNING_ATTEMPTS: u64 = 16;
pub const MAX_EXECUTION_ATTEMPTS: u64 = 16;

async fn delay(attempt_times: u64) {
    let delay_ms = 100 * (2 ^ attempt_times);
    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
}

#[derive(Debug)]
pub struct BridgeActionExecutionWrapper(pub BridgeAction, pub u64);

#[derive(Debug)]
pub struct CertifiedBridgeActionExecutionWrapper(pub VerifiedCertifiedBridgeAction, pub u64);

pub trait BridgeActionExecutorTrait {
    fn run(
        self,
    ) -> (
        Vec<tokio::task::JoinHandle<()>>,
        mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
    );
}

pub struct BridgeActionExecutor<C> {
    sui_client: Arc<SuiClient<C>>,
    bridge_auth_agg: Arc<BridgeAuthorityAggregator>,
    key: SuiKeyPair,
    sui_address: SuiAddress,
    gas_object_id: ObjectID,
    store: Arc<BridgeOrchestratorTables>,
}

impl<C> BridgeActionExecutorTrait for BridgeActionExecutor<C>
where
    C: SuiClientInner + 'static,
{
    fn run(
        self,
    ) -> (
        Vec<tokio::task::JoinHandle<()>>,
        mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
    ) {
        let key = self.key;

        let (sender, receiver) = mysten_metrics::metered_channel::channel(
            CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["executor_signing_queue"]),
        );

        let (execution_tx, execution_rx) = mysten_metrics::metered_channel::channel(
            CHANNEL_SIZE,
            &mysten_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["executor_execution_queue"]),
        );
        let execution_tx_clone = execution_tx.clone();
        let sender_clone = sender.clone();
        let mut tasks = vec![];
        tasks.push(spawn_logged_monitored_task!(
            Self::run_signature_aggregation_loop(
                self.bridge_auth_agg,
                sender_clone,
                receiver,
                execution_tx_clone,
            )
        ));

        tasks.push(spawn_logged_monitored_task!(
            Self::run_onchain_execution_loop(
                self.sui_client.clone(),
                key,
                self.sui_address,
                self.gas_object_id,
                self.store.clone(),
                execution_tx,
                execution_rx,
            )
        ));
        (tasks, sender)
    }
}

impl<C> BridgeActionExecutor<C>
where
    C: SuiClientInner + 'static,
{
    pub fn new(
        sui_client: Arc<SuiClient<C>>,
        bridge_auth_agg: Arc<BridgeAuthorityAggregator>,
        store: Arc<BridgeOrchestratorTables>,
        key: SuiKeyPair,
        sui_address: SuiAddress,
        gas_object_id: ObjectID,
    ) -> Self {
        Self {
            sui_client,
            bridge_auth_agg,
            store,
            key,
            gas_object_id,
            sui_address,
        }
    }

    async fn run_signature_aggregation_loop(
        auth_agg: Arc<BridgeAuthorityAggregator>,
        signing_queue_sender: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        mut signing_queue_receiver: mysten_metrics::metered_channel::Receiver<
            BridgeActionExecutionWrapper,
        >,
        execution_queue_sender: mysten_metrics::metered_channel::Sender<
            CertifiedBridgeActionExecutionWrapper,
        >,
    ) {
        info!("Starting run_signature_aggregation_loop");
        while let Some(action) = signing_queue_receiver.recv().await {
            info!("Received action for signing: {:?}", action);
            let auth_agg_clone = auth_agg.clone();
            let signing_queue_sender_clone = signing_queue_sender.clone();
            let execution_queue_sender_clone = execution_queue_sender.clone();
            spawn_logged_monitored_task!(Self::request_signature(
                auth_agg_clone,
                action,
                signing_queue_sender_clone,
                execution_queue_sender_clone
            ));
        }
    }

    async fn request_signature(
        auth_agg: Arc<BridgeAuthorityAggregator>,
        action: BridgeActionExecutionWrapper,
        signing_queue_sender: mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        execution_queue_sender: mysten_metrics::metered_channel::Sender<
            CertifiedBridgeActionExecutionWrapper,
        >,
    ) {
        let BridgeActionExecutionWrapper(action, attempt_times) = action;
        // TODO: use different threshold based on action types.
        match auth_agg
            .request_committee_signatures(action.clone(), VALIDITY_THRESHOLD)
            .await
        {
            Ok(certificate) => {
                execution_queue_sender
                    .send(CertifiedBridgeActionExecutionWrapper(certificate, 0))
                    .await
                    .expect("Sending to execution queue should not fail");
            }
            Err(e) => {
                warn!("Failed to collect sigs for bridge action: {:?}", e);

                if attempt_times >= MAX_SIGNING_ATTEMPTS {
                    error!("Manual intervention is required. Failed to collect sigs for bridge action after {MAX_SIGNING_ATTEMPTS} attempts: {:?}", e);
                    return;
                }
                delay(attempt_times).await;
                signing_queue_sender
                    .send(BridgeActionExecutionWrapper(action, attempt_times + 1))
                    .await
                    .expect("Sending to signing queue should not fail");
            }
        }
    }

    // Before calling this function, `key` and `sui_address` need to be
    // verified to match.
    async fn run_onchain_execution_loop(
        sui_client: Arc<SuiClient<C>>,
        sui_key: SuiKeyPair,
        sui_address: SuiAddress,
        gas_object_id: ObjectID,
        store: Arc<BridgeOrchestratorTables>,
        execution_queue_sender: mysten_metrics::metered_channel::Sender<
            CertifiedBridgeActionExecutionWrapper,
        >,
        mut execution_queue_receiver: mysten_metrics::metered_channel::Receiver<
            CertifiedBridgeActionExecutionWrapper,
        >,
    ) {
        info!("Starting run_onchain_execution_loop");
        while let Some(certificate) = execution_queue_receiver.recv().await {
            info!("Received certified action for execution: {:?}", certificate);
            let CertifiedBridgeActionExecutionWrapper(certificate, attempt_times) = certificate;

            // TODO check gas coin balance here. If gas balance too low, do not proceed.
            let (_gas_coin, gas_object_ref) =
                Self::get_gas_data_assert_ownership(sui_address, gas_object_id, &sui_client).await;

            let tx_data = build_transaction(&gas_object_ref);
            let sig = Signature::new_secure(
                &IntentMessage::new(Intent::sui_transaction(), &tx_data),
                &sui_key,
            );
            let signed_tx = Transaction::from_data(tx_data, vec![sig]);
            let tx_digest = *signed_tx.digest();
            info!(?tx_digest, ?gas_object_ref, "Sending transaction to Sui");
            // TODO: add metrics to detect low balances and so on
            match sui_client
                .execute_transaction_block_with_effects(signed_tx)
                .await
            {
                Ok(effects) => {
                    let effects = effects.effects.expect("We requested effects but got None.");
                    Self::handle_execution_effects(tx_digest, effects, &store, certificate).await
                }

                // If the transaction did not go through, retry up to a certain times.
                Err(err) => {
                    error!("Sui transaction failed at signing: {err:?}");

                    // Do this in a separate task so we won't deadlock here
                    let sender_clone = execution_queue_sender.clone();
                    spawn_logged_monitored_task!(async move {
                        // TODO: metrics + alerts
                        // If it fails for too many times, log and ask for manual intervention.
                        if attempt_times >= MAX_EXECUTION_ATTEMPTS {
                            error!("Manual intervention is required. Failed to collect execute transaction for bridge action after {MAX_EXECUTION_ATTEMPTS} attempts: {:?}", err);
                            return;
                        }
                        delay(attempt_times).await;
                        sender_clone
                            .send(CertifiedBridgeActionExecutionWrapper(
                                certificate,
                                attempt_times + 1,
                            ))
                            .await
                            .expect("Sending to execution queue should not fail");
                        info!("Re-enqueued certificate for execution");
                    });
                }
            }
        }
    }

    async fn handle_execution_effects(
        tx_digest: TransactionDigest,
        effects: SuiTransactionBlockEffects,
        store: &Arc<BridgeOrchestratorTables>,
        certificate: VerifiedCertifiedBridgeAction,
    ) {
        let status = effects.status();
        match status {
            SuiExecutionStatus::Success => {
                info!(?tx_digest, "Sui transaction executed successfully");
                store
                    .remove_pending_actions(&[certificate.data().digest()])
                    .unwrap_or_else(|e| {
                        panic!("Write to DB should not fail: {:?}", e);
                    })
            }
            SuiExecutionStatus::Failure { error } => {
                // In practice the transaction could fail because of running out of gas, but really
                // should not be due to other reasons.
                // This means manual intervention is needed. So we do not push them back to
                // the execution queue because retries are mostly likely going to fail anyway.
                // After human examination, the node should be restarted and fetch them from WAL.

                // TODO metrics + alerts
                error!(?tx_digest, "Manual intervention is needed. Sui transaction executed and failed with error: {error:?}");
            }
        }
    }

    /// Panics if the gas object is not owned by the address.
    async fn get_gas_data_assert_ownership(
        sui_address: SuiAddress,
        gas_object_id: ObjectID,
        sui_client: &SuiClient<C>,
    ) -> (GasCoin, ObjectRef) {
        let (gas_coin, gas_obj_ref, owner) = sui_client
            .get_gas_data_panic_if_not_gas(gas_object_id)
            .await;

        // TODO: when we add multiple gas support in the future we could discard
        // transferred gas object instead.
        assert_eq!(
            owner,
            Owner::AddressOwner(sui_address),
            "Gas object {:?} is no longer owned by address {}",
            gas_object_id,
            sui_address
        );
        (gas_coin, gas_obj_ref)
    }
}

pub async fn submit_to_executor(
    tx: &mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
    action: BridgeAction,
) -> Result<(), BridgeError> {
    tx.send(BridgeActionExecutionWrapper(action, 0))
        .await
        .map_err(|e| BridgeError::Generic(e.to_string()))
}

pub fn build_transaction(gas_object_ref: &ObjectRef) -> TransactionData {
    let sender = SuiAddress::ZERO;
    let mut builder = ProgrammableTransactionBuilder::new();
    builder.pay_sui(vec![SuiAddress::ZERO], vec![1u64]).unwrap();
    let pt = builder.finish();
    TransactionData::new_programmable(sender, vec![*gas_object_ref], pt, 15_000_000, 1500)
}

#[cfg(test)]
mod tests {
    use prometheus::Registry;
    use sui_json_rpc_types::SuiTransactionBlockResponse;
    use sui_types::base_types::random_object_ref;
    use sui_types::crypto::get_key_pair;
    use sui_types::gas_coin::GasCoin;

    use crate::{
        crypto::BridgeAuthorityKeyPair,
        server::mock_handler::BridgeRequestMockHandler,
        sui_mock_client::SuiMockClient,
        test_utils::{
            get_test_authorities_and_run_mock_bridge_server, get_test_sui_to_eth_bridge_action,
            sign_action_with_key,
        },
        types::BridgeCommittee,
    };

    use super::*;

    #[tokio::test]
    async fn test_onchain_execution_loop() {
        let (
            tx,
            sui_client_mock,
            mut tx_subscription,
            store,
            secrets,
            dummy_sui_key,
            mock0,
            mock1,
            mock2,
            mock3,
            _handles,
            gas_object_ref,
            sui_address,
        ) = setup();

        let (action, _, _) = get_bridge_authority_approved_action(
            vec![&mock0, &mock1, &mock2, &mock3],
            vec![&secrets[0], &secrets[1], &secrets[2], &secrets[3]],
        );

        let tx_data = build_transaction(&gas_object_ref);
        let tx_digest = get_tx_digest(tx_data, &dummy_sui_key);

        let gas_coin = GasCoin::new_for_testing(1_000_000_000_000); // dummy gas coin
        sui_client_mock.add_gas_object_info(
            gas_coin.clone(),
            gas_object_ref,
            Owner::AddressOwner(sui_address),
        );

        // Mock the transaction to be successfully executed
        mock_transaction_response(&sui_client_mock, tx_digest, SuiExecutionStatus::Success);

        store.insert_pending_actions(&[action.clone()]).unwrap();
        assert_eq!(
            store.get_all_pending_actions().unwrap()[&action.digest()],
            action.clone()
        );

        // Kick it
        submit_to_executor(&tx, action.clone()).await.unwrap();

        // Expect to see the transaction to be requested and successfully executed hence removed from WAL
        assert_eq!(tx_subscription.recv().await.unwrap(), tx_digest);
        assert!(store.get_all_pending_actions().unwrap().is_empty());

        /////////////////////////////////////////////////////////////////////////////////////////////////
        ////////////////////////////////////// Test execution failure ///////////////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////

        let (action, _, _) = get_bridge_authority_approved_action(
            vec![&mock0, &mock1, &mock2, &mock3],
            vec![&secrets[0], &secrets[1], &secrets[2], &secrets[3]],
        );

        let tx_data = build_transaction(&gas_object_ref);
        let tx_digest = get_tx_digest(tx_data, &dummy_sui_key);

        // Mock the transaction to fail
        mock_transaction_response(
            &sui_client_mock,
            tx_digest,
            SuiExecutionStatus::Failure {
                error: "failure is mother of success".to_string(),
            },
        );

        store.insert_pending_actions(&[action.clone()]).unwrap();
        assert_eq!(
            store.get_all_pending_actions().unwrap()[&action.digest()],
            action.clone()
        );

        // Kick it
        submit_to_executor(&tx, action.clone()).await.unwrap();

        // Expect to see the transaction to be requested and but failed
        assert_eq!(tx_subscription.recv().await.unwrap(), tx_digest);
        // The action is not removed from WAL because the transaction failed
        assert_eq!(
            store.get_all_pending_actions().unwrap()[&action.digest()],
            action.clone()
        );

        /////////////////////////////////////////////////////////////////////////////////////////////////
        //////////////////////////// Test transaction failed at signing stage ///////////////////////////
        /////////////////////////////////////////////////////////////////////////////////////////////////

        let (action, _, _) = get_bridge_authority_approved_action(
            vec![&mock0, &mock1, &mock2, &mock3],
            vec![&secrets[0], &secrets[1], &secrets[2], &secrets[3]],
        );

        let tx_data = build_transaction(&gas_object_ref);
        let tx_digest = get_tx_digest(tx_data, &dummy_sui_key);
        mock_transaction_error(
            &sui_client_mock,
            tx_digest,
            BridgeError::Generic("some random error".to_string()),
        );

        store.insert_pending_actions(&[action.clone()]).unwrap();
        assert_eq!(
            store.get_all_pending_actions().unwrap()[&action.digest()],
            action.clone()
        );

        // Kick it
        submit_to_executor(&tx, action.clone()).await.unwrap();

        // Failure will trigger retry, we wait for 2 requests before checking WAL log
        assert_eq!(tx_subscription.recv().await.unwrap(), tx_digest);
        assert_eq!(tx_subscription.recv().await.unwrap(), tx_digest);
        // The retry is still going on, action still in WAL
        assert!(store
            .get_all_pending_actions()
            .unwrap()
            .contains_key(&action.digest()));

        // Now let it succeed
        mock_transaction_response(&sui_client_mock, tx_digest, SuiExecutionStatus::Success);

        // Give it 1 second to retry and succeed
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        // The action is successful and should be removed from WAL now
        assert!(!store
            .get_all_pending_actions()
            .unwrap()
            .contains_key(&action.digest()));
    }

    #[tokio::test]
    async fn test_signature_aggregation_loop() {
        let (
            tx,
            sui_client_mock,
            mut tx_subscription,
            store,
            secrets,
            dummy_sui_key,
            mock0,
            mock1,
            mock2,
            mock3,
            _handles,
            gas_object_ref,
            sui_address,
        ) = setup();

        let (action, sui_tx_digest, sui_tx_event_index) = get_bridge_authority_approved_action(
            vec![&mock0, &mock1, &mock2, &mock3],
            vec![&secrets[0], &secrets[1], &secrets[2], &secrets[3]],
        );
        mock_bridge_authority_signing_errors(
            vec![&mock0, &mock1, &mock2],
            sui_tx_digest,
            sui_tx_event_index,
        );
        mock_bridge_authority_sigs(
            vec![&mock3],
            &action,
            vec![&secrets[3]],
            sui_tx_digest,
            sui_tx_event_index,
        );

        let gas_coin = GasCoin::new_for_testing(1_000_000_000_000); // dummy gas coin
        sui_client_mock.add_gas_object_info(
            gas_coin,
            gas_object_ref,
            Owner::AddressOwner(sui_address),
        );

        store.insert_pending_actions(&[action.clone()]).unwrap();
        assert_eq!(
            store.get_all_pending_actions().unwrap()[&action.digest()],
            action.clone()
        );

        // Kick it
        submit_to_executor(&tx, action.clone()).await.unwrap();

        // Wait until the transaction is retried at least once (instead of deing dropped)
        loop {
            let requested_times =
                mock0.get_sui_token_events_requested(sui_tx_digest, sui_tx_event_index);
            if requested_times >= 2 {
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        }
        // Nothing is sent to execute yet
        assert_eq!(
            tx_subscription.try_recv().unwrap_err(),
            tokio::sync::broadcast::error::TryRecvError::Empty
        );
        // Still in WAL
        assert_eq!(
            store.get_all_pending_actions().unwrap()[&action.digest()],
            action.clone()
        );

        // Let authorities to sign the action too. Now we are above the threshold
        mock_bridge_authority_sigs(
            vec![&mock2],
            &action,
            vec![&secrets[2]],
            sui_tx_digest,
            sui_tx_event_index,
        );

        let tx_data = build_transaction(&gas_object_ref);
        let tx_digest = get_tx_digest(tx_data, &dummy_sui_key);

        mock_transaction_response(&sui_client_mock, tx_digest, SuiExecutionStatus::Success);

        // Expect to see the transaction to be requested and succeed
        assert_eq!(tx_subscription.recv().await.unwrap(), tx_digest);
        // The action is removed from WAL
        assert!(!store
            .get_all_pending_actions()
            .unwrap()
            .contains_key(&action.digest()));
    }

    fn mock_bridge_authority_sigs(
        mocks: Vec<&BridgeRequestMockHandler>,
        action: &BridgeAction,
        secrets: Vec<&BridgeAuthorityKeyPair>,
        sui_tx_digest: TransactionDigest,
        sui_tx_event_index: u16,
    ) {
        assert_eq!(mocks.len(), secrets.len());
        for (mock, secret) in mocks.iter().zip(secrets.iter()) {
            mock.add_sui_event_response(
                sui_tx_digest,
                sui_tx_event_index,
                Ok(sign_action_with_key(action, secret)),
            );
        }
    }

    fn mock_bridge_authority_signing_errors(
        mocks: Vec<&BridgeRequestMockHandler>,
        sui_tx_digest: TransactionDigest,
        sui_tx_event_index: u16,
    ) {
        for mock in mocks {
            mock.add_sui_event_response(
                sui_tx_digest,
                sui_tx_event_index,
                Err(BridgeError::RestAPIError("small issue".into())),
            );
        }
    }

    /// Create a BridgeAction and mock authorities to return signatures
    fn get_bridge_authority_approved_action(
        mocks: Vec<&BridgeRequestMockHandler>,
        secrets: Vec<&BridgeAuthorityKeyPair>,
    ) -> (BridgeAction, TransactionDigest, u16) {
        let sui_tx_digest = TransactionDigest::random();
        let sui_tx_event_index = 1;
        let action = get_test_sui_to_eth_bridge_action(
            Some(sui_tx_digest),
            Some(sui_tx_event_index),
            None,
            None,
        );

        mock_bridge_authority_sigs(mocks, &action, secrets, sui_tx_digest, sui_tx_event_index);
        (action, sui_tx_digest, sui_tx_event_index)
    }

    fn get_tx_digest(tx_data: TransactionData, dummy_sui_key: &SuiKeyPair) -> TransactionDigest {
        let sig = Signature::new_secure(
            &IntentMessage::new(Intent::sui_transaction(), &tx_data),
            dummy_sui_key,
        );
        let signed_tx = Transaction::from_data(tx_data, vec![sig]);
        *signed_tx.digest()
    }

    fn mock_transaction_response(
        sui_client_mock: &SuiMockClient,
        tx_digest: TransactionDigest,
        status: SuiExecutionStatus,
    ) {
        let mut response = SuiTransactionBlockResponse::new(tx_digest);
        let effects = SuiTransactionBlockEffects::new_for_testing(tx_digest, status);
        response.effects = Some(effects);
        sui_client_mock.add_transaction_response(tx_digest, Ok(response));
    }

    fn mock_transaction_error(
        sui_client_mock: &SuiMockClient,
        tx_digest: TransactionDigest,
        error: BridgeError,
    ) {
        sui_client_mock.add_transaction_response(tx_digest, Err(error));
    }

    #[allow(clippy::type_complexity)]
    fn setup() -> (
        mysten_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        SuiMockClient,
        tokio::sync::broadcast::Receiver<TransactionDigest>,
        Arc<BridgeOrchestratorTables>,
        Vec<BridgeAuthorityKeyPair>,
        SuiKeyPair,
        BridgeRequestMockHandler,
        BridgeRequestMockHandler,
        BridgeRequestMockHandler,
        BridgeRequestMockHandler,
        Vec<tokio::task::JoinHandle<()>>,
        ObjectRef,
        SuiAddress,
    ) {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mysten_metrics::init_metrics(&registry);

        let (sui_address, kp): (_, fastcrypto::secp256k1::Secp256k1KeyPair) = get_key_pair();
        let sui_key = SuiKeyPair::from(kp);
        let gas_object_ref = random_object_ref();
        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());
        let sui_client_mock = SuiMockClient::default();
        let tx_subscription = sui_client_mock.subscribe_to_requested_transactions();
        let sui_client = Arc::new(SuiClient::new_for_testing(sui_client_mock.clone()));

        // The dummy key is used to sign transaction so we can get TransactionDigest easily.
        // User signature is not part of the transaction so it does not matter which key it is.
        let (_, dummy_kp): (_, fastcrypto::secp256k1::Secp256k1KeyPair) = get_key_pair();
        let dummy_sui_key = SuiKeyPair::from(dummy_kp);

        let mock0 = BridgeRequestMockHandler::new();
        let mock1 = BridgeRequestMockHandler::new();
        let mock2 = BridgeRequestMockHandler::new();
        let mock3 = BridgeRequestMockHandler::new();

        let (mut handles, authorities, secrets) = get_test_authorities_and_run_mock_bridge_server(
            vec![2500, 2500, 2500, 2500],
            vec![mock0.clone(), mock1.clone(), mock2.clone(), mock3.clone()],
        );

        let committee = BridgeCommittee::new(authorities).unwrap();

        let agg = Arc::new(BridgeAuthorityAggregator::new(Arc::new(committee)));

        let executor = BridgeActionExecutor::new(
            sui_client.clone(),
            agg.clone(),
            store.clone(),
            sui_key,
            sui_address,
            gas_object_ref.0,
        );

        let (executor_handle, tx) = executor.run();
        handles.extend(executor_handle);
        (
            tx,
            sui_client_mock,
            tx_subscription,
            store,
            secrets,
            dummy_sui_key,
            mock0,
            mock1,
            mock2,
            mock3,
            handles,
            gas_object_ref,
            sui_address,
        )
    }
}
