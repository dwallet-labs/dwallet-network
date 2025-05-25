//! This module handles the management of malicious actors in the MPC protocols.
//!
//! During an MPC session, parties attempting to disrupt the protocol
//! can be reported as malicious.
//! This module handles:
//! - Storing reported malicious actors.
//! - Ensuring these reports are only considered valid if submitted by a quorum of validators.
use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::stake_aggregator::StakeAggregator;
use group::PartyID;
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::dwallet_mpc_error::DwalletMPCResult;
use ika_types::messages_dwallet_mpc::MaliciousReport;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

/// A struct for managing malicious actors in MPC protocols.
///
/// This struct maintains a record of malicious actors reported by validators.
/// An actor is deemed malicious if it is reported by a quorum of validators.
/// Any message/output from these authorities will be ignored.
/// This list is maintained during the Epoch.
/// This happens automatically because the `MaliciousHandler` is part of the `
pub(crate) struct MaliciousHandler {
    committee: Arc<Committee>,
    /// The set of malicious actors that are reported by the validators.
    malicious_actors: HashSet<AuthorityName>,
    /// The reports of the malicious actors that are disrupting the MPC process.
    /// Maps the [`MaliciousReport`] to the set of authorities
    /// that reported the malicious actor.
    reports: HashMap<MaliciousReport, StakeAggregator<(), true>>,
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
    pub(crate) fn new(committee: Arc<Committee>) -> Self {
        Self {
            committee,
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
        let report_votes = self
            .reports
            .entry(report.clone())
            .or_insert(StakeAggregator::new(self.committee.clone()));
        if report_votes.has_quorum() {
            return Ok(ReportStatus::OverQuorum);
        }
        if report_votes
            .insert_generic(authority, ())
            .is_quorum_reached()
        {
            self.malicious_actors.extend(report.malicious_actors);
            return Ok(ReportStatus::QuorumReached);
        }
        Ok(ReportStatus::WaitingForQuorum)
    }

    pub(crate) fn is_malicious_actor(&self, authority: &AuthorityName) -> bool {
        self.malicious_actors.contains(authority)
    }

    pub(crate) fn get_malicious_actors_names(&self) -> &HashSet<AuthorityName> {
        &self.malicious_actors
    }

    // todo(zeev): fix this.
    #[allow(dead_code)]
    pub(crate) fn get_malicious_actors_ids(
        &self,
        epoch_store: Arc<AuthorityPerEpochStore>,
    ) -> DwalletMPCResult<HashSet<PartyID>> {
        Ok(self
            .malicious_actors
            .iter()
            .map(|name| Ok(epoch_store.authority_name_to_party_id(name)?))
            .collect::<DwalletMPCResult<HashSet<_>>>()?)
    }

    /// Reports a malicious actor disrupting the MPC process.
    /// Reported by the validator itself.
    pub(crate) fn report_malicious_actors(&mut self, authorities: &[AuthorityName]) {
        self.malicious_actors.extend(authorities);
    }
}
