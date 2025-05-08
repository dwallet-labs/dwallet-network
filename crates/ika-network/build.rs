// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::{
    env,
    path::{Path, PathBuf},
};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let out_dir = if env::var("DUMP_GENERATED_GRPC").is_ok() {
        PathBuf::from("")
    } else {
        PathBuf::from(env::var("OUT_DIR")?)
    };
    //
    // let codec_path = "mysten_network::codec::BcsCodec";
    //
    // // let validator_service = Service::builder()
    // //     .name("Validator")
    // //     .package("ika.validator")
    // //     .comment("The Validator interface")
    // //     .method(
    // //         Method::builder()
    // //             .name("transaction")
    // //             .route_name("Transaction")
    // //             .input_type("ika_types::transaction::Transaction")
    // //             .output_type("ika_types::messages_grpc::HandleTransactionResponse")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("transaction_v2")
    // //             .route_name("TransactionV2")
    // //             .input_type("ika_types::messages_grpc::HandleTransactionRequestV2")
    // //             .output_type("ika_types::messages_grpc::HandleTransactionResponseV2")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("handle_certificate_v2")
    // //             .route_name("CertifiedTransactionV2")
    // //             .input_type("ika_types::transaction::CertifiedTransaction")
    // //             .output_type("ika_types::messages_grpc::HandleCertificateResponseV2")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("handle_certificate_v3")
    // //             .route_name("CertifiedTransactionV3")
    // //             .input_type("ika_types::messages_grpc::HandleCertificateRequestV3")
    // //             .output_type("ika_types::messages_grpc::HandleCertificateResponseV3")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("handle_soft_bundle_certificates_v3")
    // //             .route_name("SoftBundleCertifiedTransactionsV3")
    // //             .input_type("ika_types::messages_grpc::HandleSoftBundleCertificatesRequestV3")
    // //             .output_type("ika_types::messages_grpc::HandleSoftBundleCertificatesResponseV3")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("submit_certificate")
    // //             .route_name("SubmitCertificate")
    // //             .input_type("ika_types::transaction::CertifiedTransaction")
    // //             .output_type("ika_types::messages_grpc::SubmitCertificateResponse")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("object_info")
    // //             .route_name("ObjectInfo")
    // //             .input_type("ika_types::messages_grpc::ObjectInfoRequest")
    // //             .output_type("ika_types::messages_grpc::ObjectInfoResponse")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("transaction_info")
    // //             .route_name("TransactionInfo")
    // //             .input_type("ika_types::messages_grpc::TransactionInfoRequest")
    // //             .output_type("ika_types::messages_grpc::TransactionInfoResponse")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("checkpoint")
    // //             .route_name("Checkpoint")
    // //             .input_type("ika_types::messages_checkpoint::CheckpointRequest")
    // //             .output_type("ika_types::messages_checkpoint::CheckpointResponse")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("checkpoint_v2")
    // //             .route_name("CheckpointV2")
    // //             .input_type("ika_types::messages_checkpoint::CheckpointRequestV2")
    // //             .output_type("ika_types::messages_checkpoint::CheckpointResponseV2")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .method(
    // //         Method::builder()
    // //             .name("get_system_state_object")
    // //             .route_name("GetSystemStateObject")
    // //             .input_type("ika_types::messages_grpc::SystemStateRequest")
    // //             .output_type("ika_types::ika_system_state::IkaSystemState")
    // //             .codec_path(codec_path)
    // //             .build(),
    // //     )
    // //     .build();
    // //
    // // Builder::new()
    // //     .out_dir(&out_dir)
    // //     .compile(&[validator_service]);

    build_anemo_services(&out_dir);

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=DUMP_GENERATED_GRPC");

    Ok(())
}

fn build_anemo_services(out_dir: &Path) {
    let codec_path = "mysten_network::codec::anemo::BcsSnappyCodec";

    let discovery = anemo_build::manual::Service::builder()
        .name("Discovery")
        .package("ika")
        .method(
            anemo_build::manual::Method::builder()
                .name("get_known_peers_v2")
                .route_name("GetKnownPeersV2")
                .request_type("()")
                .response_type("crate::discovery::GetKnownPeersResponseV2")
                .codec_path(codec_path)
                .build(),
        )
        .build();

    let state_sync = anemo_build::manual::Service::builder()
        .name("StateSync")
        .package("ika")
        .method(
            anemo_build::manual::Method::builder()
                .name("push_checkpoint_message")
                .route_name("PushCheckpointMessage")
                .request_type("ika_types::messages_checkpoint::CertifiedCheckpointMessage<ika_types::message::MessageKind>")
                .response_type("()")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            anemo_build::manual::Method::builder()
                .name("get_checkpoint_message")
                .route_name("GetCheckpointMessage")
                .request_type("crate::state_sync::GetCheckpointMessageRequest")
                .response_type("Option<ika_types::messages_checkpoint::CertifiedCheckpointMessage<ika_types::message::MessageKind>>")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            anemo_build::manual::Method::builder()
                .name("get_checkpoint_availability")
                .route_name("GetCheckpointAvailability")
                .request_type("()")
                .response_type("crate::state_sync::GetCheckpointAvailabilityResponse")
                .codec_path(codec_path)
                .build(),
        )
        .method(
            anemo_build::manual::Method::builder()
                .name("get_chain_identifier")
                .route_name("GetChainIdentifier")
                .request_type("()")
                .response_type("crate::state_sync::GetChainIdentifierResponse")
                .codec_path(codec_path)
                .build(),
        )
        .build();
    anemo_build::manual::Builder::new()
        .out_dir(out_dir)
        .compile(&[discovery, state_sync]);
}
