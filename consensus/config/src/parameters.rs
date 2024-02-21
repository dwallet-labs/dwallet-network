// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::time::Duration;

use serde::{Deserialize, Serialize};

/// Operational configurations of a consensus authority.
///
/// All fields should tolerate inconsistencies among authorities, without affecting safety of the
/// protocol. Otherwise, they need to be part of Sui protocol config or epoch state on-chain.
///
/// NOTE: default values should make sense, so most operators should not need to specify any field.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameters {
    /// Time to wait for parent round leader before sealing a block.
    #[serde(default = "Parameters::default_leader_timeout")]
    pub leader_timeout: Duration,
}

impl Parameters {
    pub fn default_leader_timeout() -> Duration {
        Duration::from_millis(250)
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Self {
            leader_timeout: Parameters::default_leader_timeout(),
        }
    }
}
