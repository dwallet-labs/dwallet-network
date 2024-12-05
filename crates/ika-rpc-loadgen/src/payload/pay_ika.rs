// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::payload::rpc_command_processor::DEFAULT_GAS_BUDGET;
use crate::payload::{PayIka, ProcessPayload, RpcCommandProcessor, SignerInfo};
use async_trait::async_trait;
use futures::future::join_all;
use ika_types::base_types::IkaAddress;
use ika_types::crypto::{EncodeDecodeBase64, IkaKeyPair};
use ika_types::quorum_driver_types::ExecuteTransactionRequestType;
use ika_types::transaction::TransactionData;
use tracing::debug;

#[async_trait]
impl<'a> ProcessPayload<'a, &'a PayIka> for RpcCommandProcessor {
    async fn process(
        &'a self,
        _op: &'a PayIka,
        signer_info: &Option<SignerInfo>,
    ) -> anyhow::Result<()> {
        let clients = self.get_clients().await?;
        let SignerInfo {
            encoded_keypair,
            gas_budget,
            gas_payment,
        } = signer_info.clone().unwrap();
        let recipient = IkaAddress::random_for_testing_only();
        let amount = 1;
        let gas_budget = gas_budget.unwrap_or(DEFAULT_GAS_BUDGET);
        let gas_payments = gas_payment.unwrap();

        let keypair =
            IkaKeyPair::decode_base64(&encoded_keypair).expect("Decoding keypair should not fail");

        debug!(
            "Transfer Ika {} time to {recipient} with {amount} NIKA with {gas_payments:?}",
            gas_payments.len()
        );

        let sender = IkaAddress::from(&keypair.public());
        // TODO: For write operations, we usually just want to submit the transaction to fullnode
        // Let's figure out what's the best way to support other mode later
        let client = clients.first().unwrap();
        let gas_price = client
            .governance_api()
            .get_reference_gas_price()
            .await
            .expect("Unable to fetch gas price");
        join_all(gas_payments.iter().map(|gas| async {
            let tx = TransactionData::new_transfer_ika(
                recipient,
                sender,
                Some(amount),
                self.get_object_ref(client, gas).await,
                gas_budget,
                gas_price,
            );
            self.sign_and_execute(
                client,
                &keypair,
                tx,
                ExecuteTransactionRequestType::WaitForEffectsCert,
            )
            .await
        }))
        .await;

        Ok(())
    }
}
