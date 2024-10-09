use async_graphql::Object;
use pera_types::messages_signature_mpc::SignatureMPCOutput as NativeSignatureMPCOutput;
use crate::types::base64::Base64;


#[derive(Clone, Eq, PartialEq)]
pub(crate) struct SignatureMPCOutputTransaction {
    pub native: NativeSignatureMPCOutput,
    /// The checkpoint sequence number this was viewed at.
    pub checkpoint_viewed_at: u64,
}

/// System transaction to store the output of signature mpc dkg on-chain.
#[Object]
impl SignatureMPCOutputTransaction {
    async fn value(&self) -> Vec<Vec<u8>> {
        self.native.value.clone()
    }

    async fn session_id(&self) -> Base64 {
        Base64::from(self.native.session_id.to_vec().as_slice())
    }

    async fn sender_address(&self) -> Base64 {
        Base64::from(self.native.sender_address.to_vec().as_slice())
    }
}