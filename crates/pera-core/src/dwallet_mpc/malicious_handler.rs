//! This module is responsible for managing the malicious actors in the MPC protocols.
//! Every MPC session can report malicious parties that are trying to disrupt the protocol.
//! This module is responsible for storing the malicious actors if they are reported by quorum validators.

use group::PartyID;
use mpc::Weight;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::dwallet_mpc_error::DwalletMPCResult;
use pera_types::messages_dwallet_mpc::MaliciousReport;
use std::collections::{HashMap, HashSet};
use tracing::error;

pub(crate) struct MaliciousHandler {
    /// The quorum threshold for the MPC process.
    quorum_threshold: StakeUnit,
    /// The set of malicious actors that are reported by the validators.
    malicious_actors: HashSet<AuthorityName>,
    /// The reports of the malicious actors that are disrupting the MPC process.
    reports: HashMap<MaliciousReport, HashSet<AuthorityName>>,
}

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
                error!("Authority {} is already reported as malicious in session {} on consensus round {}", authority, report.session_id, report.consensus_round);
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

    pub(crate) fn get_malicious_actors(&self) -> &HashSet<AuthorityName> {
        &self.malicious_actors
    }

    pub(crate) fn report_malicious_internal(&mut self, authority: AuthorityName) {
        self.malicious_actors.insert(authority);
    }
}
