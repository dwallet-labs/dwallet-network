// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use super::{CheckpointMetrics, CheckpointStore};
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::authority::StableSyncAuthoritySigner;
use crate::consensus_adapter::SubmitToConsensus;
use crate::epoch::reconfiguration::ReconfigurationInitiator;
use crate::sui_connector::CheckpointMessageSuiNotify;
use async_trait::async_trait;
use fastcrypto::traits::ToFromBytes;
use ika_types::crypto::AuthorityName;
use ika_types::error::IkaResult;
use ika_types::message_envelope::Message;
use ika_types::messages_checkpoint::{
    CertifiedCheckpointMessage, CheckpointContents, CheckpointMessage, CheckpointSignatureMessage,
    SignedCheckpointMessage, VerifiedCheckpointMessage,
};
use ika_types::messages_consensus::ConsensusTransaction;
use itertools::Itertools;
use std::sync::Arc;
use tracing::{debug, info, instrument, trace};

#[async_trait]
pub trait CheckpointOutput: Sync + Send + 'static {
    async fn checkpoint_created(
        &self,
        summary: &CheckpointMessage,
        contents: &CheckpointContents,
        epoch_store: &Arc<AuthorityPerEpochStore>,
        checkpoint_store: &Arc<CheckpointStore>,
    ) -> IkaResult;
}

#[async_trait]
pub trait CertifiedCheckpointMessageOutput: Sync + Send + 'static {
    async fn certified_checkpoint_message_created(
        &self,
        summary: &CertifiedCheckpointMessage,
    ) -> IkaResult;
}

pub struct SubmitCheckpointToConsensus<T> {
    pub sender: T,
    pub signer: StableSyncAuthoritySigner,
    pub authority: AuthorityName,
    pub next_reconfiguration_timestamp_ms: u64,
    pub metrics: Arc<CheckpointMetrics>,
}

pub struct LogCheckpointOutput;

impl LogCheckpointOutput {
    pub fn boxed() -> Box<dyn CheckpointOutput> {
        Box::new(Self)
    }

    pub fn boxed_certified() -> Box<dyn CertifiedCheckpointMessageOutput> {
        Box::new(Self)
    }
}

#[async_trait]
impl<T: SubmitToConsensus + ReconfigurationInitiator> CheckpointOutput
    for SubmitCheckpointToConsensus<T>
{
    #[instrument(level = "debug", skip_all)]
    async fn checkpoint_created(
        &self,
        summary: &CheckpointMessage,
        contents: &CheckpointContents,
        epoch_store: &Arc<AuthorityPerEpochStore>,
        checkpoint_store: &Arc<CheckpointStore>,
    ) -> IkaResult {
        LogCheckpointOutput
            .checkpoint_created(summary, contents, epoch_store, checkpoint_store)
            .await?;

        let checkpoint_timestamp = summary.timestamp_ms;
        let checkpoint_seq = summary.sequence_number;
        self.metrics.checkpoint_creation_latency.observe(
            summary
                .timestamp()
                .elapsed()
                .unwrap_or_default()
                .as_secs_f64(),
        );
        self.metrics.checkpoint_creation_latency_ms.observe(
            summary
                .timestamp()
                .elapsed()
                .unwrap_or_default()
                .as_millis() as u64,
        );

        let highest_verified_checkpoint = checkpoint_store
            .get_highest_verified_checkpoint()?
            .map(|x| *x.sequence_number());

        if Some(checkpoint_seq) > highest_verified_checkpoint {
            debug!(
                "Sending checkpoint signature at sequence {checkpoint_seq} to consensus, timestamp {checkpoint_timestamp}.
                {}ms left till end of epoch at timestamp {}",
                self.next_reconfiguration_timestamp_ms.saturating_sub(checkpoint_timestamp), self.next_reconfiguration_timestamp_ms
            );

            let summary = SignedCheckpointMessage::new(
                epoch_store.epoch(),
                summary.clone(),
                &*self.signer,
                self.authority,
            );

            let message = CheckpointSignatureMessage { checkpoint_message: summary };
            let transaction = ConsensusTransaction::new_checkpoint_signature_message(message);
            self.sender
                .submit_to_consensus(&vec![transaction], epoch_store)
                .await?;
            self.metrics
                .last_sent_checkpoint_signature
                .set(checkpoint_seq as i64);
        } else {
            debug!(
                "Checkpoint at sequence {checkpoint_seq} is already certified, skipping signature submission to consensus",
            );
            self.metrics
                .last_skipped_checkpoint_signature_submission
                .set(checkpoint_seq as i64);
        }

        if checkpoint_timestamp >= self.next_reconfiguration_timestamp_ms {
            // close_epoch is ok if called multiple times
            self.sender.close_epoch(epoch_store);
        }
        Ok(())
    }
}

#[async_trait]
impl CheckpointOutput for LogCheckpointOutput {
    async fn checkpoint_created(
        &self,
        summary: &CheckpointMessage,
        contents: &CheckpointContents,
        _epoch_store: &Arc<AuthorityPerEpochStore>,
        _checkpoint_store: &Arc<CheckpointStore>,
    ) -> IkaResult {
        trace!(
            "Including following transactions in checkpoint {}: {:?}",
            summary.sequence_number,
            contents
        );
        info!(
            "Creating checkpoint {:?} at epoch {}, sequence {}, transactions count {}, content digest {:?}",
            summary.digest(),
            summary.epoch,
            summary.sequence_number,
            contents.size(),
            summary.actions,
        );

        Ok(())
    }
}

#[async_trait]
impl CertifiedCheckpointMessageOutput for LogCheckpointOutput {
    async fn certified_checkpoint_message_created(
        &self,
        summary: &CertifiedCheckpointMessage,
    ) -> IkaResult {
        info!(
            "Certified checkpoint with sequence {} and digest {}",
            summary.sequence_number,
            summary.digest()
        );
        Ok(())
    }
}

pub struct SendCheckpointToStateSync {
    handle: ika_network::state_sync::Handle,
}

impl SendCheckpointToStateSync {
    pub fn new(handle: ika_network::state_sync::Handle) -> Self {
        Self { handle }
    }
}

#[async_trait]
impl CertifiedCheckpointMessageOutput for SendCheckpointToStateSync {
    #[instrument(level = "debug", skip_all)]
    async fn certified_checkpoint_message_created(
        &self,
        checkpoint_message: &CertifiedCheckpointMessage,
    ) -> IkaResult {
        info!(
            "Certified checkpoint with sequence {} and digest {}",
            checkpoint_message.sequence_number,
            checkpoint_message.digest(),
        );
        self.handle
            .send_checkpoint(VerifiedCheckpointMessage::new_unchecked(checkpoint_message.to_owned()))
            .await;

        Ok(())
    }
}