// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::traits::{EncodeDecodeBase64, KeyPair};
use fastcrypto_zkp::bn254::zk_login::{parse_jwks, OIDCProvider, ZkLoginInputs};
use rand::{rngs::StdRng, SeedableRng};
use shared_crypto::intent::{Intent, IntentMessage};
use sui_types::{
    authenticator_state::ActiveJwk,
    base_types::dbg_addr,
    crypto::{get_key_pair, AccountKeyPair, PublicKey, Signature, SuiKeyPair},
    error::{SuiError, UserInputError},
    multisig::{MultiSig, MultiSigPublicKey},
    signature::GenericSignature,
    transaction::{
        AuthenticatorStateUpdate, GenesisTransaction, TransactionDataAPI, TransactionExpiration,
        TransactionKind,
    },
    utils::{to_sender_signed_transaction, TestData},
    zk_login_authenticator::ZkLoginAuthenticator,
    zk_login_util::DEFAULT_JWK_BYTES,
};

use sui_macros::sim_test;

macro_rules! assert_matches {
    ($expression:expr, $pattern:pat $(if $guard: expr)?) => {
        match $expression {
            $pattern $(if $guard)? => {}
            ref e => panic!(
                "assertion failed: `(left == right)` \
                 (left: `{:?}`, right: `{:?}`)",
                e,
                stringify!($pattern $(if $guard)?)
            ),
        }
    };
}

use crate::{
    authority_client::{AuthorityAPI, NetworkAuthorityClient},
    authority_server::{AuthorityServer, AuthorityServerHandle},
    stake_aggregator::{InsertResult, StakeAggregator},
};

use super::*;
use fastcrypto::traits::AggregateAuthenticator;
use sui_types::programmable_transaction_builder::ProgrammableTransactionBuilder;

pub use crate::authority::authority_test_utils::init_state_with_ids;

#[sim_test]
async fn test_handle_transfer_transaction_bad_signature() {
    do_transaction_test(
        1,
        |_| {},
        |mut_tx| {
            let (_unknown_address, unknown_key): (_, AccountKeyPair) = get_key_pair();
            let data = mut_tx.data_mut_for_testing();
            *data.tx_signatures_mut_for_testing() =
                vec![Signature::new_secure(data.intent_message(), &unknown_key).into()];
        },
        |err| {
            assert_matches!(err, SuiError::SignerSignatureAbsent { .. });
        },
    )
    .await;
}

#[sim_test]
async fn test_handle_transfer_transaction_no_signature() {
    do_transaction_test(
        1,
        |_| {},
        |tx| {
            *tx.data_mut_for_testing().tx_signatures_mut_for_testing() = vec![];
        },
        |err| {
            assert_matches!(
                err,
                SuiError::SignerSignatureNumberMismatch {
                    expected: 1,
                    actual: 0
                }
            );
        },
    )
    .await;
}

#[sim_test]
async fn test_handle_transfer_transaction_extra_signature() {
    do_transaction_test(
        1,
        |_| {},
        |tx| {
            let sigs = tx.data_mut_for_testing().tx_signatures_mut_for_testing();
            sigs.push(sigs[0].clone());
        },
        |err| {
            assert_matches!(
                err,
                SuiError::SignerSignatureNumberMismatch {
                    expected: 1,
                    actual: 2
                }
            );
        },
    )
    .await;
}

// TODO: verify that these cases are not exploitable via consensus input
#[sim_test]
async fn test_empty_sender_signed_data() {
    do_transaction_test(
        0,
        |_| {},
        |tx| {
            let data = tx.data_mut_for_testing();
            data.inner_vec_mut_for_testing().clear();
        },
        |err| {
            assert_matches!(
                err,
                SuiError::UserInputError {
                    error: UserInputError::Unsupported { .. }
                }
            );
        },
    )
    .await;
}

#[sim_test]
async fn test_multiple_sender_signed_data() {
    do_transaction_test(
        0,
        |_| {},
        |tx| {
            let data = tx.data_mut_for_testing();
            let tx_vec = data.inner_vec_mut_for_testing();
            assert_eq!(tx_vec.len(), 1);
            let mut new = tx_vec[0].clone();
            // make sure second message has unique digest
            *new.intent_message.value.expiration_mut_for_testing() =
                TransactionExpiration::Epoch(123);
            tx_vec.push(new);
        },
        |err| {
            assert_matches!(
                err,
                SuiError::UserInputError {
                    error: UserInputError::Unsupported { .. }
                }
            );
        },
    )
    .await;
}

