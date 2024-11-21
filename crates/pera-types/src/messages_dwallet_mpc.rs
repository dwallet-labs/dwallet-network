use crate::base_types::{ObjectID, PeraAddress};
use crate::crypto::default_hash;
use crate::digests::DWalletMPCOutputDigest;
use crate::message_envelope::Message;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::fmt;
use crate::dwallet_mpc::MPCOutput;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Copy)]
pub enum MPCRound {
    DKGFirst,
    DKGSecond,
}

/// The content of the system transaction that stores the MPC session output on the chain.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DWalletMPCOutput {
    /// The session ID of the MPC session.
    pub session_id: ObjectID,
    /// The address of the initiating user.
    pub initiating_address: PeraAddress,
    pub dwallet_cap_id: ObjectID,
    pub mpc_round: MPCRound,
    /// The final value of the MPC session.
    pub value: MPCOutput,
}

impl fmt::Display for MPCRound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            MPCRound::DKGFirst => "create_dkg_first_round_output",
            MPCRound::DKGSecond => "create_dkg_second_round_output",
        };
        write!(f, "{}", output)
    }
}

impl Message for DWalletMPCOutput {
    type DigestType = DWalletMPCOutputDigest;
    const SCOPE: IntentScope = IntentScope::DWalletMPCOutput;

    fn digest(&self) -> Self::DigestType {
        DWalletMPCOutputDigest::new(default_hash(self))
    }
}
