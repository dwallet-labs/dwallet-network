// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::consensus_handler::{
    SequencedConsensusTransactionKind, VerifiedSequencedConsensusTransaction,
};
use mysten_metrics::monitored_scope;
use ika_protocol_config::ConsensusTransactionOrdering;
use sui_types::{
    messages_consensus::{ConsensusTransaction, ConsensusTransactionKind},
    transaction::TransactionDataAPI as _,
};

pub struct PostConsensusTxReorder {}

impl PostConsensusTxReorder {
    pub fn reorder(
        transactions: &mut [VerifiedSequencedConsensusTransaction],
        kind: ConsensusTransactionOrdering,
    ) {
        // TODO: make the reordering algorithm richer and depend on object hotness as well.
        // Order transactions based on their computation prices. System transactions without computation price
        // are put to the beginning of the sequenced_transactions vector.
        match kind {
            ConsensusTransactionOrdering::ByGasPrice => Self::order_by_computation_price(transactions),
            ConsensusTransactionOrdering::None => (),
        }
    }

    fn order_by_computation_price(transactions: &mut [VerifiedSequencedConsensusTransaction]) {
        let _scope = monitored_scope("ConsensusCommitHandler::order_by_computation_price");
        transactions.sort_by_key(|txn| {
            // Reverse order, so that transactions with higher computation price are put to the beginning.
            std::cmp::Reverse({
                match &txn.0.transaction {
                    // SequencedConsensusTransactionKind::External(ConsensusTransaction {
                    //     tracking_id: _,
                    //     kind: ConsensusTransactionKind::CertifiedTransaction(cert),
                    // }) => cert.computation_price(),
                    // SequencedConsensusTransactionKind::External(ConsensusTransaction {
                    //     tracking_id: _,
                    //     kind: ConsensusTransactionKind::UserTransaction(txn),
                    // }) => txn.transaction_data().computation_price(),
                    // Non-user transactions are considered to have computation price of MAX u64 and are
                    // put to the beginning.
                    _ => u64::MAX,
                }
            })
        })
    }
}