#[sim_test]
async fn test_duplicate_sender_signed_data() {
    do_transaction_test(
        0,
        |_| {},
        |tx| {
            let data = tx.data_mut_for_testing();
            let tx_vec = data.inner_vec_mut_for_testing();
            assert_eq!(tx_vec.len(), 1);
            let new = tx_vec[0].clone();
            tx_vec.push(new);
        },
        |err| {
            assert_matches!(
                err,
                SuiError::UserInputError {
                    error: UserInputError::Unsupported { .. }
                }
            );
        },
    )
    .await;
}

#[sim_test]
async fn test_empty_gas_data() {
    do_transaction_test_skip_cert_checks(
        0,
        |tx| {
            tx.gas_data_mut().payment = vec![];
        },
        |_| {},
        |err| {
            assert_matches!(
                err,
                SuiError::UserInputError {
                    error: UserInputError::MissingGasPayment
                }
            );
        },
    )
    .await;
}

#[sim_test]
async fn test_duplicate_gas_data() {
    do_transaction_test_skip_cert_checks(
        0,
        |tx| {
            let gas_data = tx.gas_data_mut();
            let new_gas = gas_data.payment[0];
            gas_data.payment.push(new_gas);
        },
        |_| {},
        |err| {
            assert_matches!(
                err,
                SuiError::UserInputError {
                    error: UserInputError::MutableObjectUsedMoreThanOnce { .. }
                }
            );
        },
    )
    .await;
}

#[sim_test]
async fn test_gas_wrong_owner_matches_sender() {
    do_transaction_test(
        1,
        |tx| {
            let gas_data = tx.gas_data_mut();
            let (new_addr, _): (_, AccountKeyPair) = get_key_pair();
            gas_data.owner = new_addr;
            *tx.sender_mut_for_testing() = new_addr;
        },
        |_| {},
        |err| {
            assert_matches!(err, SuiError::SignerSignatureAbsent { .. });
        },
    )
    .await;
}

#[sim_test]
async fn test_gas_wrong_owner() {
    do_transaction_test(
        1,
        |tx| {
            let gas_data = tx.gas_data_mut();
            let (new_addr, _): (_, AccountKeyPair) = get_key_pair();
            gas_data.owner = new_addr;
        },
        |_| {},
        |err| {
            assert_matches!(
                err,
                SuiError::SignerSignatureNumberMismatch {
                    expected: 2,
                    actual: 1
                }
            );
        },
    )
    .await;
}

#[sim_test]
async fn test_user_sends_system_transaction() {
    do_transaction_test_skip_cert_checks(
        0,
        |tx| {
            *tx.kind_mut() = TransactionKind::Genesis(GenesisTransaction { objects: vec![] });
        },
        |_| {},
        |err| {
            assert_matches!(
                err,
                SuiError::UserInputError {
                    error: UserInputError::Unsupported { .. }
                }
            );
        },
    )
    .await;
}

pub fn init_transfer_transaction(
    pre_sign_mutations: impl FnOnce(&mut TransactionData),
    sender: SuiAddress,
    secret: &AccountKeyPair,
    recipient: SuiAddress,
    object_ref: ObjectRef,
    gas_object_ref: ObjectRef,
    gas_budget: u64,
    gas_price: u64,
) -> Transaction {
    let mut data = TransactionData::new_transfer(
        recipient,
        object_ref,
        sender,
        gas_object_ref,
        gas_budget,
        gas_price,
    );
    pre_sign_mutations(&mut data);
    to_sender_signed_transaction(data, secret)
}

async fn do_transaction_test_skip_cert_checks(
    expected_sig_errors: u64,
    pre_sign_mutations: impl FnOnce(&mut TransactionData),
    post_sign_mutations: impl FnOnce(&mut Transaction),
    err_check: impl Fn(&SuiError),
) {
    do_transaction_test_impl(
        expected_sig_errors,
        false,
        pre_sign_mutations,
        post_sign_mutations,
        err_check,
    )
    .await
}

