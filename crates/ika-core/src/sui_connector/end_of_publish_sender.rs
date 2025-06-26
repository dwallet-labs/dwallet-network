use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::consensus_adapter::SubmitToConsensus;
use ika_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use ika_types::messages_consensus::ConsensusTransaction;
use std::sync::{Arc, Weak};
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tracing::error;

pub struct EndOfPublishSender {
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: u64,
    consensus_adapter: Arc<dyn SubmitToConsensus>,
    end_of_publish_receiver: Receiver<Option<u64>>,
}

impl EndOfPublishSender {
    pub fn new(
        epoch_store: Weak<AuthorityPerEpochStore>,
        consensus_adapter: Arc<dyn SubmitToConsensus>,
        end_of_publish_receiver: Receiver<Option<u64>>,
        epoch_id: u64,
    ) -> Self {
        Self {
            epoch_store,
            consensus_adapter,
            end_of_publish_receiver,
            epoch_id,
        }
    }

    pub async fn run(&self) {
        loop {
            if *self.end_of_publish_receiver.borrow() == Some(self.epoch_id) {
                if let Err(err) = self.send_end_of_publish().await {
                    error!(?err, "failed to send end of publish message");
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }

    async fn send_end_of_publish(&self) -> DwalletMPCResult<()> {
        let tx = ConsensusTransaction::new_end_of_publish(self.epoch_store()?.name);
        self.consensus_adapter
            .submit_to_consensus(&vec![tx], &self.epoch_store()?)
            .await?;
        Ok(())
    }
}
