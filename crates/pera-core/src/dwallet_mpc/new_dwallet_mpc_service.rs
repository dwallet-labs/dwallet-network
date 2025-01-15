use std::sync::{Arc, Weak};
use narwhal_types::Round;
use pera_types::base_types::EpochId;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;

struct NewDWalletMPCService {
    last_read_narwhal_round: Round,
    read_messages: usize,
    epoch_store: Weak<AuthorityPerEpochStore>,
    epoch_id: EpochId,
}

impl NewDWalletMPCService {
    pub fn new(epoch_store: Arc<AuthorityPerEpochStore>) -> NewDWalletMPCService {
        Self {
            last_read_narwhal_round: 0,
            read_messages: 0,
            epoch_store: Arc::downgrade(&epoch_store),
        }
    }

    pub fn spawn(&self) {
        let new_messages = self
            .epoch_store().unwrap()
            .tables()?
            .dwallet_mpc_messages
            .unbounded_iter()
            .map(|(_, messages)| messages)
            .flatten()
            .collect()
    }

    fn epoch_store(&self) -> DwalletMPCResult<Arc<AuthorityPerEpochStore>> {
        self.epoch_store
            .upgrade()
            .ok_or(DwalletMPCError::EpochEnded(self.epoch_id))
    }
}