async fn do_transaction_test(
    expected_sig_errors: u64,
    pre_sign_mutations: impl FnOnce(&mut TransactionData),
    post_sign_mutations: impl FnOnce(&mut Transaction),
    err_check: impl Fn(&SuiError),
) {
    do_transaction_test_impl(
        expected_sig_errors,
        true,
        pre_sign_mutations,
        post_sign_mutations,
        err_check,
    )
    .await
}

async fn do_transaction_test_impl(
    _expected_sig_errors: u64,
    check_forged_cert: bool,
    pre_sign_mutations: impl FnOnce(&mut TransactionData),
    post_sign_mutations: impl FnOnce(&mut Transaction),
    err_check: impl Fn(&SuiError),
) {
    telemetry_subscribers::init_for_testing();
    let (sender, sender_key): (_, AccountKeyPair) = get_key_pair();
    let recipient = dbg_addr(2);
    let object_id = ObjectID::random();
    let gas_object_id = ObjectID::random();
    let authority_state =
        init_state_with_ids(vec![(sender, object_id), (sender, gas_object_id)]).await;
    let rgp = authority_state.reference_gas_price_for_testing().unwrap();
    let object = authority_state
        .get_object(&object_id)
        .await
        .unwrap()
        .unwrap();
    let gas_object = authority_state
        .get_object(&gas_object_id)
        .await
        .unwrap()
        .unwrap();

    let mut transfer_transaction = init_transfer_transaction(
        pre_sign_mutations,
        sender,
        &sender_key,
        recipient,
        object.compute_object_reference(),
        gas_object.compute_object_reference(),
        rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        rgp,
    );

    let consensus_address = "/ip4/127.0.0.1/tcp/0/http".parse().unwrap();

    let server = AuthorityServer::new_for_test(
        "/ip4/127.0.0.1/tcp/0/http".parse().unwrap(),
        authority_state.clone(),
        consensus_address,
    );

    let server_handle = server.spawn_for_test().await.unwrap();

    let client = NetworkAuthorityClient::connect(server_handle.address())
        .await
        .unwrap();

    post_sign_mutations(&mut transfer_transaction);

    let err = client
        .handle_transaction(transfer_transaction.clone())
        .await
        .unwrap_err();
    err_check(&err);

    check_locks(authority_state.clone(), vec![object_id]).await;

    // now verify that the same transaction is rejected if a false certificate is somehow formed and sent
    if check_forged_cert {
        let epoch_store = authority_state.epoch_store_for_testing();
        let signed_transaction = VerifiedSignedTransaction::new(
            epoch_store.epoch(),
            VerifiedTransaction::new_unchecked(transfer_transaction),
            authority_state.name,
            &*authority_state.secret,
        );
        let mut agg = StakeAggregator::new(epoch_store.committee().clone());

        let InsertResult::QuorumReached(cert_sig) = agg.insert(signed_transaction.clone().into())
        else {
            panic!("quorum expected");
        };

        let plain_tx = signed_transaction.into_inner();

        let ct = CertifiedTransaction::new_from_data_and_sig(plain_tx.into_data(), cert_sig);

        let err = client.handle_certificate_v2(ct.clone()).await.unwrap_err();
        err_check(&err);
        epoch_store.clear_signature_cache();
        let err = client.handle_certificate_v2(ct.clone()).await.unwrap_err();
        err_check(&err);
    }
}

#[sim_test]
async fn test_zklogin_transfer_with_bad_ephemeral_sig() {
    do_zklogin_transaction_test(
        1,
        |_| {},
        |tx| {
            let data = tx.data_mut_for_testing();
            let intent_message = data.intent_message().clone();
            let sigs = data.tx_signatures_mut_for_testing();
            let GenericSignature::ZkLoginAuthenticator(zklogin) = sigs.get_mut(0).unwrap() else {
                panic!();
            };

            let (_unknown_address, unknown_key): (_, AccountKeyPair) = get_key_pair();
            let sig = Signature::new_secure(&intent_message, &unknown_key);
            *zklogin.user_signature_mut_for_testing() = sig;
        },
    )
    .await;
}

