use dwallet_mpc_types::dwallet_mpc::DwalletNetworkMPCError;
use group::PartyID;
use sui_types::base_types::{EpochId, ObjectID};

#[derive(thiserror::Error, Debug, Clone)]
pub enum DwalletMPCError {
    #[error("mpc session with ID `{session_id:?}` was not found")]
    MPCSessionNotFound { session_id: ObjectID },

    #[error("sign state for the session with ID `{session_id:?}` was not found")]
    AggregatedSignStateNotFound { session_id: ObjectID },

    #[error("mpc session with ID `{session_id:?}`, failed: {error}")]
    MPCSessionError { session_id: ObjectID, error: String },

    #[error("Operations for the epoch {0} have ended")]
    EpochEnded(EpochId),

    #[error("non MPC event {0}")]
    NonMPCEvent(String),

    #[error("authority with a name: `{0}` not found")]
    AuthorityNameNotFound(crate::crypto::AuthorityName),

    #[error("authority with a name: `{0}` not found")]
    AuthorityIndexNotFound(PartyID),

    #[error("message de/serialization error occurred in the dWallet MPC process: {0}")]
    BcsError(#[from] bcs::Error),

    #[error("received an invalid/unknown MPC party type")]
    InvalidMPCPartyType,

    #[error("malicious parties have been detected: {0:?}")]
    MaliciousParties(Vec<PartyID>),

    #[error("session failed with malicious parties: {0:?}")]
    SessionFailedWithMaliciousParties(Vec<PartyID>),

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

    #[error("the MPC class groups decryption share missing for the party ID: {0}")]
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

    #[error("network decryption key is not ready for use")]
    NetworkDecryptionKeyNotReady,

    #[error("failed to lock the mutex")]
    LockError,

    #[error("verification of the encrypted user share failed")]
    EncryptedUserShareVerificationFailed,

    #[error("the sent public key does not match the sender's address")]
    EncryptedUserSharePublicKeyDoesNotMatchAddress,

    #[error(transparent)]
    DwalletNetworkMPCError(#[from] DwalletNetworkMPCError),

    #[error("error in Class Groups: {0}")]
    ClassGroupsError(String),

    #[error("failed to read Class Groups key: {0}")]
    FailedToReadCGKey(String),

    #[error("failed to write Class Groups key: {0}")]
    FailedToWriteCGKey(String),

    #[error("missing MPC private session input")]
    MissingMPCPrivateInput,

    #[error("failed to deserialize party public key: {0}")]
    InvalidPartyPublicKey(#[from] fastcrypto::error::FastCryptoError),

    #[error("failed to read the network decryption key shares")]
    DwalletMPCNetworkKeysNotFound,

    #[error("failed to verify signature: {0}")]
    SignatureVerificationFailed(String),

    #[error("failed to get available parallelism: {0}")]
    FailedToGetAvailableParallelism(String),

    #[error("the local machine has insufficient CPU cores to run a node")]
    InsufficientCPUCores,

    #[error("failed de/serialize json: {0:?}")]
    SerdeError(serde_json::error::Category),

    #[error("failed to find the presign round data")]
    PresignRoundDataNotFound,

    #[error("unsupported network DKG key scheme")]
    UnsupportedNetworkDKGKeyScheme,

    #[error("the first MPC step should not not receive any messages from the other parties")]
    MessageForFirstMPCStep,

    #[error("failed to find the event driven data")]
    MissingEventDrivenData,

    #[error("class groups key pair not found")]
    ClassGroupsKeyPairNotFound,

    #[error("network DKG key has not been completed yet")]
    NetworkDKGNotCompleted,

    #[error("failed to find the validator with ID: {0}")]
    ValidatorIDNotFound(ObjectID),

    #[error("{0}")]
    IkaError(#[from] crate::error::IkaError),

    #[error(
        "decryption key epoch out of sync: {key_id:?} expected epoch: {expected_epoch} but got: {actual_epoch}"
    )]
    DecryptionKeyEpochMismatch {
        key_id: ObjectID,
        expected_epoch: u64,
        actual_epoch: u64,
    },
}

/// A wrapper type for the result of a runtime operation.
pub type DwalletMPCResult<T> = Result<T, DwalletMPCError>;

impl From<serde_json::Error> for DwalletMPCError {
    fn from(err: serde_json::Error) -> Self {
        DwalletMPCError::SerdeError(err.classify())
    }
}
