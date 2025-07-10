//! This module handles the management of malicious actors in the MPC protocols.
//!
//! During an MPC session, parties attempting to disrupt the protocol
//! can be reported as malicious.
//! This module handles:
//! - Storing reported malicious actors.
//! - Ensuring these reports are only considered valid if submitted by a quorum of validators.
use crate::stake_aggregator::StakeAggregator;
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
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
    /// The set of malicious actors that were agreed upon by a quorum of validators.
    /// This agreement is done synchronically, and thus is it safe to filter malicious actors.
    malicious_actors: HashSet<AuthorityName>,
    /// The reports of the malicious actors that are disrupting the MPC process.
    /// Maps the [`MaliciousReport`] to the set of authorities
    /// that reported the malicious actor.
    reports: HashMap<MaliciousReport, ReportStatus>,
}

enum ReportStatus {
    Tally(StakeAggregator<(), true>),
    QuorumReached,
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
    /// If a quorum of validators reports the same actor, it is considered malicious
    /// and inserted into the `self.malicious_actors` set.
    pub(crate) fn report_malicious_actor(
        &mut self,
        report: MaliciousReport,
        reporting_authority: AuthorityName,
    ) {
        if self.is_malicious_actor(&reporting_authority) {
            // Ignore malicious actors' votes.
            return;
        }

        let entry = self
            .reports
            .entry(report.clone())
            .or_insert(ReportStatus::Tally(StakeAggregator::new(
                self.committee.clone(),
            )));

        if let ReportStatus::Tally(stake_aggregator) = entry {
            if stake_aggregator
                .insert_generic(reporting_authority, ())
                .is_quorum_reached()
            {
                self.malicious_actors.extend(&report.malicious_actors);

                // Mark it as quorum reached so that future votes for this report would be ignored, and the data would be cleared.
                self.reports.insert(report, ReportStatus::QuorumReached);
            }
        }
    }

    pub(crate) fn is_malicious_actor(&self, authority: &AuthorityName) -> bool {
        self.malicious_actors.contains(authority)
    }

    pub(crate) fn get_malicious_actors_names(&self) -> &HashSet<AuthorityName> {
        &self.malicious_actors
    }

    /// Reports a malicious actor disrupting the MPC process.
    /// Reported by the validator itself.
    pub(crate) fn report_malicious_actors(&mut self, authorities: &[AuthorityName]) {
        // TODO(Scaly): This looks like a bug! we should only report malicious parties after getting quorum on them!
        // why doesn't it call report_malicious_actor()

        self.malicious_actors.extend(authorities);
    }
}