fn zklogin_key_pair_and_inputs() -> Vec<(Ed25519KeyPair, ZkLoginInputs)> {
    let key1 = Ed25519KeyPair::generate(&mut StdRng::from_seed([1; 32]));
    let key2 = Ed25519KeyPair::generate(&mut StdRng::from_seed([2; 32]));

    let inputs1 = ZkLoginInputs::from_json("{\"proofPoints\":{\"a\":[\"7351610957585487046328875967050889651854514987235893782501043846344306437586\",\"15901581830174345085102528605366245320934422564305327249129736514949843983391\",\"1\"],\"b\":[[\"8511334686125322419369086121569737536249817670014553268281989325333085952301\",\"4879445774811020644521006463993914729416121646921376735430388611804034116132\"],[\"17435652898871739253945717312312680537810513841582909477368887889905134847157\",\"14885460127400879557124294989610467103783286587437961743305395373299049315863\"],[\"1\",\"0\"]],\"c\":[\"18935582624804960299209074901817240117999581542763303721451852621662183299378\",\"5367019427921492326304024952457820199970536888356564030410757345854117465786\",\"1\"]},\"issBase64Details\":{\"value\":\"wiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiw\",\"indexMod4\":2},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ\"}", "20794788559620669596206457022966176986688727876128223628113916380927502737911").unwrap();
    let inputs2 = ZkLoginInputs::from_json("{\"proofPoints\":{\"a\":[\"7351610957585487046328875967050889651854514987235893782501043846344306437586\",\"15901581830174345085102528605366245320934422564305327249129736514949843983391\",\"1\"],\"b\":[[\"8511334686125322419369086121569737536249817670014553268281989325333085952301\",\"4879445774811020644521006463993914729416121646921376735430388611804034116132\"],[\"17435652898871739253945717312312680537810513841582909477368887889905134847157\",\"14885460127400879557124294989610467103783286587437961743305395373299049315863\"],[\"1\",\"0\"]],\"c\":[\"18935582624804960299209074901817240117999581542763303721451852621662183299378\",\"5367019427921492326304024952457820199970536888356564030410757345854117465786\",\"1\"]},\"issBase64Details\":{\"value\":\"wiaXNzIjoiaHR0cHM6Ly9pZC50d2l0Y2gudHYvb2F1dGgyIiw\",\"indexMod4\":2},\"headerBase64\":\"eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ\"}", "20794788559620669596206457022966176986688727876128223628113916380927502737911").unwrap();

    vec![(key1, inputs1), (key2, inputs2)]
}

#[sim_test]
async fn zklogin_test_cached_proof_wrong_key() {
    telemetry_subscribers::init_for_testing();
    let (
        mut object_ids,
        gas_object_ids,
        authority_state,
        _epoch_store,
        transfer_transaction,
        metrics,
        _server,
        client,
    ) = setup_zklogin_network(|_| {}).await;

    assert!(client
        .handle_transaction(transfer_transaction)
        .await
        .is_ok());

    /*
    assert_eq!(
        epoch_store
            .signature_verifier
            .metrics
            .zklogin_inputs_cache_misses
            .get(),
        1
    );
    */

    let (ephemeral_key, zklogin) = &zklogin_key_pair_and_inputs()[0];
    let sender = SuiAddress::try_from_padded(zklogin).unwrap();
    let recipient = dbg_addr(2);

    let mut transfer_transaction2 = init_zklogin_transfer(
        &authority_state,
        object_ids[2],
        gas_object_ids[2],
        recipient,
        sender,
        |_| {},
        ephemeral_key,
        zklogin,
    )
    .await;

    let intent_message = transfer_transaction2.data().intent_message().clone();
    match &mut transfer_transaction2
        .data_mut_for_testing()
        .tx_signatures_mut_for_testing()[0]
    {
        GenericSignature::ZkLoginAuthenticator(zklogin) => {
            let (_unknown_address, unknown_key): (_, AccountKeyPair) = get_key_pair();
            // replace the signature with a bogus one
            *zklogin.user_signature_mut_for_testing() =
                Signature::new_secure(&intent_message, &unknown_key);
        }
        _ => panic!(),
    }

    // This tx should fail, but passes because we skip the ephemeral sig check when hitting the zklogin check!
    assert!(client
        .handle_transaction(transfer_transaction2)
        .await
        .is_err());

    // TODO: re-enable when cache is re-enabled.
    /*
    assert_eq!(
        epoch_store
            .signature_verifier
            .metrics
            .zklogin_inputs_cache_hits
            .get(),
        1
    );
    */

    assert_eq!(metrics.signature_errors.get(), 1);

    object_ids.remove(0); // first object was successfully locked.
    check_locks(authority_state, object_ids).await;
}

