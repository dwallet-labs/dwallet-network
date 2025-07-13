use crate::dwallet_checkpoints::PendingDWalletCheckpointV1;
use crate::dwallet_mpc::LOG_DIR;
use class_groups::SecretKeyShareSizedInteger;
use commitment::CommitmentSizedNumber;
use dwallet_mpc_types::dwallet_mpc::{MPCMessage, MPCPrivateInput};
use group::PartyID;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::DwalletMPCError;
use ika_types::messages_dwallet_mpc::MPCSessionRequest;
use mpc::WeightedThresholdAccessStructure;
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::warn;

/// A struct to encapsulate MPC session logging parameters and functionality.
/// This separates logging-specific concerns from the core MPC advancement logic.
#[derive(Default, Clone)]
pub(crate) struct MPCSessionLogger {
    /// The MPC protocol name for logging purposes
    pub mpc_protocol_name: Option<String>,
    /// Mapping from party IDs to authority names for logging
    pub party_to_authority_map: Option<HashMap<PartyID, AuthorityName>>,
    /// Encoded class groups key pair and proof for logging
    pub encoded_class_groups_key_pair_and_proof: Option<MPCPrivateInput>,
    /// Decryption key shares for logging
    pub decryption_key_shares: Option<HashMap<PartyID, SecretKeyShareSizedInteger>>,
    /// Malicious parties detected during message processing
    pub malicious_parties: Option<Vec<PartyID>>,
}

impl MPCSessionLogger {
    /// Creates a new MPCSessionLogger with the provided parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the MPC protocol name
    pub fn with_protocol_name(mut self, name: String) -> Self {
        self.mpc_protocol_name = Some(name);
        self
    }

    /// Sets the party to authority mapping
    pub fn with_party_to_authority_map(mut self, map: HashMap<PartyID, AuthorityName>) -> Self {
        self.party_to_authority_map = Some(map);
        self
    }

    /// Sets the encoded class groups key pair and proof
    pub fn with_class_groups_key_pair_and_proof(mut self, proof: MPCPrivateInput) -> Self {
        self.encoded_class_groups_key_pair_and_proof = Some(proof);
        self
    }

    /// Sets the decryption key shares
    pub fn with_decryption_key_shares(
        mut self,
        shares: HashMap<PartyID, SecretKeyShareSizedInteger>,
    ) -> Self {
        self.decryption_key_shares = Some(shares);
        self
    }

    /// Sets the malicious parties
    pub fn with_malicious_parties(mut self, parties: Vec<PartyID>) -> Self {
        self.malicious_parties = Some(parties);
        self
    }

    /// Writes MPC session logs to disk if logging is enabled
    pub fn write_logs_to_disk(
        &self,
        session_id: CommitmentSizedNumber,
        party_id: PartyID,
        access_structure: &WeightedThresholdAccessStructure,
        messages: &HashMap<usize, HashMap<PartyID, MPCMessage>>,
    ) {
        if std::env::var("IKA_WRITE_MPC_SESSION_LOGS_TO_DISK").unwrap_or_default() != "1" {
            return;
        }

        if std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() % 60 == 0)
            .unwrap_or(false)
        {
            warn!("Writing MPC session logs to disk");
        }

        // Determine round number
        let round = messages.len();

        // Get (and initialize once) the log directory
        let log_dir = match self.get_log_dir() {
            Ok(dir) => dir,
            Err(err) => {
                warn!(?err, "Failed to get the logs directory");
                return;
            }
        };
        let filename = format!("session_{}_round_{}.json", session_id, round);
        let path = log_dir.join(&filename);

