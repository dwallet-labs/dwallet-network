// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use consensus_config::Parameters;
use insta::assert_yaml_snapshot;

#[test]
fn parameters_snapshot_matches() {
    let parameters = Parameters::default();
    assert_yaml_snapshot!("parameters", parameters)
}
