use ika_types::sui::{DWalletCoordinatorInnerV1, SystemInner, SystemInnerV1};
use tracing::warn;

#[derive(Clone)]
pub struct CommonSuiObjectsSyncer {
    system_inner_receiver: tokio::sync::watch::Receiver<Option<SystemInnerV1>>,
    coordinator_inner_receiver: tokio::sync::watch::Receiver<Option<DWalletCoordinatorInnerV1>>,
    system_inner_sender: tokio::sync::watch::Sender<Option<SystemInnerV1>>,
    coordinator_inner_sender: tokio::sync::watch::Sender<Option<DWalletCoordinatorInnerV1>>,
}

impl CommonSuiObjectsSyncer {
    pub fn new() -> Self {
        let (system_inner_sender, system_inner_receiver) = tokio::sync::watch::channel(None);
        let (coordinator_inner_sender, coordinator_inner_receiver) =
            tokio::sync::watch::channel(None);

        Self {
            system_inner_receiver,
            coordinator_inner_receiver,
            system_inner_sender,
            coordinator_inner_sender,
        }
    }
    /// Retry every second until success
    pub async fn must_get_system_inner(&self) -> SystemInnerV1 {
        loop {
            match self.system_inner_receiver.borrow().clone() {
                Some(system_inner) => return system_inner,
                None => {
                    warn!("Waiting for system inner object...");
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    pub async fn must_get_coordinator_inner(&self) -> DWalletCoordinatorInnerV1 {
        loop {
            match self.coordinator_inner_receiver.borrow().clone() {
                Some(coordinator_inner) => return coordinator_inner,
                None => {
                    warn!("Waiting for coordinator inner object...");
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }
    }
}
