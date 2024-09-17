// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use std::collections::BTreeSet;

use consensus_config::{AuthorityIndex, NetworkKeyPair};
use fastcrypto::ed25519::Ed25519PublicKey;
use tokio_rustls::rustls::{ClientConfig, ServerConfig};

use crate::context::Context;

pub(crate) fn create_rustls_server_config(
    context: &Context,
    network_keypair: NetworkKeyPair,
) -> ServerConfig {
    let allower = AllowedPublicKeys::new(context);
    let verifier = pera_tls::ClientCertVerifier::new(allower, certificate_server_name(context));
    // TODO: refactor to use key bytes
    let self_signed_cert = pera_tls::SelfSignedCertificate::new(
        network_keypair.private_key().into_inner(),
        &certificate_server_name(context),
    );
    let tls_cert = self_signed_cert.rustls_certificate();
    let tls_private_key = self_signed_cert.rustls_private_key();
    let mut tls_config = verifier
        .rustls_server_config(vec![tls_cert], tls_private_key)
        .unwrap_or_else(|e| panic!("Failed to create TLS server config: {:?}", e));
    tls_config.alpn_protocols = vec![b"h2".to_vec()];
    tls_config
}

pub(crate) fn create_rustls_client_config(
    context: &Context,
    network_keypair: NetworkKeyPair,
    target: AuthorityIndex,
) -> ClientConfig {
    let target_public_key = context
        .committee
        .authority(target)
        .network_key
        .clone()
        .into_inner();
    let self_signed_cert = pera_tls::SelfSignedCertificate::new(
        network_keypair.private_key().into_inner(),
        &certificate_server_name(context),
    );
    let tls_cert = self_signed_cert.rustls_certificate();
    let tls_private_key = self_signed_cert.rustls_private_key();
    let mut tls_config =
        pera_tls::ServerCertVerifier::new(target_public_key, certificate_server_name(context))
            .rustls_client_config(vec![tls_cert], tls_private_key)
            .unwrap_or_else(|e| panic!("Failed to create TLS client config: {:?}", e));
    // ServerCertVerifier sets alpn for completeness, but alpn cannot be predefined when
    // using HttpsConnector from hyper-rustls, as in TonicManager.
    tls_config.alpn_protocols = vec![];
    tls_config
}

// Checks if the public key from a TLS certificate belongs to one of the validators.
#[derive(Debug)]
struct AllowedPublicKeys {
    // TODO: refactor to use key bytes
    keys: BTreeSet<Ed25519PublicKey>,
}

impl AllowedPublicKeys {
    fn new(context: &Context) -> Self {
        let keys = context
            .committee
            .authorities()
            .map(|(_i, a)| a.network_key.clone().into_inner())
            .collect();
        Self { keys }
    }
}

impl pera_tls::Allower for AllowedPublicKeys {
    fn allowed(&self, key: &Ed25519PublicKey) -> bool {
        self.keys.contains(key)
    }
}

fn certificate_server_name(context: &Context) -> String {
    format!("consensus_epoch_{}", context.committee.epoch())
}
