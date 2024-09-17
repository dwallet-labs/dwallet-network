// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::Parser;
use move_cli::base::migrate;
use move_package::BuildConfig as MoveBuildConfig;
use std::path::Path;

#[derive(Parser)]
#[group(id = "pera-move-migrate")]
pub struct Migrate {
    #[clap(flatten)]
    pub migrate: migrate::Migrate,
}

impl Migrate {
    pub fn execute(self, path: Option<&Path>, config: MoveBuildConfig) -> anyhow::Result<()> {
        self.migrate.execute(path, config)
    }
}
