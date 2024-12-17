use crate::base_types::{AuthorityName, EpochId, ObjectID};
use crate::dwallet_mpc::DWalletMPCNetworkKey;
use group::PartyID;
// todo(zeev): remove unused errors.

#[derive(thiserror::Error, Debug)]
pub enum DwalletMPCError {
    #[error("received a `Finalize` event for session ID `{0}` that does not exist")]
    FinalizeEventSessionNotFound(ObjectID),

    #[error("mpc session with ID `{session_id:?}` was not found")]
    MPCSessionNotFound { session_id: ObjectID },

    #[error("mpc session with ID `{session_id:?}`, failed: {error}")]
    MPCSessionError { session_id: ObjectID, error: String },

    #[error("failed to create an MPC message for party ID: {0}")]
    MPCMessageError(PartyID),

    #[error("Operations for the epoch {0} have ended")]
    EpochEnded(EpochId),

    #[error("non MPC event")]
    NonMPCEvent,

    #[error("authority with a name: `{0}` not found")]
    AuthorityNameNotFound(AuthorityName),

    #[error("authority with a name: `{0}` not found")]
    AuthorityIndexNotFound(PartyID),

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

    #[error("malicious parties have been detected: {0:?}")]
    MaliciousParties(Vec<PartyID>),

    #[error("dWallet MPC Manager error: {0}")]
    MPCManagerError(String),

    #[error("missing MPC class groups decryption shares in config")]
    MissingDwalletMPCClassGroupsDecryptionShares,

    #[error("missing DWallet MPC outputs manager")]
    MissingDwalletMPCOutputsManager,

    #[error("MPC class groups decryption share missing for the party ID: {0}")]
    DwalletMPCClassGroupsDecryptionShareMissing(PartyID),

    #[error("missing MPC public parameters in config")]
    MissingDwalletMPCDecryptionSharesPublicParameters,

    #[error("tried to start DKG on an epoch that is not the first one")]
    DKGNotOnFirstEpoch,

    // Note:
    // this one actually takes mpc_error,
    // but because of poor error design in the underline lib we can't use it,
    // since there are generic implementations
    // that conflict with generic implementations in the current lib.
    #[error("TwoPC MPC error: {0}")]
    TwoPCMPCError(String),
    // // todo(zeev): fix the errors.
    // #[error("TwoPC MPC check error: {0}")]
    // TwoPCMPCCheckError(#[from] twopc_mpc::Error),
    #[error("failed to find a message in batch: {0:?}")]
    MissingMessageInBatch(Vec<u8>),

    #[error("wrong epoch access {0}")]
    WrongEpoch(u64),

    #[error("missing encrypted decryption key shares in the config")]
    MissingEncryptionOfDecryptionKeyShares,

    #[error("missing dwallet mpc network key version")]
    MissingKeyVersion,

    #[error("MPC instance missing private output")]
    InstanceMissingPrivateOutput,

    #[error("invalid dWallet MPC network key")]
    InvalidDWalletMPCNetworkKey,

    #[error("failed to lock the mutex")]
    LockError,
}

/// A wrapper type for the result of a runtime operation.
pub type DwalletMPCResult<T> = Result<T, DwalletMPCError>;
