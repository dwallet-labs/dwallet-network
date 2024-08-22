// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use mysten_metrics::spawn_monitored_task;
use sui_types::messages_signature_mpc::{
    InitSignatureMPCProtocolSequenceNumber, InitiateSignatureMPCProtocol, SignatureMPCSessionID,
};

use std::sync::Arc;
use tokio::sync::{mpsc, watch};

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
use crate::signature_mpc::MAX_MESSAGES_IN_PROGRESS;
use futures::future::{select, Either};
use futures::FutureExt;
use std::str::FromStr;
use std::time::Duration;
use sui_types::base_types::SuiAddress;
use sui_types::signature_mpc::{
    DKGSession, CREATE_DKG_SESSION_FUNC_NAME, DWALLET_2PC_MPC_ECDSA_K1_MODULE_NAME,
};
use sui_types::storage::ObjectStore;
use sui_types::SUI_SYSTEM_ADDRESS;
use tokio_stream::{Stream, StreamExt};
use tracing::{debug, error, info, instrument, subscriber, trace_span};

pub struct SignatureMpcSubscriber {
    epoch_store: Arc<AuthorityPerEpochStore>,
    exit: watch::Receiver<()>,
    tx_initiate_signature_mpc_protocol_sender: mpsc::Sender<InitiateSignatureMPCProtocol>,
    last: InitSignatureMPCProtocolSequenceNumber,
}

impl SignatureMpcSubscriber {
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        exit: watch::Receiver<()>,
    ) -> mpsc::Receiver<InitiateSignatureMPCProtocol> {
        let (tx_initiate_signature_mpc_protocol_sender, rx_initiate_signature_mpc_protocol_sender) =
            mpsc::channel(MAX_MESSAGES_IN_PROGRESS);

        let subscriber = Self {
            epoch_store,
            exit,
            tx_initiate_signature_mpc_protocol_sender,
            last: 0,
        };

        spawn_monitored_task!(subscriber.run());

        rx_initiate_signature_mpc_protocol_sender
    }

    async fn run(mut self) {
        info!("Starting SignatureMpcSubscriber");
        loop {
            // Check whether an exit signal has been received if so we break the loop.
            // This gives us a chance to exit if checkpoint making keeps failing.
            match self.exit.has_changed() {
                Ok(true) | Err(_) => {
                    break;
                }
                Ok(false) => (),
            };
            let messages = self
                .epoch_store
                .get_initiate_signature_mpc_protocols(self.last)
                .unwrap();
            for (last, message) in messages {
                let _ = self
                    .tx_initiate_signature_mpc_protocol_sender
                    .send(message)
                    .await;
                self.last = last;
            }
            tokio::task::yield_now().await;
        }
        info!("Shutting down SignatureMpcSubscriber");
    }

    // fn stream(&self) -> impl Stream<Item = sui_json_rpc_types::SuiTransactionBlockEffects> {
    //     self.state.subscription_handler.subscribe_transactions(TransactionFilter::MoveFunction {
    //         package: "0x3".parse().unwrap(),
    //         module: None,
    //         function: None,
    //     })
    // }
    //
    // async fn run(mut self) {
    //     info!("Starting SignatureMpcSubscriber");
    //
    //     let mut stream = self.stream();
    //
    //
    //
    //     loop {
    //         tokio::select! {
    //             biased;
    //
    //             _ = self.exit.changed().boxed() => {
    //                 // return on exit signal
    //                 info!("Shutting down SignatureMpcSubscriber");
    //                 return;
    //             }
    //
    //             Some(effects) = stream.next() => {
    //                 if let sui_json_rpc_types::SuiTransactionBlockEffects::V1(effects) = effects {
    //                     let obj_ref = &effects.created[0];
    //
    //                     // TODO: Rewrite the code with no unwrap
    //
    //                     let obj = self.state.database.get_object(&obj_ref.object_id()).unwrap();
    //                     if let Some(move_object) = obj.unwrap().data.try_as_move() {
    //                         let obj: DKGSession = bcs::from_bytes(move_object.contents()).ok().unwrap();
    //                         info!("fetching DKGSession {:?}", obj);
    //                         let commitment = obj.commitment;
    //                         // TODO: validate commitment error
    //                         let message = InitiateSignatureMPCProtocol::DKG((SignatureMpcSessionID(move_object.id().into_bytes()), bcs::from_bytes(&*commitment).unwrap()));
    //                         let _ = self.tx_initiate_signature_mpc_protocol_sender.send(message);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    // }
}
