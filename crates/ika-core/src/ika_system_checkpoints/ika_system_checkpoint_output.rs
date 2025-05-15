// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{IkaSystemCheckpointMetrics, IkaSystemCheckpointStore};
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::authority::StableSyncAuthoritySigner;
use crate::consensus_adapter::SubmitToConsensus;
use async_trait::async_trait;
use ika_types::crypto::AuthorityName;
use ika_types::error::IkaResult;
use ika_types::message_envelope::Message;
use ika_types::messages_consensus::ConsensusTransaction;
use ika_types::messages_ika_system_checkpoints::{
    CertifiedIkaSystemCheckpoint, IkaSystemCheckpoint, IkaSystemCheckpointSignatureMessage,
    SignedIkaSystemCheckpoint, VerifiedIkaSystemCheckpoint,
};
use std::sync::Arc;
use tracing::{debug, info, instrument, trace};

#[async_trait]
pub trait IkaSystemCheckpointOutput: Sync + Send + 'static {
    async fn ika_system_checkpoint_created(
        &self,
        summary: &IkaSystemCheckpoint,
        epoch_store: &Arc<AuthorityPerEpochStore>,
        ika_system_checkpoint_store: &Arc<IkaSystemCheckpointStore>,
    ) -> IkaResult;
}

#[async_trait]
pub trait CertifiedIkaSystemCheckpointOutput: Sync + Send + 'static {
    async fn certified_ika_system_checkpoint_created(
        &self,
        summary: &CertifiedIkaSystemCheckpoint,
    ) -> IkaResult;
}

pub struct SubmitIkaSystemCheckpointToConsensus<T> {
    pub sender: T,
    pub signer: StableSyncAuthoritySigner,
    pub authority: AuthorityName,
    pub metrics: Arc<IkaSystemCheckpointMetrics>,
}

pub struct LogIkaSystemCheckpointOutput;

impl LogIkaSystemCheckpointOutput {
    pub fn boxed() -> Box<dyn IkaSystemCheckpointOutput> {
        Box::new(Self)
    }

    pub fn boxed_certified() -> Box<dyn CertifiedIkaSystemCheckpointOutput> {
        Box::new(Self)
    }
}

#[async_trait]
impl<T: SubmitToConsensus> IkaSystemCheckpointOutput for SubmitIkaSystemCheckpointToConsensus<T> {
    #[instrument(level = "debug", skip_all)]
    async fn ika_system_checkpoint_created(
        &self,
        ika_system_checkpoint: &IkaSystemCheckpoint,
        epoch_store: &Arc<AuthorityPerEpochStore>,
        ika_system_checkpoint_store: &Arc<IkaSystemCheckpointStore>,
    ) -> IkaResult {
        LogIkaSystemCheckpointOutput
            .ika_system_checkpoint_created(
                ika_system_checkpoint,
                epoch_store,
                ika_system_checkpoint_store,
            )
            .await?;

        let ika_system_checkpoint_timestamp = ika_system_checkpoint.timestamp_ms;
        let ika_system_checkpoint_seq = ika_system_checkpoint.sequence_number;
        self.metrics.ika_system_checkpoint_creation_latency.observe(
            ika_system_checkpoint
                .timestamp()
                .elapsed()
                .unwrap_or_default()
                .as_secs_f64(),
        );

        let highest_verified_ika_system_checkpoint = ika_system_checkpoint_store
            .get_highest_verified_ika_system_checkpoint()?
            .map(|x| *x.sequence_number());

        if Some(ika_system_checkpoint_seq) > highest_verified_ika_system_checkpoint {
            debug!(
                "Sending ika_system_checkpoint signature at sequence {ika_system_checkpoint_seq} to consensus, timestamp {ika_system_checkpoint_timestamp}."
            );

            let summary = SignedIkaSystemCheckpoint::new(
                epoch_store.epoch(),
                ika_system_checkpoint.clone(),
                &*self.signer,
                self.authority,
            );

            let message = IkaSystemCheckpointSignatureMessage {
                ika_system_checkpoint: summary,
            };
            let transaction =
                ConsensusTransaction::new_ika_system_checkpoint_signature_message(message);
            self.sender
                .submit_to_consensus(&vec![transaction], epoch_store)
                .await?;
            self.metrics
                .last_sent_ika_system_checkpoint_signature
                .set(ika_system_checkpoint_seq as i64);
        } else {
            debug!(
                "IkaSystemCheckpoint at sequence {ika_system_checkpoint_seq} is already certified, skipping signature submission to consensus",
            );
            self.metrics
                .last_skipped_ika_system_checkpoint_signature_submission
                .set(ika_system_checkpoint_seq as i64);
        }

        Ok(())
    }
}
// ?
#[async_trait]
impl IkaSystemCheckpointOutput for LogIkaSystemCheckpointOutput {
    async fn ika_system_checkpoint_created(
        &self,
        ika_system_checkpoint: &IkaSystemCheckpoint,
        _epoch_store: &Arc<AuthorityPerEpochStore>,
        _ika_system_checkpoint_store: &Arc<IkaSystemCheckpointStore>,
    ) -> IkaResult {
        trace!(
            "Including following transactions in ika_system_checkpoint {}: {:#?}",
            ika_system_checkpoint.sequence_number,
            ika_system_checkpoint.messages,
        );
        info!(
            "Creating ika_system_checkpoint {:?} at epoch {}, sequence {}, messages count {}",
            ika_system_checkpoint.digest(),
            ika_system_checkpoint.epoch,
            ika_system_checkpoint.sequence_number,
            ika_system_checkpoint.messages.len(),
        );

        Ok(())
    }
}

#[async_trait]
impl CertifiedIkaSystemCheckpointOutput for LogIkaSystemCheckpointOutput {
    async fn certified_ika_system_checkpoint_created(
        &self,
        summary: &CertifiedIkaSystemCheckpoint,
    ) -> IkaResult {
        info!(
            "Certified ika_system_checkpoint with sequence {} and digest {}",
            summary.sequence_number,
            summary.digest()
        );
        Ok(())
    }
}

pub struct SendIkaSystemCheckpointToStateSync {
    handle: ika_network::state_sync::Handle,
}

impl SendIkaSystemCheckpointToStateSync {
    pub fn new(handle: ika_network::state_sync::Handle) -> Self {
        Self { handle }
    }
}

#[async_trait]
impl CertifiedIkaSystemCheckpointOutput for SendIkaSystemCheckpointToStateSync {
    #[instrument(level = "debug", skip_all)]
    async fn certified_ika_system_checkpoint_created(
        &self,
        ika_system_checkpoint: &CertifiedIkaSystemCheckpoint,
    ) -> IkaResult {
        info!(
            "Certified ika_system_checkpoint with sequence {} and digest {}",
            ika_system_checkpoint.sequence_number,
            ika_system_checkpoint.digest(),
        );
        self.handle
            .send_ika_system_checkpoint(VerifiedIkaSystemCheckpoint::new_unchecked(
                ika_system_checkpoint.to_owned(),
            ))
            .await;

        Ok(())
    }
}
