// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::sync::Arc;

use async_trait::async_trait;
use tracing::instrument;

use sui_types::base_types::AuthorityName;
use sui_types::error::SuiResult;
use sui_types::messages_consensus::ConsensusTransaction;
use sui_types::messages_signature_mpc::{
    SignatureMPCMessage, SignatureMPCMessageSummary, SignatureMPCOutput,
    SignedSignatureMPCMessageSummary, SignedSignatureMPCOutput,
};

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::authority::StableSyncAuthoritySigner;
use crate::consensus_adapter::SubmitToConsensus;
use crate::epoch::reconfiguration::ReconfigurationInitiator;

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
        Ok(())
    }

    #[instrument(level = "debug", skip_all)]
    async fn sign_and_submit_output(
        &self,
        output: &SignatureMPCOutput,
        epoch_store: &Arc<AuthorityPerEpochStore>,
    ) -> SuiResult {
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
        Ok(())
    }
}
