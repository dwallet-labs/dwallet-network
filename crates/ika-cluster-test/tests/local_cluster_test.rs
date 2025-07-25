// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use ika_cluster_test::{config::ClusterTestOpt, ClusterTest};

#[tokio::test]
async fn cluster_test() {
    telemetry_subscribers::init_for_testing();

    ClusterTest::run(ClusterTestOpt::new_local()).await;
}

#[tokio::test]
async fn test_ika_cluster() {
    use reqwest::StatusCode;
    use ika_cluster_test::cluster::Cluster;
    use ika_cluster_test::cluster::LocalNewCluster;
    use ika_graphql_rpc::client::simple_client::SimpleClient;
    use tokio::time::sleep;

    telemetry_subscribers::init_for_testing();

    let opts = ClusterTestOpt {
        with_indexer_and_graphql: true,
        ..ClusterTestOpt::new_local()
    };

    let cluster = LocalNewCluster::start(&opts).await.unwrap();

    let grphql_url = cluster.graphql_url().to_owned().unwrap();

    sleep(std::time::Duration::from_secs(20)).await;

    // Try JSON RPC URL
    let query = r#"
        {
            checkpoint {
                sequenceNumber
            }
        }
    "#;
    let resp = SimpleClient::new(grphql_url)
        .execute_to_graphql(query.to_string(), true, vec![], vec![])
        .await
        .unwrap();

    assert!(resp.errors().is_empty());
    assert!(resp.http_status() == StatusCode::OK);
    let resp_body = resp.response_body().data.clone().into_json().unwrap();
    assert!(resp_body.get("checkpoint").is_some());
}