async fn do_zklogin_transaction_test(
    expected_sig_errors: u64,
    pre_sign_mutations: impl FnOnce(&mut TransactionData),
    post_sign_mutations: impl FnOnce(&mut Transaction),
) {
    let (
        object_ids,
        _gas_object_id,
        authority_state,
        _epoch_store,
        mut transfer_transaction,
        metrics,
        _server,
        client,
    ) = setup_zklogin_network(pre_sign_mutations).await;

    post_sign_mutations(&mut transfer_transaction);

    assert!(client
        .handle_transaction(transfer_transaction)
        .await
        .is_err());

    // TODO: re-enable when cache is re-enabled.
    /*
    assert_eq!(
        epoch_store
            .signature_verifier
            .metrics
            .zklogin_inputs_cache_misses
            .get(),
        1
    );
    */

    assert_eq!(metrics.signature_errors.get(), expected_sig_errors);

    check_locks(authority_state, object_ids).await;
}

async fn check_locks(authority_state: Arc<AuthorityState>, object_ids: Vec<ObjectID>) {
    for object_id in object_ids {
        let object = authority_state
            .get_object(&object_id)
            .await
            .unwrap()
            .unwrap();
        assert!(authority_state
            .get_transaction_lock(
                &object.compute_object_reference(),
                &authority_state.epoch_store_for_testing()
            )
            .await
            .unwrap()
            .is_none());
    }
}

async fn setup_zklogin_network(
    pre_sign_mutations: impl FnOnce(&mut TransactionData),
) -> (
    Vec<ObjectID>, // objects
    Vec<ObjectID>, // gas objects
    Arc<AuthorityState>,
    Guard<Arc<AuthorityPerEpochStore>>,
    sui_types::message_envelope::Envelope<SenderSignedData, sui_types::crypto::EmptySignInfo>,
    Arc<crate::authority_server::ValidatorServiceMetrics>,
    AuthorityServerHandle,
    NetworkAuthorityClient,
) {
    let (ephemeral_key, zklogin) = &zklogin_key_pair_and_inputs()[0];

    let sender = SuiAddress::try_from_unpadded(zklogin).unwrap();

    let recipient = dbg_addr(2);
    let objects: Vec<_> = (0..10).map(|_| (sender, ObjectID::random())).collect();
    let gas_objects: Vec<_> = (0..10).map(|_| (sender, ObjectID::random())).collect();
    let object_ids: Vec<_> = objects.iter().map(|(_, id)| *id).collect();
    let gas_object_ids: Vec<_> = gas_objects.iter().map(|(_, id)| *id).collect();

    let authority_state =
        init_state_with_ids(objects.into_iter().chain(gas_objects).collect::<Vec<_>>()).await;

    let object_id = object_ids[0];
    let gas_object_id = gas_object_ids[0];
    let jwks = "{\"keys\":[{\"alg\":\"RS256\",\"e\":\"AQAB\",\"kid\":\"1\",\"kty\":\"RSA\",\"n\":\"6lq9MQ-q6hcxr7kOUp-tHlHtdcDsVLwVIw13iXUCvuDOeCi0VSuxCCUY6UmMjy53dX00ih2E4Y4UvlrmmurK0eG26b-HMNNAvCGsVXHU3RcRhVoHDaOwHwU72j7bpHn9XbP3Q3jebX6KIfNbei2MiR0Wyb8RZHE-aZhRYO8_-k9G2GycTpvc-2GBsP8VHLUKKfAs2B6sW3q3ymU6M0L-cFXkZ9fHkn9ejs-sqZPhMJxtBPBxoUIUQFTgv4VXTSv914f_YkNw-EjuwbgwXMvpyr06EyfImxHoxsZkFYB-qBYHtaMxTnFsZBr6fn8Ha2JqT1hoP7Z5r5wxDu3GQhKkHw\",\"use\":\"sig\"}]}";
    let jwks = parse_jwks(jwks.as_bytes(), &OIDCProvider::Twitch).unwrap();
    let epoch_store = authority_state.epoch_store_for_testing();
    epoch_store.update_authenticator_state(&AuthenticatorStateUpdate {
        epoch: 0,
        round: 0,
        new_active_jwks: jwks
            .into_iter()
            .map(|(jwk_id, jwk)| ActiveJwk {
                jwk_id,
                jwk,
                epoch: 0,
            })
            .collect(),
        authenticator_obj_initial_shared_version: 1.into(),
    });

    let transfer_transaction = init_zklogin_transfer(
        &authority_state,
        object_id,
        gas_object_id,
        recipient,
        sender,
        pre_sign_mutations,
        ephemeral_key,
        zklogin,
    )
    .await;

    let consensus_address = "/ip4/127.0.0.1/tcp/0/http".parse().unwrap();

    let server = AuthorityServer::new_for_test(
        "/ip4/127.0.0.1/tcp/0/http".parse().unwrap(),
        authority_state.clone(),
        consensus_address,
    );
    let metrics = server.metrics.clone();

    let server_handle = server.spawn_for_test().await.unwrap();

    let client = NetworkAuthorityClient::connect(server_handle.address())
        .await
        .unwrap();
    (
        object_ids,
        gas_object_ids,
        authority_state,
        epoch_store,
        transfer_transaction,
        metrics,
        server_handle,
        client,
    )
}

