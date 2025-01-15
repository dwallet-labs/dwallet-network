use std::sync::{Arc, Weak};
use narwhal_types::Round;
use pera_types::base_types::EpochId;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use typed_store::Map;
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;

struct NewDWalletMPCService {
    last_read_narwhal_round: Round,
    read_messages: usize,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
}

impl NewDWalletMPCService {
    pub fn new(epoch_store: Arc<AuthorityPerEpochStore>) -> Self {
        Self {
            last_read_narwhal_round: 0,
            read_messages: 0,
            epoch_store: Arc::downgrade(&epoch_store),
            epoch_id: epoch_store.epoch(),
        }
    }

    pub fn spawn(&self) {
        let new_messages = self
            .epoch_store().unwrap()
            .tables().unwrap()
            .dwallet_mpc_messages
            .iter_with_bounds(
                Some(self.last_read_narwhal_round),
                None,
            )
            .map(|(_, messages)| messages)
            .flatten()
            .collect();

        // Update the MPC manager with the new messages.
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }
}