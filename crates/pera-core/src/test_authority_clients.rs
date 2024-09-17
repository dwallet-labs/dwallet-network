// Copyright (c) 2021, Facebook, Inc. and its affiliates
// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::authority::test_authority_builder::TestAuthorityBuilder;
use crate::{authority::AuthorityState, authority_client::AuthorityAPI};
use async_trait::async_trait;
use mysten_metrics::spawn_monitored_task;
use pera_config::genesis::Genesis;
use pera_types::messages_grpc::{
    HandleCertificateResponseV2, HandleSoftBundleCertificatesRequestV3,
    HandleSoftBundleCertificatesResponseV3, HandleTransactionResponse, ObjectInfoRequest,
    ObjectInfoResponse, SystemStateRequest, TransactionInfoRequest, TransactionInfoResponse,
};
use pera_types::pera_system_state::PeraSystemState;
use pera_types::{
    crypto::AuthorityKeyPair,
    error::PeraError,
    messages_checkpoint::{CheckpointRequest, CheckpointResponse},
    transaction::{CertifiedTransaction, Transaction, VerifiedTransaction},
};
use pera_types::{
    effects::TransactionEffectsAPI,
    messages_checkpoint::{CheckpointRequestV2, CheckpointResponseV2},
};
use pera_types::{
    error::PeraResult,
    messages_grpc::{HandleCertificateRequestV3, HandleCertificateResponseV3},
};

#[derive(Clone, Copy, Default)]
pub struct LocalAuthorityClientFaultConfig {
    pub fail_before_handle_transaction: bool,
    pub fail_after_handle_transaction: bool,
    pub fail_before_handle_confirmation: bool,
    pub fail_after_handle_confirmation: bool,
    pub overload_retry_after_handle_transaction: Option<Duration>,
}

