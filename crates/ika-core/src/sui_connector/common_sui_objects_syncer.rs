use ika_types::sui::{DWalletCoordinatorInnerV1, SystemInner, SystemInnerV1};
use tracing::warn;

pub struct CommonSuiObjectsSyncer {
    system_inner_receiver: tokio::sync::watch::Receiver<Option<SystemInnerV1>>,
    coordinator_inner_receiver: tokio::sync::watch::Receiver<Option<DWalletCoordinatorInnerV1>>,
}

impl CommonSuiObjectsSyncer {
    pub fn new(
        system_inner_receiver: tokio::sync::watch::Receiver<Option<SystemInnerV1>>,
        coordinator_inner_receiver: tokio::sync::watch::Receiver<Option<DWalletCoordinatorInnerV1>>,
    ) -> Self {
        Self {
            system_inner_receiver,
            coordinator_inner_receiver,
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