async fn init_zklogin_transfer(
    authority_state: &Arc<AuthorityState>,
    object_id: ObjectID,
    gas_object_id: ObjectID,
    recipient: SuiAddress,
    sender: SuiAddress,
    pre_sign_mutations: impl FnOnce(&mut TransactionData),
    ephemeral_key: &Ed25519KeyPair,
    zklogin: &ZkLoginInputs,
) -> sui_types::message_envelope::Envelope<SenderSignedData, sui_types::crypto::EmptySignInfo> {
    let rgp = authority_state.reference_gas_price_for_testing().unwrap();
    let object = authority_state
        .get_object(&object_id)
        .await
        .unwrap()
        .unwrap();
    let gas_object = authority_state
        .get_object(&gas_object_id)
        .await
        .unwrap()
        .unwrap();
    let object_ref = object.compute_object_reference();
    let gas_object_ref = gas_object.compute_object_reference();
    let gas_budget = rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER;
    let mut data = TransactionData::new_transfer(
        recipient,
        object_ref,
        sender,
        gas_object_ref,
        gas_budget,
        rgp,
    );
    pre_sign_mutations(&mut data);
    let mut tx = to_sender_signed_transaction(data, ephemeral_key);
    let GenericSignature::Signature(signature) =
        tx.data_mut_for_testing().tx_signatures_mut_for_testing()[0].clone()
    else {
        panic!();
    };
    let authenticator = GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(
        zklogin.clone(),
        10,
        signature,
    ));
    tx.data_mut_for_testing().tx_signatures_mut_for_testing()[0] = authenticator;
    tx
}

