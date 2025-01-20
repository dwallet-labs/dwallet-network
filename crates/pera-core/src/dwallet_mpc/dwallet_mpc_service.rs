///! This module contains the DWalletMPCService struct, which is responsible to read
/// DWallet MPC messages from the local DB every five seconds and forward them to the [`crate::dwallet_mpc::mpc_manager::DWalletMPCManager`].
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::mpc_manager::DWalletMPCDBMessage;
use narwhal_types::Round;
use pera_types::base_types::EpochId;
use std::sync::Arc;
use tokio::sync::watch::Receiver;
use tokio::sync::{watch, Notify};
use typed_store::Map;

pub struct DWalletMPCService {
    last_read_narwhal_round: Round,
    read_messages: usize,
    epoch_store: Arc<AuthorityPerEpochStore>,
    epoch_id: EpochId,
    notify: Arc<Notify>,
    pub exit: Receiver<()>,
}

impl DWalletMPCService {
    pub fn new(epoch_store: Arc<AuthorityPerEpochStore>, exit: watch::Receiver<()>) -> Self {
        Self {
            last_read_narwhal_round: 0,
            read_messages: 0,
            epoch_store: epoch_store.clone(),
            epoch_id: epoch_store.epoch(),
            notify: Arc::new(Notify::new()),
            exit,
        }
    }

    /// Spawns the DWallet MPC service, that
    /// read DWallet MPC messages from the local DB every five seconds and forward them to the
    /// [`crate::dwallet_mpc::mpc_manager::DWalletMPCManager`].
    ///
    /// The service exists upon an epoch switch.
    pub async fn spawn(&mut self) {
        'main: loop {
            match self.exit.has_changed() {
                Ok(true) | Err(_) => {
                    break;
                }
                Ok(false) => (),
            };
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

            let Ok(tables) = self.epoch_store.tables() else {
                continue 'main;
            };
            let new_dwallet_messages_iter = tables
                .dwallet_mpc_messages
                .iter_with_bounds(Some(self.last_read_narwhal_round + 1), None);
            let mut new_messages = vec![];
            for (round, messages) in new_dwallet_messages_iter {
                self.last_read_narwhal_round = round;
                new_messages.extend(messages);
            }
            let mut manager = self.epoch_store.get_dwallet_mpc_manager().await;
            for message in new_messages.into_iter() {
                manager.handle_dwallet_db_message(message).await;
            }
            manager
                .handle_dwallet_db_message(DWalletMPCDBMessage::PerformCryptographicComputations)
                .await;
        }
    }
}
