use anyhow::anyhow;
use helios::client::{Client, ClientBuilder};
use helios::consensus::database::FileDB;
use log::info;
use sui_sdk::sui_client_config::EthLightClientConfig;
pub struct EthLightClient {
    pub client: Client<FileDB>,
    config: EthLightClientConfig,
}

impl EthLightClient {
    pub async fn new(conf: EthLightClientConfig) -> Result<Self, anyhow::Error> {
        let network = &conf.network;

        let client: Client<FileDB> = ClientBuilder::new()
            .network(network.clone())
            .execution_rpc(&conf.execution_rpc)
            .consensus_rpc(&conf.consensus_rpc)
            .checkpoint(&conf.checkpoint)
            .data_dir("/tmp/helios".parse()?)
            .build()
            .map_err(|e| anyhow!("failed to create client: {}", e))?;

        info!("EthLightClient created");

        Ok(Self {
            client,
            config: conf,
        })
    }

    pub async fn start(&mut self) -> Result<(), anyhow::Error> {
        self.client
            .start()
            .await
            .map_err(|e| anyhow!("failed to start client: {}", e))?;
        self.client.wait_synced().await;
        info!("EthLightClient connected");

        Ok(())
    }

}
