// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::pin::Pin;
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use crate::epoch::reconfiguration::ReconfigurationInitiator;
use async_trait::async_trait;
use std::sync::Arc;
use fastcrypto::ed25519::Ed25519Signature;
use fastcrypto::traits::Signer;
use sui_types::base_types::AuthorityName;
use sui_types::error::SuiResult;
use sui_types::message_envelope::Message;
use sui_types::messages_checkpoint::{
    CertifiedCheckpointSummary, CheckpointContents, CheckpointSignatureMessage, CheckpointSummary,
    SignedCheckpointSummary, VerifiedCheckpoint,
};
use sui_types::messages_consensus::ConsensusTransaction;
use tracing::{debug, info, instrument, trace};
use sui_types::crypto::{AuthoritySignature, NetworkKeyPair};
use sui_types::messages_signature_mpc::{SignatureMPCOutput, SignatureMPCMessage, SignatureMPCMessageSummary, SignedSignatureMPCOutput, SignedSignatureMPCMessageSummary};
use crate::authority::StableSyncAuthoritySigner;

use super::SignatureMPCMetrics;

#[async_trait]
pub trait SubmitSignatureMPC: Sync + Send + 'static {
    async fn sign_and_submit_message(
        &self,
        summary: &SignatureMPCMessageSummary,
        epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> SuiResult;

    async fn sign_and_submit_output(
        &self,
        output: &SignatureMPCOutput,
        epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> SuiResult;
}

//pub type StableSyncAuthoritySigner = Pin<Arc<dyn Signer<Ed25519Signature> + Send + Sync>>;

pub struct SubmitSignatureMPCToConsensus<T> {
    pub sender: T,
    pub signer: StableSyncAuthoritySigner,
    pub authority: AuthorityName,
    pub next_reconfiguration_timestamp_ms: u64,
    pub metrics: Arc<SignatureMPCMetrics>,
}

#[async_trait]
impl<T: SubmitToConsensus + ReconfigurationInitiator> SubmitSignatureMPC
    for SubmitSignatureMPCToConsensus<T>
{
    #[instrument(level = "debug", skip_all)]
    async fn sign_and_submit_message(
        &self,
        summary: &SignatureMPCMessageSummary,
        epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> SuiResult {
        // let checkpoint_timestamp = summary.timestamp_ms;
        // self.metrics.checkpoint_creation_latency_ms.observe(
        //     summary
        //         .timestamp()
        //         .elapsed()
        //         .unwrap_or_default()
        //         .as_millis() as u64,
        // );
        // debug!(
        //     "Sending checkpoint signature at sequence {checkpoint_seq} to consensus, timestamp {checkpoint_timestamp}.
        //     {}ms left till end of epoch at timestamp {}",
        //     self.next_reconfiguration_timestamp_ms.saturating_sub(checkpoint_timestamp), self.next_reconfiguration_timestamp_ms
        // );

        let summary = SignedSignatureMPCMessageSummary::new(
            epoch_store.epoch(),
            summary.clone(),
            &*self.signer,
            self.authority,
        );

        let message = SignatureMPCMessage { summary };
        let transaction = ConsensusTransaction::new_signature_mpc_message(message);
        self.sender
            .submit_to_consensus(&transaction, epoch_store)
            .await?;
        // self.metrics
        //     .last_sent_checkpoint_signature
        //     .set(checkpoint_seq as i64);
        // if checkpoint_timestamp >= self.next_reconfiguration_timestamp_ms {
        //     // close_epoch is ok if called multiple times
        //     self.sender.close_epoch(epoch_store);
        // }
        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    async fn sign_and_submit_output(
        &self,
        output: &SignatureMPCOutput,
        epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> SuiResult {
        // let checkpoint_timestamp = summary.timestamp_ms;
        // self.metrics.checkpoint_creation_latency_ms.observe(
        //     summary
        //         .timestamp()
        //         .elapsed()
        //         .unwrap_or_default()
        //         .as_millis() as u64,
        // );
        // debug!(
        //     "Sending checkpoint signature at sequence {checkpoint_seq} to consensus, timestamp {checkpoint_timestamp}.
        //     {}ms left till end of epoch at timestamp {}",
        //     self.next_reconfiguration_timestamp_ms.saturating_sub(checkpoint_timestamp), self.next_reconfiguration_timestamp_ms
        // );

        let message = SignedSignatureMPCOutput::new(
            epoch_store.epoch(),
            output.clone(),
            &*self.signer,
            self.authority,
        );

        let transaction = ConsensusTransaction::new_signature_mpc_dkg_output(message);
        self.sender
            .submit_to_consensus(&transaction, epoch_store)
            .await?;
        // self.metrics
        //     .last_sent_checkpoint_signature
        //     .set(checkpoint_seq as i64);
        // if checkpoint_timestamp >= self.next_reconfiguration_timestamp_ms {
        //     // close_epoch is ok if called multiple times
        //     self.sender.close_epoch(epoch_store);
        // }
        Ok(())
    }
}