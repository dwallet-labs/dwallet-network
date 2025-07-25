// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear
use std::{collections::HashMap, sync::Arc};

use arc_swap::ArcSwap;
use consensus_config::Committee as ConsensusCommittee;
use ika_types::committee::Committee;
use ika_types::crypto::AuthorityName;
use ika_types::messages_consensus::AuthorityIndex;
use tracing::debug;

use crate::authority::AuthorityMetrics;

/// Updates list of authorities that are deemed to have low reputation scores by consensus
/// these may be lagging behind the network, byzantine, or not reliably participating for any reason.
/// The algorithm is flagging as low scoring authorities all the validators that have the lowest scores
/// up to the defined protocol_config.consensus_bad_nodes_stake_threshold. This is done to align the
/// submission side with the consensus leader election schedule. Practically we don't want to submit
/// transactions for sequencing to validators that have low scores and are not part of the leader
/// schedule since the chances of getting them sequenced are lower.
pub(crate) fn update_low_scoring_authorities(
    low_scoring_authorities: Arc<ArcSwap<HashMap<AuthorityName, u64>>>,
    ika_committee: &Committee,
    consensus_committee: &ConsensusCommittee,
    reputation_score_sorted_desc: Option<Vec<(AuthorityIndex, u64)>>,
    metrics: &Arc<AuthorityMetrics>,
    consensus_bad_nodes_stake_threshold: u64,
) {
    assert!(
        (0..=33).contains(&consensus_bad_nodes_stake_threshold),
        "The bad_nodes_stake_threshold should be in range [0 - 33], out of bounds parameter detected {consensus_bad_nodes_stake_threshold}"
    );

    let Some(reputation_scores) = reputation_score_sorted_desc else {
        return;
    };

    // We order the authorities by score ascending order in the exact same way as the reputation
    // scores do - so we keep complete alignment between implementations
    let scores_per_authority_order_asc: Vec<_> = reputation_scores
        .into_iter()
        .rev() // we reverse so we get them in asc order
        .collect();

    let mut final_low_scoring_map = HashMap::new();
    let mut total_stake = 0;
    for (index, score) in scores_per_authority_order_asc {
        let authority_name = ika_committee.authority_by_index(index).unwrap();
        let authority_index = consensus_committee
            .to_authority_index(index as usize)
            .unwrap();
        let consensus_authority = consensus_committee.authority(authority_index);
        let hostname = &consensus_authority.hostname;
        let stake = consensus_authority.stake;
        total_stake += stake;

        let included = if total_stake
            <= consensus_bad_nodes_stake_threshold * consensus_committee.total_stake() / 100
        {
            final_low_scoring_map.insert(*authority_name, score);
            true
        } else {
            false
        };

        if !hostname.is_empty() {
            debug!(
                "authority {} has score {}, is low scoring: {}",
                hostname, score, included
            );

            metrics
                .consensus_handler_scores
                .with_label_values(&[hostname])
                .set(score as i64);
        }
    }
    // Report the actual flagged final low scoring authorities
    metrics
        .consensus_handler_num_low_scoring_authorities
        .set(final_low_scoring_map.len() as i64);
    low_scoring_authorities.swap(Arc::new(final_low_scoring_map));
}
