use crate::base_types::{AuthorityName, EpochId, ObjectID};
use dwallet_mpc_types::dwallet_mpc::DwalletNetworkMPCError;
use group::PartyID;

#[derive(thiserror::Error, Debug)]
pub enum DwalletMPCError {
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

    #[error("authority with a name: `{0}` not found")]
    AuthorityIndexNotFound(PartyID),

    #[error("message de/serialization error occurred in the dWallet MPC process: {0}")]
    BcsError(#[from] bcs::Error),

    #[error("received an invalid/unknown MPC party type")]
    InvalidMPCPartyType,

    #[error("malicious parties have been detected: {0:?}")]
    MaliciousParties(Vec<PartyID>),

    #[error("dWallet MPC Manager error: {0}")]
    MPCManagerError(String),

    #[error("missing MPC class groups decryption shares in config")]
    MissingDwalletMPCClassGroupsDecryptionShares,

    #[error("missing DWallet MPC outputs verifier")]
    MissingDwalletMPCOutputsVerifier,

    #[error("missing DWallet MPC batches manager")]
    MissingDWalletMPCBatchesManager,

    #[error("missing dWallet MPC Sender")]
    MissingDWalletMPCSender,

    #[error("dwallet MPC Sender failed: {0}")]
    DWalletMPCSenderSendFailed(String),

    #[error("MPC class groups decryption share missing for the party ID: {0}")]
    DwalletMPCClassGroupsDecryptionShareMissing(PartyID),

    #[error("missing MPC public parameters in config")]
    MissingDwalletMPCDecryptionSharesPublicParameters,

    // Note:
    // this one actually takes mpc_error,
    // but because of poor error design in the underline lib we can't use it,
    // since there are generic implementations
    // that conflict with generic implementations in the current lib.
    #[error("TwoPC MPC error: {0}")]
    TwoPCMPCError(String),

    #[error("failed to find a message in batch: {0:?}")]
    MissingMessageInBatch(Vec<u8>),

    #[error("missing dwallet mpc decryption key shares")]
    MissingDwalletMPCDecryptionKeyShares,

    #[error("missing dwallet mpc network key version")]
    MissingKeyVersion,

    #[error("failed to lock the mutex")]
    LockError,

    #[error("verification of the encrypted user share failed")]
    EncryptedUserShareVerificationFailed,

    #[error(transparent)]
    DwalletNetworkMPCError(#[from] DwalletNetworkMPCError),
}

/// A wrapper type for the result of a runtime operation.
pub type DwalletMPCResult<T> = Result<T, DwalletMPCError>;