#[tokio::test]
async fn zk_multisig_test() {
    telemetry_subscribers::init_for_testing();

    // User generate a multisig account with no zklogin signer.
    let keys = sui_types::utils::keys();
    let pk1 = keys[0].public();
    let pk2 = keys[1].public();
    let pk3 = keys[2].public();
    let multisig_pk = MultiSigPublicKey::new(
        vec![pk1.clone(), pk2.clone(), pk3.clone()],
        vec![1, 1, 1],
        2,
    )
    .unwrap();
    let victim_addr = SuiAddress::from(&multisig_pk);

    let recipient = dbg_addr(2);
    let object_id = ObjectID::random();
    let gas_object_id = ObjectID::random();
    let authority_state =
        init_state_with_ids(vec![(victim_addr, object_id), (victim_addr, gas_object_id)]).await;

    let jwks = parse_jwks(DEFAULT_JWK_BYTES, &OIDCProvider::Twitch).unwrap();
    let epoch_store = authority_state.epoch_store_for_testing();
    epoch_store.update_authenticator_state(&AuthenticatorStateUpdate {
        epoch: 0,
        round: 0,
        new_active_jwks: jwks
            .into_iter()
            .map(|(jwk_id, jwk)| ActiveJwk {
                jwk_id,
                jwk,
                epoch: 0,
            })
            .collect(),
        authenticator_obj_initial_shared_version: 1.into(),
    });

    let rgp = authority_state.reference_gas_price_for_testing().unwrap();
    let object = authority_state
        .get_object(&object_id)
        .await
        .unwrap()
        .unwrap();
    let gas_object = authority_state
        .get_object(&gas_object_id)
        .await
        .unwrap()
        .unwrap();

    let data = TransactionData::new_transfer(
        recipient,
        object.compute_object_reference(),
        victim_addr,
        gas_object.compute_object_reference(),
        rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        rgp,
    );

    // Poof of concept for bypassing zklogin verification starts here.
    // Step 1. construct 2 zklogin signatures
    // read in test files that has a list of matching zklogin_inputs and its ephemeral private keys.

    let file = std::fs::File::open("../sui-types/src/unit_tests/zklogin_test_vectors.json")
        .expect("Unable to open file");
    let test_datum: Vec<TestData> = serde_json::from_reader(file).unwrap();
    let mut pks = vec![];
    let mut kps_and_zklogin_inputs = vec![];
    for test in test_datum {
        let kp = SuiKeyPair::decode_base64(&test.kp).unwrap();
        let inputs = ZkLoginInputs::from_json(&test.zklogin_inputs, &test.address_seed).unwrap();
        let pk_zklogin = PublicKey::from_zklogin_inputs(&inputs).unwrap();
        pks.push(pk_zklogin);
        kps_and_zklogin_inputs.push((kp, inputs));
    }

    let mut zklogin_sigs = vec![];
    for (kp, inputs) in kps_and_zklogin_inputs {
        let intent_message = IntentMessage::new(Intent::sui_transaction(), data.clone());
        let eph_sig = Signature::new_secure(&intent_message, &kp);
        let zklogin_sig =
            GenericSignature::ZkLoginAuthenticator(ZkLoginAuthenticator::new(inputs, 10, eph_sig));
        zklogin_sigs.push(zklogin_sig);
    }

    // Step 2. Construct the fake multisig with the zklogin signatures.
    let multisig = MultiSig::new(
        vec![
            zklogin_sigs[0].clone().to_compressed().unwrap(),
            zklogin_sigs[1].clone().to_compressed().unwrap(),
        ], // zklogin sigs
        3,
        multisig_pk,
    );

    let generic_sig = GenericSignature::MultiSig(multisig);

    let transfer_transaction = Transaction::from_generic_sig_data(data, vec![generic_sig]);

    let consensus_address = "/ip4/127.0.0.1/tcp/0/http".parse().unwrap();

    let server = AuthorityServer::new_for_test(
        "/ip4/127.0.0.1/tcp/0/http".parse().unwrap(),
        authority_state.clone(),
        consensus_address,
    );

    let server_handle = server.spawn_for_test().await.unwrap();

    let client = NetworkAuthorityClient::connect(server_handle.address())
        .await
        .unwrap();

    let err = client
        .handle_transaction(transfer_transaction.clone())
        .await;

    assert!(dbg!(err).is_err());

    check_locks(authority_state.clone(), vec![object_id]).await;
}

