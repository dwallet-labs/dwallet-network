use crate::types::base64::Base64;
use async_graphql::Object;
use pera_types::messages_dwallet_mpc::SignatureMPCOutput as NativeSignatureMPCOutput;

/// System transaction to store the output of signature mpc on-chain.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct SignatureMPCOutputTransaction {
    /// The native representation of the transaction arguments
    pub native: NativeSignatureMPCOutput,
    /// The checkpoint sequence number this transaction was viewed at.
    pub checkpoint_viewed_at: u64,
}

#[Object]
impl SignatureMPCOutputTransaction {
    /// The output value of the signature mpc session
    async fn value(&self) -> Vec<u8> {
        self.native.value.clone()
    }

    /// The session ID
    async fn session_id(&self) -> Base64 {
        Base64::from(self.native.session_id.to_vec().as_slice())
    }

    /// The address of the session initiator
    async fn sender_address(&self) -> Base64 {
        Base64::from(self.native.sender_address.to_vec().as_slice())
    }
}
