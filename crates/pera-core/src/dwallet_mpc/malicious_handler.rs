//! This module is responsible for managing the malicious actors in the MPC protocols.
//! Every MPC session can report malicious parties that are trying to disrupt the protocol.
//! This module is responsible for storing the malicious actors if they are reported by quorum validators.

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::authority_name_to_party_id;
use group::PartyID;
use mpc::Weight;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::dwallet_mpc_error::DwalletMPCResult;
use pera_types::messages_dwallet_mpc::MaliciousReport;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tracing::error;

/// A struct to handle the malicious actors in the MPC protocols.
/// It stores the malicious actors that are reported by the validators.
/// If a quorum of validators report the same actor, it is considered malicious.
pub(crate) struct MaliciousHandler {
    /// The quorum threshold for the MPC process.
    quorum_threshold: StakeUnit,
    /// The set of malicious actors that are reported by the validators.
    malicious_actors: HashSet<AuthorityName>,
    /// The reports of the malicious actors that are disrupting the MPC process.
    /// Maps the [`MaliciousReport`] to the set of authorities that reported the malicious actor.
    reports: HashMap<MaliciousReport, HashSet<AuthorityName>>,
}

/// The status of the report after it is reported by the validators.
pub(crate) enum ReportStatus {
    WaitingForQuorum,
    QuorumReached,
    OverQuorum,
}

impl MaliciousHandler {
    pub(crate) fn new(quorum_threshold: StakeUnit) -> Self {
        Self {
            quorum_threshold,
            malicious_actors: HashSet::new(),
            reports: HashMap::new(),
        }
    }

    /// Reports malicious actors in the MPC process.
    /// If a quorum of validators report the same actor, it is considered malicious.
    /// Returns [`ReportStatus`]  the status of the report after it is reported by the validators.
    pub(crate) fn report_malicious_actor(
        &mut self,
        report: MaliciousReport,
        authority: AuthorityName,
    ) -> ReportStatus {
        if self.reports.contains_key(&report) {
            // Safe to unwrap because the key exists.
            let malicious_actors = self.reports.get_mut(&report).unwrap();
            if !malicious_actors.contains(&authority) {
                malicious_actors.insert(authority);
            } else {
                error!(
                    "Authority {} is already reported as malicious in session {}",
                    authority, report.session_id
                );
            }
        } else {
            let mut reporters = HashSet::new();
            reporters.insert(authority);
            self.reports.insert(report.clone(), reporters);
        }

        // Safe to unwrap because the key exists by now.
        let number_of_reports = self.reports.get(&report).unwrap().len();
        if number_of_reports == self.quorum_threshold as usize {
            self.malicious_actors.insert(authority);
            ReportStatus::QuorumReached
        } else if number_of_reports > self.quorum_threshold as usize {
            ReportStatus::OverQuorum
        } else {
            ReportStatus::WaitingForQuorum
        }
    }

    pub(crate) fn is_malicious_actor(&self, authority: &AuthorityName) -> bool {
        self.malicious_actors.contains(authority)
    }

    pub(crate) fn get_malicious_actors_names(&self) -> &HashSet<AuthorityName> {
        &self.malicious_actors
    }

    pub(crate) fn get_malicious_actors_ids(
        &self,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> DwalletMPCResult<HashSet<PartyID>> {
        Ok(self
            .malicious_actors
            .iter()
            .map(|name| Ok(authority_name_to_party_id(name, &epoch_store)?))
            .collect::<DwalletMPCResult<HashSet<_>>>()?)
    }

    /// Reports a malicious actor that is disrupting the MPC process.
    /// Reported by the validator itself.
    pub(crate) fn report_malicious_internal(&mut self, authority: AuthorityName) {
        self.malicious_actors.insert(authority);
    }
}