#[tokio::test]
async fn test_oversized_txn() {
    telemetry_subscribers::init_for_testing();
    let (sender, sender_key): (_, AccountKeyPair) = get_key_pair();
    let recipient = dbg_addr(2);
    let object_id = ObjectID::random();
    let authority_state = init_state_with_ids(vec![(sender, object_id)]).await;
    let max_txn_size = authority_state
        .epoch_store_for_testing()
        .protocol_config()
        .max_tx_size_bytes() as usize;
    let object = authority_state
        .get_object(&object_id)
        .await
        .unwrap()
        .unwrap();
    let obj_ref = object.compute_object_reference();

    // Construct an oversized txn.
    let pt = {
        let mut builder = ProgrammableTransactionBuilder::new();
        // Put a lot of commands in the txn so it's large.
        for _ in 0..(1024 * 16) {
            builder.transfer_object(recipient, obj_ref).unwrap();
        }
        builder.finish()
    };

    let txn_data = TransactionData::new_programmable(sender, vec![obj_ref], pt, 0, 0);

    let txn = to_sender_signed_transaction(txn_data, &sender_key);
    let tx_size = bcs::serialized_size(&txn).unwrap();

    // Making sure the txn is larger than the max txn size.
    assert!(tx_size > max_txn_size);

    let consensus_address = "/ip4/127.0.0.1/tcp/0/http".parse().unwrap();

    let server = AuthorityServer::new_for_test(
        "/ip4/127.0.0.1/tcp/0/http".parse().unwrap(),
        authority_state.clone(),
        consensus_address,
    );

    let server_handle = server.spawn_for_test().await.unwrap();

    let client = NetworkAuthorityClient::connect(server_handle.address())
        .await
        .unwrap();

    let res = client.handle_transaction(txn).await;
    // The txn should be rejected due to its size.
    assert!(res
        .err()
        .unwrap()
        .to_string()
        .contains("serialized transaction size exceeded maximum"));
}

#[tokio::test]
async fn test_very_large_certificate() {
    telemetry_subscribers::init_for_testing();
    let (sender, sender_key): (_, AccountKeyPair) = get_key_pair();
    let recipient = dbg_addr(2);
    let object_id = ObjectID::random();
    let gas_object_id = ObjectID::random();
    let authority_state =
        init_state_with_ids(vec![(sender, object_id), (sender, gas_object_id)]).await;
    let rgp = authority_state.reference_gas_price_for_testing().unwrap();
    let object = authority_state
        .get_object(&object_id)
        .await
        .unwrap()
        .unwrap();
    let gas_object = authority_state
        .get_object(&gas_object_id)
        .await
        .unwrap()
        .unwrap();

    let transfer_transaction = init_transfer_transaction(
        |_| {},
        sender,
        &sender_key,
        recipient,
        object.compute_object_reference(),
        gas_object.compute_object_reference(),
        rgp * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        rgp,
    );

    let consensus_address = "/ip4/127.0.0.1/tcp/0/http".parse().unwrap();

    let server = AuthorityServer::new_for_test(
        "/ip4/127.0.0.1/tcp/0/http".parse().unwrap(),
        authority_state.clone(),
        consensus_address,
    );

    let server_handle = server.spawn_for_test().await.unwrap();

    let client = NetworkAuthorityClient::connect(server_handle.address())
        .await
        .unwrap();

    let auth_sig = client
        .handle_transaction(transfer_transaction.clone())
        .await
        .unwrap()
        .status
        .into_signed_for_testing();

    let signatures: BTreeMap<_, _> = vec![auth_sig]
        .into_iter()
        .map(|a| (a.authority, a.signature))
        .collect();

    // Insert a lot into the bitmap so the cert is very large, while the txn inside is reasonably sized.
    let mut signers_map = roaring::bitmap::RoaringBitmap::new();
    signers_map.insert_range(0..52108864);
    let sigs: Vec<AuthoritySignature> = signatures.into_values().collect();

    let quorum_signature = sui_types::crypto::AuthorityQuorumSignInfo {
        epoch: 0,
        signature: sui_types::crypto::AggregateAuthoritySignature::aggregate(&sigs)
            .map_err(|e| SuiError::InvalidSignature {
                error: e.to_string(),
            })
            .expect("Validator returned invalid signature"),
        signers_map,
    };
    let cert = sui_types::message_envelope::Envelope::new_from_data_and_sig(
        transfer_transaction.into_data(),
        quorum_signature,
    );

    let res = client.handle_certificate_v2(cert).await;
    assert!(res.is_err());
    let err = res.err().unwrap();
    // The resulting error should be a RpcError with a message length too large.
    assert!(
        matches!(err, SuiError::RpcError(..))
            && err.to_string().contains("message length too large")
    );
}