impl LocalAuthorityClientFaultConfig {
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

#[derive(Clone)]
pub struct LocalAuthorityClient {
    pub state: Arc<AuthorityState>,
    pub fault_config: LocalAuthorityClientFaultConfig,
}

#[async_trait]
impl AuthorityAPI for LocalAuthorityClient {
    async fn handle_transaction(
        &self,
        transaction: Transaction,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleTransactionResponse, PeraError> {
        if self.fault_config.fail_before_handle_transaction {
            return Err(PeraError::from("Mock error before handle_transaction"));
        }
        let state = self.state.clone();
        let epoch_store = self.state.load_epoch_store_one_call_per_task();
        let transaction = epoch_store
            .signature_verifier
            .verify_tx(transaction.data())
            .map(|_| VerifiedTransaction::new_from_verified(transaction))?;
        let result = state.handle_transaction(&epoch_store, transaction).await;
        if self.fault_config.fail_after_handle_transaction {
            return Err(PeraError::GenericAuthorityError {
                error: "Mock error after handle_transaction".to_owned(),
            });
        }
        if let Some(duration) = self.fault_config.overload_retry_after_handle_transaction {
            return Err(PeraError::ValidatorOverloadedRetryAfter {
                retry_after_secs: duration.as_secs(),
            });
        }
        result
    }

    async fn handle_certificate_v2(
        &self,
        certificate: CertifiedTransaction,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleCertificateResponseV2, PeraError> {
        let state = self.state.clone();
        let fault_config = self.fault_config;
        let request = HandleCertificateRequestV3 {
            certificate,
            include_events: true,
            include_input_objects: false,
            include_output_objects: false,
            include_auxiliary_data: false,
        };
        spawn_monitored_task!(Self::handle_certificate(state, request, fault_config))
            .await
            .unwrap()
            .map(|resp| HandleCertificateResponseV2 {
                signed_effects: resp.effects,
                events: resp.events.unwrap_or_default(),
                fastpath_input_objects: vec![],
            })
    }

    async fn handle_certificate_v3(
        &self,
        request: HandleCertificateRequestV3,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleCertificateResponseV3, PeraError> {
        let state = self.state.clone();
        let fault_config = self.fault_config;
        spawn_monitored_task!(Self::handle_certificate(state, request, fault_config))
            .await
            .unwrap()
    }

    async fn handle_soft_bundle_certificates_v3(
        &self,
        _request: HandleSoftBundleCertificatesRequestV3,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleSoftBundleCertificatesResponseV3, PeraError> {
        unimplemented!()
    }

    async fn handle_object_info_request(
        &self,
        request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, PeraError> {
        let state = self.state.clone();
        state.handle_object_info_request(request).await
    }

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, PeraError> {
        let state = self.state.clone();
        state.handle_transaction_info_request(request).await
    }

    async fn handle_checkpoint(
        &self,
        request: CheckpointRequest,
    ) -> Result<CheckpointResponse, PeraError> {
        let state = self.state.clone();

        state.handle_checkpoint_request(&request)
    }

    async fn handle_checkpoint_v2(
        &self,
        request: CheckpointRequestV2,
    ) -> Result<CheckpointResponseV2, PeraError> {
        let state = self.state.clone();

        state.handle_checkpoint_request_v2(&request)
    }

    async fn handle_system_state_object(
        &self,
        _request: SystemStateRequest,
    ) -> Result<PeraSystemState, PeraError> {
        self.state.get_pera_system_state_object_for_testing()
    }
}

impl LocalAuthorityClient {
    pub async fn new(secret: AuthorityKeyPair, genesis: &Genesis) -> Self {
        let state = TestAuthorityBuilder::new()
            .with_genesis_and_keypair(genesis, &secret)
            .build()
            .await;
        Self {
            state,
            fault_config: LocalAuthorityClientFaultConfig::default(),
        }
    }

    pub fn new_from_authority(state: Arc<AuthorityState>) -> Self {
        Self {
            state,
            fault_config: LocalAuthorityClientFaultConfig::default(),
        }
    }

    // One difference between this implementation and actual certificate execution, is that
    // this assumes shared object locks have already been acquired and tries to execute shared
    // object transactions as well as owned object transactions.
    async fn handle_certificate(
        state: Arc<AuthorityState>,
        request: HandleCertificateRequestV3,
        fault_config: LocalAuthorityClientFaultConfig,
    ) -> Result<HandleCertificateResponseV3, PeraError> {
        if fault_config.fail_before_handle_confirmation {
            return Err(PeraError::GenericAuthorityError {
                error: "Mock error before handle_confirmation_transaction".to_owned(),
            });
        }
        // Check existing effects before verifying the cert to allow querying certs finalized
        // from previous epochs.
        let tx_digest = *request.certificate.digest();
        let epoch_store = state.epoch_store_for_testing();
        let signed_effects = match state
            .get_signed_effects_and_maybe_resign(&tx_digest, &epoch_store)
        {
            Ok(Some(effects)) => effects,
            _ => {
                let certificate = epoch_store
                    .signature_verifier
                    .verify_cert(request.certificate)
                    .await?;
                //let certificate = certificate.verify(epoch_store.committee())?;
                state.enqueue_certificates_for_execution(vec![certificate.clone()], &epoch_store);
                let effects = state.notify_read_effects(&certificate).await?;
                state.sign_effects(effects, &epoch_store)?
            }
        }
        .into_inner();

        let events = if request.include_events {
            if let Some(digest) = signed_effects.events_digest() {
                Some(state.get_transaction_events(digest)?)
            } else {
                None
            }
        } else {
            None
        };

        if fault_config.fail_after_handle_confirmation {
            return Err(PeraError::GenericAuthorityError {
                error: "Mock error after handle_confirmation_transaction".to_owned(),
            });
        }

        let input_objects = request
            .include_input_objects
            .then(|| state.get_transaction_input_objects(&signed_effects))
            .and_then(Result::ok);

        let output_objects = request
            .include_output_objects
            .then(|| state.get_transaction_output_objects(&signed_effects))
            .and_then(Result::ok);

        Ok(HandleCertificateResponseV3 {
            effects: signed_effects,
            events,
            input_objects,
            output_objects,
            auxiliary_data: None, // We don't have any aux data generated presently
        })
    }
}

#[derive(Clone)]
pub struct MockAuthorityApi {
    delay: Duration,
    count: Arc<Mutex<u32>>,
    handle_object_info_request_result: Option<PeraResult<ObjectInfoResponse>>,
}

impl MockAuthorityApi {
    pub fn new(delay: Duration, count: Arc<Mutex<u32>>) -> Self {
        MockAuthorityApi {
            delay,
            count,
            handle_object_info_request_result: None,
        }
    }

    pub fn set_handle_object_info_request(&mut self, result: PeraResult<ObjectInfoResponse>) {
        self.handle_object_info_request_result = Some(result);
    }
}

#[async_trait]
impl AuthorityAPI for MockAuthorityApi {
    /// Initiate a new transaction to a Pera or Primary account.
    async fn handle_transaction(
        &self,
        _transaction: Transaction,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleTransactionResponse, PeraError> {
        unimplemented!();
    }

    /// Execute a certificate.
    async fn handle_certificate_v2(
        &self,
        _certificate: CertifiedTransaction,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleCertificateResponseV2, PeraError> {
        unimplemented!()
    }

    async fn handle_certificate_v3(
        &self,
        _request: HandleCertificateRequestV3,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleCertificateResponseV3, PeraError> {
        unimplemented!()
    }

    async fn handle_soft_bundle_certificates_v3(
        &self,
        _request: HandleSoftBundleCertificatesRequestV3,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleSoftBundleCertificatesResponseV3, PeraError> {
        unimplemented!()
    }

    /// Handle Object information requests for this account.
    async fn handle_object_info_request(
        &self,
        _request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, PeraError> {
        self.handle_object_info_request_result.clone().unwrap()
    }

    /// Handle Object information requests for this account.
    async fn handle_transaction_info_request(
        &self,
        request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, PeraError> {
        let count = {
            let mut count = self.count.lock().unwrap();
            *count += 1;
            *count
        };

        // timeout until the 15th request
        if count < 15 {
            tokio::time::sleep(self.delay).await;
        }

        Err(PeraError::TransactionNotFound {
            digest: request.transaction_digest,
        })
    }

    async fn handle_checkpoint(
        &self,
        _request: CheckpointRequest,
    ) -> Result<CheckpointResponse, PeraError> {
        unimplemented!();
    }

    async fn handle_checkpoint_v2(
        &self,
        _request: CheckpointRequestV2,
    ) -> Result<CheckpointResponseV2, PeraError> {
        unimplemented!();
    }

    async fn handle_system_state_object(
        &self,
        _request: SystemStateRequest,
    ) -> Result<PeraSystemState, PeraError> {
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct HandleTransactionTestAuthorityClient {
    pub tx_info_resp_to_return: PeraResult<HandleTransactionResponse>,
    pub cert_resp_to_return: PeraResult<HandleCertificateResponseV2>,
    // If set, sleep for this duration before responding to a request.
    // This is useful in testing a timeout scenario.
    pub sleep_duration_before_responding: Option<Duration>,
}

#[async_trait]
impl AuthorityAPI for HandleTransactionTestAuthorityClient {
    async fn handle_transaction(
        &self,
        _transaction: Transaction,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleTransactionResponse, PeraError> {
        if let Some(duration) = self.sleep_duration_before_responding {
            tokio::time::sleep(duration).await;
        }
        self.tx_info_resp_to_return.clone()
    }

    async fn handle_certificate_v2(
        &self,
        _certificate: CertifiedTransaction,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleCertificateResponseV2, PeraError> {
        if let Some(duration) = self.sleep_duration_before_responding {
            tokio::time::sleep(duration).await;
        }
        self.cert_resp_to_return.clone()
    }

    async fn handle_certificate_v3(
        &self,
        _request: HandleCertificateRequestV3,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleCertificateResponseV3, PeraError> {
        unimplemented!()
    }

    async fn handle_soft_bundle_certificates_v3(
        &self,
        _request: HandleSoftBundleCertificatesRequestV3,
        _client_addr: Option<SocketAddr>,
    ) -> Result<HandleSoftBundleCertificatesResponseV3, PeraError> {
        unimplemented!()
    }

    async fn handle_object_info_request(
        &self,
        _request: ObjectInfoRequest,
    ) -> Result<ObjectInfoResponse, PeraError> {
        unimplemented!()
    }

    async fn handle_transaction_info_request(
        &self,
        _request: TransactionInfoRequest,
    ) -> Result<TransactionInfoResponse, PeraError> {
        unimplemented!()
    }

    async fn handle_checkpoint(
        &self,
        _request: CheckpointRequest,
    ) -> Result<CheckpointResponse, PeraError> {
        unimplemented!()
    }

    async fn handle_checkpoint_v2(
        &self,
        _request: CheckpointRequestV2,
    ) -> Result<CheckpointResponseV2, PeraError> {
        unimplemented!()
    }

    async fn handle_system_state_object(
        &self,
        _request: SystemStateRequest,
    ) -> Result<PeraSystemState, PeraError> {
        unimplemented!()
    }
}

impl HandleTransactionTestAuthorityClient {
    pub fn new() -> Self {
        Self {
            tx_info_resp_to_return: Err(PeraError::Unknown("".to_string())),
            cert_resp_to_return: Err(PeraError::Unknown("".to_string())),
            sleep_duration_before_responding: None,
        }
    }

    pub fn set_tx_info_response(&mut self, resp: HandleTransactionResponse) {
        self.tx_info_resp_to_return = Ok(resp);
    }

    pub fn set_tx_info_response_error(&mut self, error: PeraError) {
        self.tx_info_resp_to_return = Err(error);
    }

    pub fn reset_tx_info_response(&mut self) {
        self.tx_info_resp_to_return = Err(PeraError::Unknown("".to_string()));
    }

    pub fn set_cert_resp_to_return(&mut self, resp: HandleCertificateResponseV2) {
        self.cert_resp_to_return = Ok(resp);
    }

    pub fn set_cert_resp_to_return_error(&mut self, error: PeraError) {
        self.cert_resp_to_return = Err(error);
    }

    pub fn reset_cert_response(&mut self) {
        self.cert_resp_to_return = Err(PeraError::Unknown("".to_string()));
    }

    pub fn set_sleep_duration_before_responding(&mut self, duration: Duration) {
        self.sleep_duration_before_responding = Some(duration);
    }
}

impl Default for HandleTransactionTestAuthorityClient {
    fn default() -> Self {
        Self::new()
    }
}
