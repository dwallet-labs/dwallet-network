use crate::dwallet_checkpoints::DWalletCheckpointStore;
use crate::sui_connector::metrics::SuiConnectorMetrics;
use crate::sui_connector::sui_executor::{StopReason, SuiExecutor};
use crate::sui_connector::sui_syncer::SuiSyncer;
use crate::system_checkpoints::SystemCheckpointStore;
use anyhow::anyhow;
use async_trait::async_trait;
use dwallet_mpc_types::dwallet_mpc::NetworkDecryptionKeyPublicData;
use futures::{future, StreamExt};
use ika_config::node::{RunWithRange, SuiChainIdentifier, SuiConnectorConfig};
use ika_sui_client::{SuiClient, SuiClientInner};
use ika_types::committee::{Committee, EpochId};
use ika_types::error::IkaResult;
use ika_types::messages_consensus::MovePackageDigest;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use shared_crypto::intent::{Intent, IntentMessage};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use sui_json_rpc_types::{Coin, SuiEvent};
use sui_sdk::apis::CoinReadApi;
use sui_sdk::SuiClient as SuiSdkClient;
use sui_types::base_types::{ObjectID, ObjectRef, SuiAddress};
use sui_types::crypto::{Signature, SuiKeyPair};
use sui_types::digests::{get_mainnet_chain_identifier, get_testnet_chain_identifier};
use sui_types::transaction::{ProgrammableTransaction, Transaction, TransactionData};
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tracing::info;

pub mod metrics;
pub mod sui_executor;
pub mod sui_syncer;

pub const TEST_MODULE_NAME: &IdentStr = ident_str!("test");
pub const DWALLET_2PC_MPC_COORDINATOR_INNER_MODULE_NAME: &IdentStr =
    ident_str!("dwallet_2pc_mpc_coordinator_inner");

pub struct SuiNotifier {
    sui_key: SuiKeyPair,
    sui_address: SuiAddress,
}

pub struct SuiConnectorService {
    sui_client: Arc<SuiClient<SuiSdkClient>>,
    sui_executor: SuiExecutor<SuiSdkClient>,
    #[allow(dead_code)]
    task_handles: Vec<JoinHandle<()>>,
    #[allow(dead_code)]
    sui_connector_config: SuiConnectorConfig,
    #[allow(dead_code)]
    metrics: Arc<SuiConnectorMetrics>,
}

impl SuiConnectorService {
    pub async fn new(
        checkpoint_store: Arc<DWalletCheckpointStore>,
        system_checkpoint_store: Arc<SystemCheckpointStore>,
        sui_client: Arc<SuiClient<SuiSdkClient>>,
        sui_connector_config: SuiConnectorConfig,
        sui_connector_metrics: Arc<SuiConnectorMetrics>,
        is_validator: bool,
        network_keys_sender: watch::Sender<Arc<HashMap<ObjectID, NetworkDecryptionKeyPublicData>>>,
        next_epoch_committee_sender: watch::Sender<Committee>,
        new_events_sender: tokio::sync::broadcast::Sender<Vec<SuiEvent>>,
    ) -> anyhow::Result<Self> {
        let sui_notifier = Self::prepare_for_sui(
            sui_connector_config.clone(),
            sui_client.clone(),
            sui_connector_metrics.clone(),
        )
        .await?;

        let sui_executor = SuiExecutor::new(
            sui_connector_config.ika_system_package_id,
            checkpoint_store.clone(),
            system_checkpoint_store.clone(),
            sui_notifier,
            sui_client.clone(),
            sui_connector_metrics.clone(),
        );

        let sui_modules_to_watch = vec![
            TEST_MODULE_NAME.to_owned(),
            DWALLET_2PC_MPC_COORDINATOR_INNER_MODULE_NAME.to_owned(),
        ];
        let task_handles = SuiSyncer::new(
            sui_client.clone(),
            sui_modules_to_watch,
            sui_connector_metrics.clone(),
        )
        .run(
            Duration::from_secs(2),
            next_epoch_committee_sender,
            is_validator,
            network_keys_sender,
            new_events_sender,
        )
        .await
        .map_err(|e| anyhow::anyhow!("Failed to start sui syncer: {e}"))?;
        Ok(Self {
            sui_client,
            sui_executor,
            task_handles,
            sui_connector_config,
            metrics: sui_connector_metrics,
        })
    }

    pub async fn run_epoch(
        &self,
        epoch_id: EpochId,
        run_with_range: Option<RunWithRange>,
    ) -> StopReason {
        self.sui_executor.run_epoch(epoch_id, run_with_range).await
    }

