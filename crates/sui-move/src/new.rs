// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use clap::Parser;
use move_cli::base::new;
use std::path::PathBuf;

const SUI_PKG_NAME: &str = "Sui";

// Use testnet by default. Probably want to add options to make this configurable later
const SUI_PKG_PATH: &str = "{ git = \"https://github.com/MystenLabs/sui.git\", subdir = \"crates/sui-framework/packages/dwallet-framework\", rev = \"framework/testnet\" }";

#[derive(Parser)]
#[group(id = "sui-move-new")]
pub struct New {
    #[clap(flatten)]
    pub new: new::New,
}

impl New {
    pub fn execute(self, path: Option<PathBuf>) -> anyhow::Result<()> {
        let name = &self.new.name.to_lowercase();
        self.new
            .execute(path, [(SUI_PKG_NAME, SUI_PKG_PATH)], [(name, "0x0")], "")?;
        Ok(())
    }
}
