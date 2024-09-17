// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

//# init --protocol-version 51 --simulator --accounts C

//# create-checkpoint

//# run-graphql
{
    protocolConfig {
        protocolVersion
        config(key: "max_move_identifier_len") {
            value
        }
        featureFlag(key: "enable_coin_deny_list") {
            value
        }
    }
}

//# run-graphql
{
    protocolConfig(protocolVersion: 8) {
        protocolVersion
        config(key: "max_move_identifier_len") {
            value
        }
        featureFlag(key: "enable_coin_deny_list") {
            value
        }
    }
}