        // Serialize to JSON.
        let log = json!({
            "session_id": session_id,
            "round": round,
            "party_id": party_id,
            "access_structure": access_structure,
            "messages": messages,
            "mpc_protocol": self.mpc_protocol_name,
            "party_to_authority_map": self.party_to_authority_map,
            "class_groups_key_pair_and_proof": self.encoded_class_groups_key_pair_and_proof,
            "decryption_key_shares": self.decryption_key_shares,
            "malicious_parties": self.malicious_parties,
        });

        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to create log file {}: {}", path.display(), e);
                return;
            }
        };
        if let Err(e) = file.write_all(log.to_string().as_bytes()) {
            warn!("Failed to write to the log file {}: {}", path.display(), e);
        }
    }

    /// Writes MPC session logs to disk if logging is enabled
    #[allow(dead_code)]
    pub fn write_output_to_disk(
        &self,
        session_id: CommitmentSizedNumber,
        party_id: PartyID,
        output_sender_party_id: PartyID,
        output: &[u8],
        session_request: &MPCSessionRequest,
        round: u64,
        idx: usize,
    ) {
        if std::env::var("IKA_WRITE_MPC_OUTPUTS_TO_DISK").unwrap_or_default() != "1" {
            return;
        }
        if std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() % 60 == 0)
            .unwrap_or(false)
        {
            warn!("Writing MPC session output to disk");
        }
        // Get (and initialize once) the log directory
        let log_dir = match self.get_log_dir() {
            Ok(dir) => dir,
            Err(err) => {
                warn!(?err, "Failed to get the logs directory");
                return;
            }
        };
        let filename =
            format!("{round}_{idx}_session_{session_id}_from_{output_sender_party_id}.json",);
        let path = log_dir.join(&filename);

        // Serialize to JSON.
        let log = json!({
            "session_id": session_id,
            "party_id": party_id,
            "mpc_protocol": self.mpc_protocol_name,
            "party_to_authority_map": self.party_to_authority_map,
            "output": output,
            "session_request": session_request.clone(),
        });

        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to create log file {}: {}", path.display(), e);
                return;
            }
        };
        if let Err(e) = file.write_all(log.to_string().as_bytes()) {
            warn!("Failed to write to the log file {}: {}", path.display(), e);
        }
    }

    /// Writes MPC session logs to disk if logging is enabled
    pub fn write_pending_checkpoint(&self, pending_checkpoint: &PendingDWalletCheckpointV1) {
        if std::env::var("IKA_WRITE_PENDING_CHECKPOINTS").unwrap_or_default() != "1" {
            return;
        }

        if std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() % 60 == 0)
            .unwrap_or(false)
        {
            warn!("Writing MPC pending checkpoint to disk");
        }

        // Get (and initialize once) the log directory
        let log_dir = match self.get_log_dir() {
            Ok(dir) => dir,
            Err(err) => {
                warn!(?err, "Failed to get the logs directory");
                return;
            }
        };
        let filename = format!(
            "pending_checkpoint_{}.json",
            pending_checkpoint.details.checkpoint_height
        );
        let path = log_dir.join(&filename);

        // Serialize to JSON.
        let log = json!({
            "pending_checkpoint": pending_checkpoint
        });

        let mut file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => {
                warn!("Failed to create log file {}: {}", path.display(), e);
                return;
            }
        };
        if let Err(e) = file.write_all(log.to_string().as_bytes()) {
            warn!("Failed to write to the log file {}: {}", path.display(), e);
        }
    }

    fn get_log_dir(&self) -> Result<&'static PathBuf, DwalletMPCError> {
        if let Some(dir) = LOG_DIR.get() {
            return Ok(dir);
        }

        // Otherwise, attempt creation
        const PRIMARY: &str = "/opt/ika/db/mpclogs/logs";
        const FALLBACK: &str = "/tmp/mpclogs/logs";

        let chosen = if fs::create_dir_all(PRIMARY).is_ok() {
            PRIMARY
        } else {
            // Primary failed → try fallback (propagate error if that fails).
            fs::create_dir_all(FALLBACK).map_err(|e| {
                DwalletMPCError::TwoPCMPCError(format!(
                    "Failed to create a fallback log directory {}: {}",
                    FALLBACK, e
                ))
            })?;
            FALLBACK
        };

        // Insert into our OnceLock (this only ever succeeds once).
        let pathbuf = PathBuf::from(chosen);
        LOG_DIR.set(pathbuf).map_err(|_| {
            DwalletMPCError::TwoPCMPCError("failed to set a global log directory".into())
        })?;

        // Safe to unwrap — we just set it
        Ok(LOG_DIR.get().unwrap())
    }
}
