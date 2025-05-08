// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{PeerHeights, StateSync, StateSyncMessage};
use anemo::{rpc::Status, types::response::StatusCode, Request, Response, Result};
use dashmap::DashMap;
use futures::future::BoxFuture;
use ika_types::digests::ChainIdentifier;
use ika_types::message::MessageKind;
use ika_types::{
    digests::{CheckpointContentsDigest, CheckpointMessageDigest},
    messages_checkpoint::{
        CertifiedDWalletCheckpointMessage, CheckpointSequenceNumber, VerifiedCheckpointMessage,
    },
    storage::WriteStore,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::task::{Context, Poll};
use tokio::sync::{mpsc, OwnedSemaphorePermit, Semaphore};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash, Copy)]
pub enum GetCheckpointMessageRequest {
    ByDigest(CheckpointMessageDigest),
    BySequenceNumber(CheckpointSequenceNumber),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetCheckpointAvailabilityResponse {
    pub(crate) highest_synced_checkpoint: Option<CertifiedDWalletCheckpointMessage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetChainIdentifierResponse {
    pub(crate) chain_identifier: ChainIdentifier,
}

pub(super) struct Server<S> {
    pub(super) store: S,
    pub(super) peer_heights: Arc<RwLock<PeerHeights>>,
    pub(super) sender: mpsc::WeakSender<StateSyncMessage>,
    pub(crate) chain_identifier: ChainIdentifier,
}

#[anemo::async_trait]
impl<S> StateSync for Server<S>
where
    S: WriteStore + Send + Sync + 'static,
{
    async fn push_checkpoint_message(
        &self,
        request: Request<CertifiedDWalletCheckpointMessage>,
    ) -> Result<Response<()>, Status> {
        let peer_id = request
            .peer_id()
            .copied()
            .ok_or_else(|| Status::internal("unable to query sender's PeerId"))?;

        let checkpoint = request.into_inner();
        if !self
            .peer_heights
            .write()
            .unwrap()
            .update_peer_info(peer_id, checkpoint.clone())
        {
            return Ok(Response::new(()));
        }

        let highest_verified_checkpoint = self
            .store
            .get_highest_verified_checkpoint()
            .map_err(|e| Status::internal(e.to_string()))?;

        let should_sync = highest_verified_checkpoint
            .map(|c| *checkpoint.sequence_number() > c.sequence_number)
            .unwrap_or(true);

        // If this checkpoint is higher than our highest verified checkpoint notify the
        // event loop to potentially sync it
        if should_sync {
            if let Some(sender) = self.sender.upgrade() {
                sender.send(StateSyncMessage::StartSyncJob).await.unwrap();
            }
        }

        Ok(Response::new(()))
    }

    async fn get_checkpoint_message(
        &self,
        request: Request<GetCheckpointMessageRequest>,
    ) -> Result<Response<Option<CertifiedDWalletCheckpointMessage>>, Status> {
        let checkpoint = match request.inner() {
            GetCheckpointMessageRequest::ByDigest(digest) => {
                self.store.get_checkpoint_by_digest(digest)
            }
            GetCheckpointMessageRequest::BySequenceNumber(sequence_number) => self
                .store
                .get_checkpoint_by_sequence_number(*sequence_number),
        }
        .map_err(|e| Status::internal(e.to_string()))?
        .map(VerifiedCheckpointMessage::into_inner);

        Ok(Response::new(checkpoint))
    }

    async fn get_checkpoint_availability(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetCheckpointAvailabilityResponse>, Status> {
        let highest_synced_checkpoint = self
            .store
            .get_highest_synced_checkpoint()
            .map_err(|e| Status::internal(e.to_string()))?
            .map(VerifiedCheckpointMessage::into_inner);

        Ok(Response::new(GetCheckpointAvailabilityResponse {
            highest_synced_checkpoint,
        }))
    }

    async fn get_chain_identifier(
        &self,
        _request: Request<()>,
    ) -> Result<Response<GetChainIdentifierResponse>, Status> {
        Ok(Response::new(GetChainIdentifierResponse {
            chain_identifier: self.chain_identifier,
        }))
    }
}

/// [`Layer`] for adding a per-checkpoint limit to the number of inflight GetCheckpointContent
/// requests.
#[derive(Clone)]
pub(super) struct CheckpointMessageDownloadLimitLayer {
    inflight_per_checkpoint: Arc<DashMap<GetCheckpointMessageRequest, Arc<Semaphore>>>,
    max_inflight_per_checkpoint: usize,
}

impl CheckpointMessageDownloadLimitLayer {
    pub(super) fn new(max_inflight_per_checkpoint: usize) -> Self {
        Self {
            inflight_per_checkpoint: Arc::new(DashMap::new()),
            max_inflight_per_checkpoint,
        }
    }

    pub(super) fn maybe_prune_map(&self) {
        const PRUNE_THRESHOLD: usize = 5000;
        if self.inflight_per_checkpoint.len() >= PRUNE_THRESHOLD {
            self.inflight_per_checkpoint.retain(|_, semaphore| {
                semaphore.available_permits() < self.max_inflight_per_checkpoint
            });
        }
    }
}

impl<S> tower::layer::Layer<S> for CheckpointMessageDownloadLimitLayer {
    type Service = CheckpointMessageDownloadLimit<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CheckpointMessageDownloadLimit {
            inner,
            inflight_per_checkpoint: self.inflight_per_checkpoint.clone(),
            max_inflight_per_checkpoint: self.max_inflight_per_checkpoint,
        }
    }
}

/// Middleware for adding a per-checkpoint limit to the number of inflight GetCheckpointContent
/// requests.
#[derive(Clone)]
pub(super) struct CheckpointMessageDownloadLimit<S> {
    inner: S,
    inflight_per_checkpoint: Arc<DashMap<GetCheckpointMessageRequest, Arc<Semaphore>>>,
    max_inflight_per_checkpoint: usize,
}

impl<S> tower::Service<Request<GetCheckpointMessageRequest>> for CheckpointMessageDownloadLimit<S>
where
    S: tower::Service<
            Request<GetCheckpointMessageRequest>,
            Response = Response<Option<CertifiedDWalletCheckpointMessage>>,
            Error = Status,
        >
        + 'static
        + Clone
        + Send,
    <S as tower::Service<Request<GetCheckpointMessageRequest>>>::Future: Send,
    Request<GetCheckpointMessageRequest>: 'static + Send + Sync,
{
    type Response = Response<Option<CertifiedDWalletCheckpointMessage>>;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<GetCheckpointMessageRequest>) -> Self::Future {
        let inflight_per_checkpoint = self.inflight_per_checkpoint.clone();
        let max_inflight_per_checkpoint = self.max_inflight_per_checkpoint;
        let mut inner = self.inner.clone();

        let fut = async move {
            let semaphore = {
                let semaphore_entry = inflight_per_checkpoint
                    .entry(*req.body())
                    .or_insert_with(|| Arc::new(Semaphore::new(max_inflight_per_checkpoint)));
                semaphore_entry.value().clone()
            };
            let permit = semaphore.try_acquire_owned().map_err(|e| match e {
                tokio::sync::TryAcquireError::Closed => {
                    anemo::rpc::Status::new(StatusCode::InternalServerError)
                }
                tokio::sync::TryAcquireError::NoPermits => {
                    anemo::rpc::Status::new(StatusCode::TooManyRequests)
                }
            })?;

            struct SemaphoreExtension(#[allow(unused)] OwnedSemaphorePermit);
            inner.call(req).await.map(move |mut response| {
                // Insert permit as extension so it's not dropped until the response is sent.
                response
                    .extensions_mut()
                    .insert(Arc::new(SemaphoreExtension(permit)));
                response
            })
        };
        Box::pin(fut)
    }
}
