// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use crate::benchmark_context::BenchmarkContext;
use crate::command::WorkloadKind;
use crate::tx_generator::{MoveTxGenerator, NonMoveTxGenerator, TxGenerator};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub struct Workload {
    pub tx_count: u64,
    pub workload_kind: WorkloadKind,
    pub num_input_objects: u8,
}

impl Workload {
    pub fn new(tx_count: u64, workload_kind: WorkloadKind, num_input_objects: u8) -> Self {
        Self {
            tx_count,
            workload_kind,
            num_input_objects,
        }
    }

    pub(crate) fn num_accounts(&self) -> u64 {
        self.tx_count
    }

    pub(crate) fn gas_object_num_per_account(&self) -> u64 {
        self.num_input_objects as u64
    }

    pub(crate) async fn create_tx_generator(
        &self,
        ctx: &mut BenchmarkContext,
    ) -> Arc<dyn TxGenerator> {
        assert!(
            self.num_input_objects >= 1,
            "Each transaction requires at least 1 input object"
        );
        match self.workload_kind {
            WorkloadKind::NoMove => Arc::new(NonMoveTxGenerator::new(self.num_input_objects)),
            WorkloadKind::Move {
                num_dynamic_fields,
                computation,
            } => {
                assert!(
                    self.num_input_objects >= 2,
                    "Move transaction requires at least 2 input objects"
                );
                let move_package = ctx.publish_package().await;
                let root_objects = ctx
                    .preparing_dynamic_fields(move_package.0, num_dynamic_fields)
                    .await;
                Arc::new(MoveTxGenerator::new(
                    move_package.0,
                    self.num_input_objects,
                    computation,
                    root_objects,
                ))
            }
        }
    }
}
