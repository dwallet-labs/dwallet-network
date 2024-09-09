// Copyright (c) dWallet Labs, Ltd.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use mysten_metrics::spawn_monitored_task;
use sui_types::messages_signature_mpc::{
    InitSignatureMPCProtocolSequenceNumber, InitiateSignatureMPCProtocol, SignatureMPCSessionID,
};

use std::sync::Arc;
use tokio::sync::{mpsc, watch};

use crate::authority::authority_per_epoch_store::AuthorityPerEpochStore;
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
    // Create a channel for sending and receiving MPC messages.
    pub fn new(
        epoch_store: Arc<AuthorityPerEpochStore>,
        exit: watch::Receiver<()>,
        max_mpc_protocol_messages_in_progress: usize,
    ) -> mpsc::Receiver<InitiateSignatureMPCProtocol> {
        let (tx_initiate_signature_mpc_protocol_sender, rx_initiate_signature_mpc_protocol_sender) =
            mpsc::channel(max_mpc_protocol_messages_in_progress);

        // Subscribe to MPC msgs.
        let subscriber = Self {
            epoch_store,
            exit,
            tx_initiate_signature_mpc_protocol_sender,
            last: 0,
        };

        spawn_monitored_task!(subscriber.run());

        rx_initiate_signature_mpc_protocol_sender
    }

    // A special subscriber that listens for new MPC sessions.
    // todo(mpc-async): SignatureMpcSubscriber should renamed to SignatureInitMpcSubscriber
    // todo(mpc-async): make sure this does not kill the CPU,
    // todo(mpc-async): check if there is a delay or sleep needed.
    async fn run(mut self) {
        info!("Starting SignatureMpcSubscriber");
        loop {
            // If an exit signal received, break the loop.
            // This gives a chance to exit, if checkpoint making keeps failing.
            match self.exit.has_changed() {
                Ok(true) | Err(_) => {
                    break;
                }
                Ok(false) => (),
            };
            // todo(mpc-async): remove unwrap().
            let messages = self
                .epoch_store
                .get_initiate_signature_mpc_protocols(self.last)
                .unwrap();
            for (last_sequence, message) in messages {
                // Send MPC messages to channel.
                // todo(mpc-async): handle error.
                let _ = self
                    .tx_initiate_signature_mpc_protocol_sender
                    .send(message)
                    .await;
                self.last = last_sequence;
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
