//! MPC output transaction code & types declarations to support the GraphQL API.
use crate::types::base64::Base64;
use async_graphql::Object;
use pera_types::messages_dwallet_mpc::DWalletMPCOutput as NativeDWalletMPCOutput;

/// System transaction to store the output of dwallet mpc on-chain.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct DWalletMPCOutputTransaction {
    /// The native representation of the transaction arguments.
    pub native: NativeDWalletMPCOutput,
    /// The checkpoint sequence number this transaction was viewed at.
    pub checkpoint_viewed_at: u64,
}

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct LockNextCommitteeTransaction();

#[Object]
impl DWalletMPCOutputTransaction {
    /// The output value of the dwallet mpc session.
    async fn value(&self) -> Vec<u8> {
        self.native.output.clone()
    }

    /// The session ID.
    async fn session_id(&self) -> Base64 {
        Base64::from(self.native.session_info.session_id.to_vec().as_slice())
    }

    /// The address of the session initiator.
    async fn sender_address(&self) -> Base64 {
        Base64::from(
            self.native
                .session_info
                .initiating_user_address
                .to_vec()
                .as_slice(),
        )
    }
}

#[Object]
impl LockNextCommitteeTransaction {
    async fn value(&self) -> String {
        "LockNextCommitteeTransaction".to_string()
    }
}