    async fn prepare_for_sui(
        sui_connector_config: SuiConnectorConfig,
        sui_client: Arc<SuiClient<SuiSdkClient>>,
        _sui_connector_metrics: Arc<SuiConnectorMetrics>,
    ) -> anyhow::Result<Option<SuiNotifier>> {
        let Some(sui_key_path) = sui_connector_config.notifier_client_key_pair else {
            return Ok(None);
        };

        let sui_key = sui_key_path.keypair().copy();

        // If sui chain id is  Mainent or Testnet, we expect to see chain
        // identifier to match accordingly.
        let sui_identifier = sui_client
            .get_chain_identifier()
            .await
            .map_err(|e| anyhow!("Error getting chain identifier from Sui: {:?}", e))?;

        if sui_connector_config.sui_chain_identifier == SuiChainIdentifier::Mainnet
            && sui_identifier != get_mainnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected sui chain {}, but connected to {}",
                sui_connector_config.sui_chain_identifier,
                sui_identifier
            );
        }
        if sui_connector_config.sui_chain_identifier == SuiChainIdentifier::Testnet
            && sui_identifier != get_testnet_chain_identifier().to_string()
        {
            anyhow::bail!(
                "Expected sui chain {}, but connected to {}",
                sui_connector_config.sui_chain_identifier,
                sui_identifier
            );
        }
        info!(
            "Connected sui chain {}, sui identifier: {}",
            sui_connector_config.sui_chain_identifier, sui_identifier
        );

        let sui_address = SuiAddress::from(&sui_key.public());
        Ok(Some(SuiNotifier {
            sui_key,
            sui_address,
        }))
    }

    pub async fn get_available_move_packages(
        &self,
    ) -> anyhow::Result<Vec<(ObjectID, MovePackageDigest)>> {
        self.sui_client
            .get_available_move_packages()
            .await
            .map_err(|e| anyhow!("Cannot get available move packages: {:?}", e))
    }
}

#[async_trait]
pub trait CheckpointMessageSuiNotify: Sync + Send + 'static {
    async fn notify_certified_checkpoint_message(
        &self,
        signature: Vec<u8>,
        signers: Vec<u16>,
        message: Vec<u8>,
    ) -> IkaResult;
}

#[async_trait]
impl CheckpointMessageSuiNotify for SuiConnectorService {
    async fn notify_certified_checkpoint_message(
        &self,
        _signature: Vec<u8>,
        _signers: Vec<u16>,
        _message: Vec<u8>,
    ) -> IkaResult {
        Ok(())
    }
}

pub(crate) async fn build_sui_transaction<C: SuiClientInner>(
    signer: SuiAddress,
    pt: ProgrammableTransaction,
    sui_client: &Arc<SuiClient<C>>,
    gas_payment: Vec<ObjectRef>,
    sui_key: &SuiKeyPair,
) -> Transaction {
    let computation_price = sui_client.get_reference_gas_price_until_success().await;

    let tx_data = TransactionData::new_programmable(
        signer,
        gas_payment,
        pt,
        10_000_000_000,
        computation_price,
    );

    let signature = Signature::new_secure(
        &IntentMessage::new(Intent::sui_transaction(), &tx_data),
        sui_key,
    );

    Transaction::from_data(tx_data, vec![signature])
}

pub async fn pick_highest_balance_coin(
    coin_read_api: &CoinReadApi,
    address: SuiAddress,
    minimal_amount: u64,
) -> anyhow::Result<Coin> {
    let mut highest_balance = 0;
    let mut highest_balance_coin = None;
    coin_read_api
        .get_coins_stream(address, None)
        .for_each(|coin: Coin| {
            if coin.balance > highest_balance {
                highest_balance = coin.balance;
                highest_balance_coin = Some(coin.clone());
            }
            future::ready(())
        })
        .await;
    if highest_balance_coin.is_none() {
        return Err(anyhow!("No Sui coins found for address {:?}", address));
    }
    if highest_balance < minimal_amount {
        return Err(anyhow!(
            "Found no single coin that has >= {} balance Sui for address {:?}",
            minimal_amount,
            address,
        ));
    }
    Ok(highest_balance_coin.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tracing::debug;
    async fn example_func_ok() -> anyhow::Result<()> {
        Ok(())
    }

    async fn example_func_err() -> anyhow::Result<()> {
        info!("example_func_err");
        Err(anyhow::anyhow!(""))
    }

    #[tokio::test]
    async fn test_retry_with_max_elapsed_time() {
        telemetry_subscribers::init_for_testing();
        // no retry is needed, should return immediately. We give it a very small
        // max_elapsed_time and it should still finish in time.
        let max_elapsed_time = Duration::from_millis(20);
        retry_with_max_elapsed_time!(example_func_ok(), max_elapsed_time)
            .unwrap()
            .unwrap();

        // now call a function that always errors and expect it to return before max_elapsed_time runs out
        let max_elapsed_time = Duration::from_secs(10);
        let instant = std::time::Instant::now();
        retry_with_max_elapsed_time!(example_func_err(), max_elapsed_time).unwrap_err();
        assert!(instant.elapsed() < max_elapsed_time);
    }
}
