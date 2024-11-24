use crate::base_types::{AuthorityName, EpochId, ObjectID};
use crate::dwallet_mpc::MPCSessionStatus;

#[derive(thiserror::Error, Debug)]
pub enum DwalletMPCError {
    // #[error("error occurred in the 2PCMPC process: {0}")]
    // TwoPCMPCError(#[from] twopc_mpc::Error),
    #[error(
        "received a `Finalize` event for session ID `{session_id:?}` that is not in the finalizing state; current state: {status}"
    )]
    InvalidFinalizeState {
        session_id: ObjectID,
        status: MPCSessionStatus,
    },

    #[error("received a `Finalize` event for session ID `{0}` that does not exist")]
    FinalizeEventSessionNotFound(ObjectID),

    #[error("mpc session with ID `{session_id:?}` was not found")]
    MPCSessionNotFound { session_id: ObjectID },

    #[error("mpc session with ID `{session_id:?}`, failed: {error}")]
    MPCSessionError { session_id: ObjectID, error: String },

    #[error("Operations for the epoch {0} have ended")]
    EpochEnded(EpochId),

    #[error("non MPC event")]
    NonMPCEvent,

    #[error("authority with a name: `{0}` not found")]
    AuthorityNameNotFound(AuthorityName),

    #[error("message de/serialization error occurred in the dWallet MPC process: {0}")]
    BcsError(#[from] bcs::Error),

    // Note:
    // this one actually takes mpc_error,
    // but because of poor error design in the underline lib we can't use it,
    // since there are generic implementations
    // that conflict with generic implementations in the current lib.
    #[error("error occurred in the dWallet MPC advance process: {0}")]
    AdvanceError(String),

    #[error("received an invalid/unknown MPC party type")]
    InvalidMPCPartyType,
}

/// A wrapper type for the result of a runtime operation.
pub type DwalletMPCResult<T> = Result<T, DwalletMPCError>;
