use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use narwhal_types::Round;
use pera_types::base_types::EpochId;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use std::sync::{Arc, Weak};
use futures::future::{select, Either};
use futures::FutureExt;
use tokio::select;
use tokio::sync::Notify;
use tokio::sync::watch::Receiver;
use typed_store::Map;

pub struct DWalletMPCService {
    last_read_narwhal_round: Round,
    read_messages: usize,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    notify: Arc<Notify>,
}

impl DWalletMPCService {
    pub fn new(epoch_store: Arc<AuthorityPerEpochStore>) -> Self {
        Self {
            last_read_narwhal_round: 0,
            read_messages: 0,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id: epoch_store.epoch(),
            notify: Arc::new(Notify::new()),
        }
    }

    pub async fn spawn(&mut self, mut exit_receiver: Receiver<()>) {
        loop {
            let epoch_store = self
                .epoch_store().unwrap();
            let arc = epoch_store
                .tables()
                .unwrap();
            let new_dwallet_messages_iter = arc
                .dwallet_mpc_messages
                .iter_with_bounds(Some(self.last_read_narwhal_round), None);
            let mut new_messages = vec![];
            for (round, messages) in new_dwallet_messages_iter {
                self.last_read_narwhal_round = round;
                new_messages.extend(messages);
            }
            for message in new_messages.into_iter() {
                epoch_store
                    .send_message_to_dwallet_mpc_manager(message)
                    .await;
            }

            match select(exit_receiver.changed().boxed(), self.notify.notified().boxed()).await {
                Either::Left(_) => {
                    // break loop on exit signal
                    break;
                }
                Either::Right(_) => {}
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }
}
