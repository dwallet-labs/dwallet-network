// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::Parser;
use move_cli::base::coverage;
use move_package::BuildConfig;
use std::path::Path;

#[derive(Parser)]
#[group(id = "pera-move-coverage")]
pub struct Coverage {
    #[clap(flatten)]
    pub coverage: coverage::Coverage,
}

impl Coverage {
    pub fn execute(self, path: Option<&Path>, build_config: BuildConfig) -> anyhow::Result<()> {
        self.coverage.execute(path, build_config)?;
        Ok(())
    }
}
