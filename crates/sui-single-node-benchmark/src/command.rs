// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::{Parser, Subcommand, ValueEnum};
use strum_macros::EnumIter;

#[derive(Parser)]
#[clap(
    name = "sui-single-node-benchmark",
    about = "Benchmark a single validator node",
    rename_all = "kebab-case",
    author,
    version
)]
pub struct Command {
    #[arg(
        long,
        default_value_t = 500000,
        help = "Number of transactions to submit"
    )]
    pub tx_count: u64,
    #[arg(
        long,
        default_value_t = 100,
        help = "Number of transactions in a consensus commit/checkpoint"
    )]
    pub checkpoint_size: usize,
    #[arg(
        long,
        default_value_t = 2,
        help = "Number of address owned input objects per transaction.\
            This represents the amount of DB reads per transaction prior to execution."
    )]
    pub num_input_objects: u8,
    #[arg(
        long,
        default_value = "baseline",
        ignore_case = true,
        help = "Which component to benchmark"
    )]
    pub component: Component,
    #[clap(subcommand)]
    pub workload: WorkloadKind,
}

#[derive(Copy, Clone, EnumIter, ValueEnum)]
pub enum Component {
    ExecutionOnly,
    /// Baseline includes the execution and storage layer only.
    Baseline,
    /// On top of Baseline, this schedules transactions through the transaction manager.
    WithTxManager,
    /// This goes through the `handle_certificate` entry point on authority_server, which includes
    /// certificate verification, transaction manager, as well as a noop consensus layer. The noop
    /// consensus layer does absolutely nothing when receiving a transaction in consensus.
    ValidatorWithoutConsensus,
    /// Similar to ValidatorWithNoopConsensus, but the consensus layer contains a fake consensus
    /// protocol that basically sequences transactions in order. It then verify the transaction
    /// and store the sequenced transactions into the store. It covers the consensus-independent
    /// portion of the code in consensus handler.
    ValidatorWithFakeConsensus,
    /// Benchmark only validator signing component: `handle_transaction`.
    TxnSigning,
    /// Benchmark the checkpoint executor by constructing a full epoch of checkpoints, execute
    /// all transactions in them and measure time.
    CheckpointExecutor,
}

#[derive(Subcommand, Clone, Copy)]
pub enum WorkloadKind {
    NoMove,
    Move {
        #[arg(
            long,
            default_value_t = 0,
            help = "Number of dynamic fields read per transaction.\
            This represents the amount of runtime DB reads per transaction during execution."
        )]
        num_dynamic_fields: u64,
        #[arg(
            long,
            default_value_t = 0,
            help = "Computation intensity per transaction.\
            The transaction computes the n-th Fibonacci number \
            specified by this parameter * 100."
        )]
        computation: u8,
    },
}
