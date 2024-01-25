// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::base64::Base64;
use async_graphql::*;

#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
pub(crate) struct ValidatorCredentials {
    pub protocol_pub_key: Option<Base64>,
    pub network_pub_key: Option<Base64>,
    pub worker_pub_key: Option<Base64>,
    pub proof_of_possession: Option<Base64>,
    pub net_address: Option<String>,
    pub p2p_address: Option<String>,
    pub primary_address: Option<String>,
    pub worker_address: Option<String>,
}
