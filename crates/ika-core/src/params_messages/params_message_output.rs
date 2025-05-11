// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{ParamsMessageMetrics, ParamsMessageStore};
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::authority::StableSyncAuthoritySigner;
use crate::consensus_adapter::SubmitToConsensus;
use async_trait::async_trait;
use ika_types::crypto::AuthorityName;
use ika_types::error::IkaResult;
use ika_types::message_envelope::Message;
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_params_messages::{
    CertifiedParamsMessage, ParamsMessage, ParamsMessageSignatureMessage, SignedParamsMessage,
    VerifiedParamsMessage,
};
use std::sync::Arc;
use tracing::{debug, info, instrument, trace};

#[async_trait]
pub trait ParamsMessageOutput: Sync + Send + 'static {
    async fn params_message_created(
        &self,
        summary: &ParamsMessage,
        epoch_store: &Arc<AuthorityPerEpochStore>,
        params_message_store: &Arc<ParamsMessageStore>,
    ) -> IkaResult;
}

#[async_trait]
pub trait CertifiedParamsMessageOutput: Sync + Send + 'static {
    async fn certified_params_message_created(&self, summary: &CertifiedParamsMessage)
        -> IkaResult;
}

pub struct SubmitParamsMessageToConsensus<T> {
    pub sender: T,
    pub signer: StableSyncAuthoritySigner,
    pub authority: AuthorityName,
    pub metrics: Arc<ParamsMessageMetrics>,
}

pub struct LogParamsMessageOutput;

impl LogParamsMessageOutput {
    pub fn boxed() -> Box<dyn ParamsMessageOutput> {
        Box::new(Self)
    }

    pub fn boxed_certified() -> Box<dyn CertifiedParamsMessageOutput> {
        Box::new(Self)
    }
}

#[async_trait]
impl<T: SubmitToConsensus> ParamsMessageOutput for SubmitParamsMessageToConsensus<T> {
    #[instrument(level = "debug", skip_all)]
    async fn params_message_created(
        &self,
        params_message: &ParamsMessage,
        epoch_store: &Arc<AuthorityPerEpochStore>,
        params_message_store: &Arc<ParamsMessageStore>,
    ) -> IkaResult {
        LogParamsMessageOutput
            .params_message_created(params_message, epoch_store, params_message_store)
            .await?;

        let params_message_timestamp = params_message.timestamp_ms;
        let params_message_seq = params_message.sequence_number;
        self.metrics.params_message_creation_latency.observe(
            params_message
                .timestamp()
                .elapsed()
                .unwrap_or_default()
                .as_secs_f64(),
        );

        let highest_verified_params_message = params_message_store
            .get_highest_verified_params_message()?
            .map(|x| *x.sequence_number());

        if Some(params_message_seq) > highest_verified_params_message {
            debug!(
                "Sending params_message signature at sequence {params_message_seq} to consensus, timestamp {params_message_timestamp}."
            );

            let summary = SignedParamsMessage::new(
                epoch_store.epoch(),
                params_message.clone(),
                &*self.signer,
                self.authority,
            );

            let message = ParamsMessageSignatureMessage {
                params_message: summary,
            };
            let transaction = ConsensusTransaction::new_params_message_signature_message(message);
            self.sender
                .submit_to_consensus(&vec![transaction], epoch_store)
                .await?;
            self.metrics
                .last_sent_params_message_signature
                .set(params_message_seq as i64);
        } else {
            debug!(
                "ParamsMessage at sequence {params_message_seq} is already certified, skipping signature submission to consensus",
            );
            self.metrics
                .last_skipped_params_message_signature_submission
                .set(params_message_seq as i64);
        }

        Ok(())
    }
}
// ?
#[async_trait]
impl ParamsMessageOutput for LogParamsMessageOutput {
    async fn params_message_created(
        &self,
        params_message: &ParamsMessage,
        _epoch_store: &Arc<AuthorityPerEpochStore>,
        _params_message_store: &Arc<ParamsMessageStore>,
    ) -> IkaResult {
        trace!(
            "Including following transactions in params_message {}: {:#?}",
            params_message.sequence_number,
            params_message.messages,
        );
        info!(
            "Creating params_message {:?} at epoch {}, sequence {}, messages count {}",
            params_message.digest(),
            params_message.epoch,
            params_message.sequence_number,
            params_message.messages.len(),
        );

        Ok(())
    }
}

#[async_trait]
impl CertifiedParamsMessageOutput for LogParamsMessageOutput {
    async fn certified_params_message_created(
        &self,
        summary: &CertifiedParamsMessage,
    ) -> IkaResult {
        info!(
            "Certified params_message with sequence {} and digest {}",
            summary.sequence_number,
            summary.digest()
        );
        Ok(())
    }
}

pub struct SendParamsMessageToStateSync {
    handle: ika_network::state_sync::Handle,
}

impl SendParamsMessageToStateSync {
    pub fn new(handle: ika_network::state_sync::Handle) -> Self {
        Self { handle }
    }
}

#[async_trait]
impl CertifiedParamsMessageOutput for SendParamsMessageToStateSync {
    #[instrument(level = "debug", skip_all)]
    async fn certified_params_message_created(
        &self,
        params_message: &CertifiedParamsMessage,
    ) -> IkaResult {
        info!(
            "Certified params_message with sequence {} and digest {}",
            params_message.sequence_number,
            params_message.digest(),
        );
        self.handle
            .send_params_message(VerifiedParamsMessage::new_unchecked(
                params_message.to_owned(),
            ))
            .await;

        Ok(())
    }
}
