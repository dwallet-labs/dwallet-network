// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use pera_replay::execute_replay_command;
use pera_replay::ReplayToolCommand;
use std::path::PathBuf;

#[tokio::test]
async fn replay_sandboxes() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/sandbox_snapshots");

    // For each file in the sandbox, replay the transactions and compare the results.
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        assert!(path.is_file());
        let cmd = ReplayToolCommand::ReplaySandbox { path };

        execute_replay_command(None, true, true, None, None, cmd)
            .await
            .unwrap();
    }
}
