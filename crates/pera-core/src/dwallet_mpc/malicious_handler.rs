//! This module handles the management of malicious actors in the MPC protocols.
//!
//! During an MPC session, parties attempting to disrupt the protocol
//! can be reported as malicious.
//! This module handles:
//! - Storing reported malicious actors.
//! - Ensuring these reports are only considered valid if submitted by a quorum of validators.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::dwallet_mpc::authority_name_to_party_id;
use group::PartyID;
use mpc::Weight;
use narwhal_config::Export;
use pera_types::base_types::{AuthorityName, ObjectID};
use pera_types::committee::StakeUnit;
use pera_types::dwallet_mpc_error::{DwalletMPCError, DwalletMPCResult};
use pera_types::messages_dwallet_mpc::MaliciousReport;
use std::collections::{hash_map, HashMap, HashSet};
use std::sync::Arc;
use tracing::error;

/// A struct for managing malicious actors in MPC protocols.
///
/// This struct maintains a record of malicious actors reported by validators.
/// An actor is deemed malicious if it is reported by a quorum of validators.
/// Any message/output from these authorities will be ignored.
/// This list is maintained during the Epoch.
/// This happens automatically because the `MaliciousHandler` is part of the `
pub(crate) struct MaliciousHandler {
    /// The quorum threshold for the MPC process.
    quorum_threshold: StakeUnit,
    /// A mapping between an authority name to its stake.
    pub weighted_parties: HashMap<AuthorityName, StakeUnit>,
    /// The set of malicious actors that are reported by the validators.
    malicious_actors: HashSet<AuthorityName>,
    /// The reports of the malicious actors that are disrupting the MPC process.
    /// Maps the [`MaliciousReport`] to the set of authorities
    /// that reported the malicious actor.
    reports: HashMap<MaliciousReport, HashSet<AuthorityName>>,
}

/// The status of the report after it is reported by the validators.
pub(crate) enum ReportStatus {
    /// The report is waiting for a quorum of validators to report the same actors.
    WaitingForQuorum,
    /// Quorum has been reached, the actor is considered malicious,
    /// handles the report.
    QuorumReached,
    /// The case where a Quorum has been reached before,
    /// prevent duplicate reports.
    OverQuorum,
}

impl MaliciousHandler {
    pub(crate) fn new(
        quorum_threshold: StakeUnit,
        weighted_parties: HashMap<AuthorityName, StakeUnit>,
    ) -> Self {
        Self {
            quorum_threshold,
            weighted_parties,
            malicious_actors: HashSet::new(),
            reports: HashMap::new(),
        }
    }

    /// Reports malicious actors in the MPC process.
    /// If a quorum of validators reports the same actor, it is considered malicious.
    /// Returns [`ReportStatus`] the status of the report after
    /// it is reported by the validators.
    pub(crate) fn report_malicious_actor(
        &mut self,
        report: MaliciousReport,
        authority: AuthorityName,
    ) -> DwalletMPCResult<ReportStatus> {
        let authority_voting_weight = self
            .weighted_parties
            .get(&authority)
            .ok_or(DwalletMPCError::AuthorityNameNotFound(authority))?
            .clone() as usize;

        match self.reports.entry(report.clone()) {
            hash_map::Entry::Occupied(mut entry) => {
                if !entry.get_mut().insert(authority) {
                    error!("authority {} already reported {:?}", authority, report);
                }
            }
            hash_map::Entry::Vacant(entry) => {
                let mut reporters = HashSet::new();
                reporters.insert(authority);
                entry.insert(reporters);
            }
        }

        let total_voting_weight = self.calculate_total_voting_weight(report);
        let has_reached_quorum = total_voting_weight >= self.quorum_threshold as usize;
        let above_quorum = total_voting_weight > self.quorum_threshold as usize;
        let first_quorum_reached =
            total_voting_weight - authority_voting_weight < self.quorum_threshold as usize;
        if has_reached_quorum && first_quorum_reached {
            self.malicious_actors.insert(authority);
            Ok(ReportStatus::QuorumReached)
        } else if above_quorum {
            Ok(ReportStatus::OverQuorum)
        } else {
            Ok(ReportStatus::WaitingForQuorum)
        }
    }

    fn calculate_total_voting_weight(&self, report: MaliciousReport) -> usize {
        let mut total_voting_weight = 0;
        if let Some(reporters) = self.reports.get(&report) {
            for authority in reporters {
                if let Some(weight) = self.weighted_parties.get(authority) {
                    total_voting_weight += *weight as usize;
                }
            }
        }
        total_voting_weight
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

    /// Reports malicious actors that are disrupting the MPC process.
    /// Reported by the validator itself.
    pub(crate) fn report_malicious_actors(&mut self, authorities: &[AuthorityName]) {
        self.malicious_actors.extend(authorities);
    }
}
