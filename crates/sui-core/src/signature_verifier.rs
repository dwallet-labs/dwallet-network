// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use either::Either;
use fastcrypto_zkp::bn254::zk_login::JwkId;
use fastcrypto_zkp::bn254::zk_login::{OIDCProvider, JWK};
use fastcrypto_zkp::bn254::zk_login_api::ZkLoginEnv;
use futures::pin_mut;
use im::hashmap::HashMap as ImHashMap;
use itertools::izip;
use lru::LruCache;
use mysten_metrics::monitored_scope;
use parking_lot::{Mutex, MutexGuard, RwLock};
use prometheus::{register_int_counter_with_registry, IntCounter, Registry};
use shared_crypto::intent::Intent;
use std::hash::Hash;
use std::sync::Arc;
use sui_types::digests::SenderSignedDataDigest;
use sui_types::digests::ZKLoginInputsDigest;
use sui_types::messages_signature_mpc::{
    SignatureMPCMessageSummary, SignedSignatureMPCMessageSummary, SignedSignatureMPCOutput,
};
use sui_types::transaction::SenderSignedData;
use sui_types::{
    committee::Committee,
    crypto::{AuthoritySignInfoTrait, VerificationObligation},
    digests::CertificateDigest,
    error::{SuiError, SuiResult},
    message_envelope::{AuthenticatedMessage, Message},
    messages_checkpoint::SignedCheckpointSummary,
    signature::VerifyParams,
    transaction::{CertifiedTransaction, VerifiedCertificate},
};
use tap::TapFallible;
use tokio::runtime::Handle;
use tokio::{
    sync::oneshot,
    time::{timeout, Duration},
};
use tracing::debug;

// Maximum amount of time we wait for a batch to fill up before verifying a partial batch.
const BATCH_TIMEOUT_MS: Duration = Duration::from_millis(10);

// Maximum size of batch to verify. Increasing this value will slightly improve CPU utilization
// (batching starts to hit steeply diminishing marginal returns around batch sizes of 16), at the
// cost of slightly increasing latency (BATCH_TIMEOUT_MS will be hit more frequently if system is
// not heavily loaded).
const MAX_BATCH_SIZE: usize = 8;

type Sender = oneshot::Sender<SuiResult<VerifiedCertificate>>;

struct CertBuffer {
    certs: Vec<CertifiedTransaction>,
    senders: Vec<Sender>,
    id: u64,
}

impl CertBuffer {
    fn new(capacity: usize) -> Self {
        Self {
            certs: Vec::with_capacity(capacity),
            senders: Vec::with_capacity(capacity),
            id: 0,
        }
    }

    // Function consumes MutexGuard, therefore releasing the lock after mem swap is done
    fn take_and_replace(mut guard: MutexGuard<'_, Self>) -> Self {
        let this = &mut *guard;
        let mut new = CertBuffer::new(this.capacity());
        new.id = this.id + 1;
        std::mem::swap(&mut new, this);
        new
    }

    fn capacity(&self) -> usize {
        debug_assert_eq!(self.certs.capacity(), self.senders.capacity());
        self.certs.capacity()
    }

    fn len(&self) -> usize {
        debug_assert_eq!(self.certs.len(), self.senders.len());
        self.certs.len()
    }

    fn push(&mut self, tx: Sender, cert: CertifiedTransaction) {
        self.senders.push(tx);
        self.certs.push(cert);
    }
}

/// Verifies signatures in ways that faster than verifying each signature individually.
/// - BLS signatures - caching and batch verification.
/// - User signed data - caching.
pub struct SignatureVerifier {
    committee: Arc<Committee>,
    certificate_cache: VerifiedDigestCache<CertificateDigest>,
    signed_data_cache: VerifiedDigestCache<SenderSignedDataDigest>,
    zklogin_inputs_cache: VerifiedDigestCache<ZKLoginInputsDigest>,

    /// Map from JwkId (iss, kid) to the fetched JWK for that key.
    /// We use an immutable data structure because verification of ZKLogins may be slow, so we
    /// don't want to pass a reference to the map to the verify method, since that would lead to a
    /// lengthy critical section. Instead, we use an immutable data structure which can be cloned
    /// very cheaply.
    jwks: RwLock<ImHashMap<JwkId, JWK>>,

    /// Params that contains a list of supported providers for ZKLogin and the environment (prod/test) the code runs in.
    zk_login_params: ZkLoginParams,

    queue: Mutex<CertBuffer>,
    pub metrics: Arc<SignatureVerifierMetrics>,
}

/// Contains two parameters to pass in to verify a ZkLogin signature.
#[derive(Clone)]
struct ZkLoginParams {
    /// A list of supported OAuth providers for ZkLogin.
    pub supported_providers: Vec<OIDCProvider>,
    /// The environment (prod/test) the code runs in. It decides which verifying key to use in fastcrypto.
    pub env: ZkLoginEnv,
    pub verify_legacy_zklogin_address: bool,
    pub accept_zklogin_in_multisig: bool,
}

impl SignatureVerifier {
    pub fn new_with_batch_size(
        committee: Arc<Committee>,
        batch_size: usize,
        metrics: Arc<SignatureVerifierMetrics>,
        supported_providers: Vec<OIDCProvider>,
        env: ZkLoginEnv,
        verify_legacy_zklogin_address: bool,
        accept_zklogin_in_multisig: bool,
    ) -> Self {
        Self {
            committee,
            certificate_cache: VerifiedDigestCache::new(
                metrics.certificate_signatures_cache_hits.clone(),
                metrics.certificate_signatures_cache_misses.clone(),
                metrics.certificate_signatures_cache_evictions.clone(),
            ),
            signed_data_cache: VerifiedDigestCache::new(
                metrics.signed_data_cache_hits.clone(),
                metrics.signed_data_cache_misses.clone(),
                metrics.signed_data_cache_evictions.clone(),
            ),
            zklogin_inputs_cache: VerifiedDigestCache::new(
                metrics.zklogin_inputs_cache_hits.clone(),
                metrics.zklogin_inputs_cache_misses.clone(),
                metrics.zklogin_inputs_cache_evictions.clone(),
            ),
            jwks: Default::default(),
            queue: Mutex::new(CertBuffer::new(batch_size)),
            metrics,
            zk_login_params: ZkLoginParams {
                supported_providers,
                env,
                verify_legacy_zklogin_address,
                accept_zklogin_in_multisig,
            },
        }
    }

    pub fn new(
        committee: Arc<Committee>,
        metrics: Arc<SignatureVerifierMetrics>,
        supported_providers: Vec<OIDCProvider>,
        zklogin_env: ZkLoginEnv,
        verify_legacy_zklogin_address: bool,
        accept_zklogin_in_multisig: bool,
    ) -> Self {
        Self::new_with_batch_size(
            committee,
            MAX_BATCH_SIZE,
            metrics,
            supported_providers,
            zklogin_env,
            verify_legacy_zklogin_address,
            accept_zklogin_in_multisig,
        )
    }

    /// Verifies all certs, returns Ok only if all are valid.
    pub fn verify_certs_and_checkpoints_and_signature_mpc_messages(
        &self,
        certs: Vec<CertifiedTransaction>,
        checkpoints: Vec<SignedCheckpointSummary>,
        signature_mpc_messages: Vec<SignedSignatureMPCMessageSummary>,
        signed_dkg_signature_mpc_outputs: Vec<SignedSignatureMPCOutput>,
    ) -> SuiResult {
        let certs: Vec<_> = certs
            .into_iter()
            .filter(|cert| !self.certificate_cache.is_cached(&cert.certificate_digest()))
            .collect();

        // Verify only the user sigs of certificates that were not cached already, since whenever we
        // insert a certificate into the cache, it is already verified.
        for cert in &certs {
            self.verify_tx(cert.data())?;
        }
        batch_verify_all_certificates_and_checkpoints_and_signature_mpc_messages(
            &self.committee,
            &certs,
            &checkpoints,
            &signature_mpc_messages,
            &signed_dkg_signature_mpc_outputs,
        )?;
        self.certificate_cache
            .cache_digests(certs.into_iter().map(|c| c.certificate_digest()).collect());
        Ok(())
    }

    /// Verifies one cert asynchronously, in a batch.
    pub async fn verify_cert(&self, cert: CertifiedTransaction) -> SuiResult<VerifiedCertificate> {
        let cert_digest = cert.certificate_digest();
        if self.certificate_cache.is_cached(&cert_digest) {
            return Ok(VerifiedCertificate::new_unchecked(cert));
        }
        self.verify_tx(cert.data())?;
        self.verify_cert_skip_cache(cert)
            .await
            .tap_ok(|_| self.certificate_cache.cache_digest(cert_digest))
    }

    /// exposed as a public method for the benchmarks
    pub async fn verify_cert_skip_cache(
        &self,
        cert: CertifiedTransaction,
    ) -> SuiResult<VerifiedCertificate> {
        // this is the only innocent error we are likely to encounter - filter it before we poison
        // a whole batch.
        if cert.auth_sig().epoch != self.committee.epoch() {
            return Err(SuiError::WrongEpoch {
                expected_epoch: self.committee.epoch(),
                actual_epoch: cert.auth_sig().epoch,
            });
        }

        self.verify_cert_inner(cert).await
    }

    async fn verify_cert_inner(
        &self,
        cert: CertifiedTransaction,
    ) -> SuiResult<VerifiedCertificate> {
        // Cancellation safety: we use parking_lot locks, which cannot be held across awaits.
        // Therefore once the queue has been taken by a thread, it is guaranteed to process the
        // queue and send all results before the future can be cancelled by the caller.
        let (tx, rx) = oneshot::channel();
        pin_mut!(rx);

        let prev_id_or_buffer = {
            let mut queue = self.queue.lock();
            queue.push(tx, cert);
            if queue.len() == queue.capacity() {
                Either::Right(CertBuffer::take_and_replace(queue))
            } else {
                Either::Left(queue.id)
            }
        };
        let prev_id = match prev_id_or_buffer {
            Either::Left(prev_id) => prev_id,
            Either::Right(buffer) => {
                self.metrics.full_batches.inc();
                self.process_queue(buffer).await;
                // unwrap ok - process_queue will have sent the result already
                return rx.try_recv().unwrap();
            }
        };

        if let Ok(res) = timeout(BATCH_TIMEOUT_MS, &mut rx).await {
            // unwrap ok - tx cannot have been dropped without sending a result.
            return res.unwrap();
        }
        self.metrics.timeouts.inc();

        let buffer = {
            let queue = self.queue.lock();
            // check if another thread took the queue while we were re-acquiring lock.
            if prev_id == queue.id {
                debug_assert_ne!(queue.len(), queue.capacity());
                Some(CertBuffer::take_and_replace(queue))
            } else {
                None
            }
        };

        if let Some(buffer) = buffer {
            self.metrics.partial_batches.inc();
            self.process_queue(buffer).await;
            // unwrap ok - process_queue will have sent the result already
            return rx.try_recv().unwrap();
        }

        // unwrap ok - another thread took the queue while we were re-acquiring the lock and is
        // guaranteed to process the queue immediately.
        rx.await.unwrap()
    }

    async fn process_queue(&self, buffer: CertBuffer) {
        let committee = self.committee.clone();
        let metrics = self.metrics.clone();
        Handle::current()
            .spawn_blocking(move || Self::process_queue_sync(committee, metrics, buffer))
            .await
            .expect("Spawn blocking should not fail");
    }

    fn process_queue_sync(
        committee: Arc<Committee>,
        metrics: Arc<SignatureVerifierMetrics>,
        buffer: CertBuffer,
    ) {
        let _scope = monitored_scope("BatchCertificateVerifier::process_queue");

        let results = batch_verify_certificates(&committee, &buffer.certs);
        izip!(
            results.into_iter(),
            buffer.certs.into_iter(),
            buffer.senders.into_iter(),
        )
        .for_each(|(result, cert, tx)| {
            tx.send(match result {
                Ok(()) => {
                    metrics.total_verified_certs.inc();
                    Ok(VerifiedCertificate::new_unchecked(cert))
                }
                Err(e) => {
                    metrics.total_failed_certs.inc();
                    Err(e)
                }
            })
            .ok();
        });
    }

    /// Insert a JWK into the verifier state. Pre-existing entries for a given JwkId will not be
    /// overwritten.
    pub(crate) fn insert_jwk(&self, jwk_id: &JwkId, jwk: &JWK) {
        let mut jwks = self.jwks.write();
        match jwks.entry(jwk_id.clone()) {
            im::hashmap::Entry::Occupied(_) => {
                debug!("JWK with kid {:?} already exists", jwk_id);
            }
            im::hashmap::Entry::Vacant(entry) => {
                debug!("inserting JWK with kid: {:?}", jwk_id);
                entry.insert(jwk.clone());
            }
        }
    }

    pub fn has_jwk(&self, jwk_id: &JwkId, jwk: &JWK) -> bool {
        let jwks = self.jwks.read();
        jwks.get(jwk_id) == Some(jwk)
    }

    pub fn get_jwks(&self) -> ImHashMap<JwkId, JWK> {
        self.jwks.read().clone()
    }

    pub fn verify_tx(&self, signed_tx: &SenderSignedData) -> SuiResult {
        self.signed_data_cache.is_verified(
            signed_tx.full_message_digest(),
            || {
                signed_tx.verify_epoch(self.committee.epoch())?;
                let jwks = self.jwks.read().clone();
                let verify_params = VerifyParams::new(
                    jwks,
                    self.zk_login_params.supported_providers.clone(),
                    self.zk_login_params.env.clone(),
                    self.zk_login_params.verify_legacy_zklogin_address,
                    self.zk_login_params.accept_zklogin_in_multisig,
                );
                signed_tx.verify_message_signature(&verify_params)
            },
            || Ok(()),
        )
    }

    pub fn clear_signature_cache(&self) {
        self.certificate_cache.clear();
        self.signed_data_cache.clear();
        self.zklogin_inputs_cache.clear();
    }
}

pub struct SignatureVerifierMetrics {
    pub certificate_signatures_cache_hits: IntCounter,
    pub certificate_signatures_cache_misses: IntCounter,
    pub certificate_signatures_cache_evictions: IntCounter,
    pub signed_data_cache_hits: IntCounter,
    pub signed_data_cache_misses: IntCounter,
    pub signed_data_cache_evictions: IntCounter,
    pub zklogin_inputs_cache_hits: IntCounter,
    pub zklogin_inputs_cache_misses: IntCounter,
    pub zklogin_inputs_cache_evictions: IntCounter,
    timeouts: IntCounter,
    full_batches: IntCounter,
    partial_batches: IntCounter,
    total_verified_certs: IntCounter,
    total_failed_certs: IntCounter,
}

impl SignatureVerifierMetrics {
    pub fn new(registry: &Registry) -> Arc<Self> {
        Arc::new(Self {
            certificate_signatures_cache_hits: register_int_counter_with_registry!(
                "certificate_signatures_cache_hits",
                "Number of certificates which were known to be verified because of signature cache.",
                registry
            )
            .unwrap(),
            certificate_signatures_cache_misses: register_int_counter_with_registry!(
                "certificate_signatures_cache_misses",
                "Number of certificates which missed the signature cache",
                registry
            )
            .unwrap(),
            certificate_signatures_cache_evictions: register_int_counter_with_registry!(
                "certificate_signatures_cache_evictions",
                "Number of times we evict a pre-existing key were known to be verified because of signature cache.",
                registry
            )
            .unwrap(),
            signed_data_cache_hits: register_int_counter_with_registry!(
                "signed_data_cache_hits",
                "Number of signed data which were known to be verified because of signature cache.",
                registry
            )
            .unwrap(),
            signed_data_cache_misses: register_int_counter_with_registry!(
                "signed_data_cache_misses",
                "Number of signed data which missed the signature cache.",
                registry
            )
            .unwrap(),
            signed_data_cache_evictions: register_int_counter_with_registry!(
                "signed_data_cache_evictions",
                "Number of times we evict a pre-existing signed data were known to be verified because of signature cache.",
                registry
            )
                .unwrap(),
                zklogin_inputs_cache_hits: register_int_counter_with_registry!(
                    "zklogin_inputs_cache_hits",
                    "Number of zklogin signature which were known to be partially verified because of zklogin inputs cache.",
                    registry
                )
                .unwrap(),
                zklogin_inputs_cache_misses: register_int_counter_with_registry!(
                    "zklogin_inputs_cache_misses",
                    "Number of zklogin signatures which missed the zklogin inputs cache.",
                    registry
                )
                .unwrap(),
                zklogin_inputs_cache_evictions: register_int_counter_with_registry!(
                    "zklogin_inputs_cache_evictions",
                    "Number of times we evict a pre-existing zklogin inputs digest that was known to be verified because of zklogin inputs cache.",
                    registry
                )
                .unwrap(),
            timeouts: register_int_counter_with_registry!(
                "async_batch_verifier_timeouts",
                "Number of times batch verifier times out and verifies a partial batch",
                registry
            )
            .unwrap(),
            full_batches: register_int_counter_with_registry!(
                "async_batch_verifier_full_batches",
                "Number of times batch verifier verifies a full batch",
                registry
            )
            .unwrap(),
            partial_batches: register_int_counter_with_registry!(
                "async_batch_verifier_partial_batches",
                "Number of times batch verifier verifies a partial batch",
                registry
            )
            .unwrap(),
            total_verified_certs: register_int_counter_with_registry!(
                "async_batch_verifier_total_verified_certs",
                "Total number of certs batch verifier has verified",
                registry
            )
            .unwrap(),
            total_failed_certs: register_int_counter_with_registry!(
                "async_batch_verifier_total_failed_certs",
                "Total number of certs batch verifier has rejected",
                registry
            )
            .unwrap(),
        })
    }
}

/// Verifies all certificates - if any fail return error.
pub fn batch_verify_all_certificates_and_checkpoints_and_signature_mpc_messages(
    committee: &Committee,
    certs: &[CertifiedTransaction],
    checkpoints: &[SignedCheckpointSummary],
    signature_mpc_messages: &[SignedSignatureMPCMessageSummary],
    signed_dkg_signature_mpc_outputs: &[SignedSignatureMPCOutput],
) -> SuiResult {
    // certs.data() is assumed to be verified already by the caller.

    for ckpt in checkpoints {
        ckpt.data().verify_epoch(committee.epoch())?;
    }

    for msg in signature_mpc_messages {
        msg.data().verify_epoch(committee.epoch())?;
    }

    for msg in signed_dkg_signature_mpc_outputs {
        msg.data().verify_epoch(committee.epoch())?;
    }

    batch_verify(
        committee,
        certs,
        checkpoints,
        signature_mpc_messages,
        signed_dkg_signature_mpc_outputs,
    )
}

/// Verifies certificates in batch mode, but returns a separate result for each cert.
pub fn batch_verify_certificates(
    committee: &Committee,
    certs: &[CertifiedTransaction],
) -> Vec<SuiResult> {
    // certs.data() is assumed to be verified already by the caller.
    let verify_params = VerifyParams::new(
        Default::default(),
        Vec::new(),
        Default::default(),
        true,
        true,
    );
    match batch_verify(committee, certs, &[], &[], &[]) {
        Ok(_) => vec![Ok(()); certs.len()],

        // Verify one by one to find which certs were invalid.
        Err(_) if certs.len() > 1 => certs
            .iter()
            // TODO: verify_signature currently checks the tx sig as well, which might be cached
            // already.
            .map(|c| c.verify_signatures_authenticated(committee, &verify_params))
            .collect(),

        Err(e) => vec![Err(e)],
    }
}

fn batch_verify(
    committee: &Committee,
    certs: &[CertifiedTransaction],
    checkpoints: &[SignedCheckpointSummary],
    signature_mpc_messages: &[SignedSignatureMPCMessageSummary],
    signed_dkg_signature_mpc_outputs: &[SignedSignatureMPCOutput],
) -> SuiResult {
    let mut obligation = VerificationObligation::default();

    for cert in certs {
        let idx = obligation.add_message(cert.data(), cert.epoch(), Intent::sui_app(cert.scope()));
        cert.auth_sig()
            .add_to_verification_obligation(committee, &mut obligation, idx)?;
    }

    for ckpt in checkpoints {
        let idx = obligation.add_message(ckpt.data(), ckpt.epoch(), Intent::sui_app(ckpt.scope()));
        ckpt.auth_sig()
            .add_to_verification_obligation(committee, &mut obligation, idx)?;
    }

    for msg in signature_mpc_messages {
        let idx = obligation.add_message(msg.data(), msg.epoch(), Intent::sui_app(msg.scope()));
        msg.auth_sig()
            .add_to_verification_obligation(committee, &mut obligation, idx)?;
    }

    for msg in signed_dkg_signature_mpc_outputs {
        let idx = obligation.add_message(msg.data(), msg.epoch(), Intent::sui_app(msg.scope()));
        msg.auth_sig()
            .add_to_verification_obligation(committee, &mut obligation, idx)?;
    }

    obligation.verify_all()
}

// Cache up to 20000 verified certs. We will need to tune this number in the future - a decent
// guess to start with is that it should be 10-20 times larger than peak transactions per second,
// on the assumption that we should see most certs twice within about 10-20 seconds at most: Once via RPC, once via consensus.
const VERIFIED_CERTIFICATE_CACHE_SIZE: usize = 20000;

pub struct VerifiedDigestCache<D> {
    inner: RwLock<LruCache<D, ()>>,
    cache_hits_counter: IntCounter,
    cache_misses_counter: IntCounter,
    cache_evictions_counter: IntCounter,
}

impl<D: Hash + Eq + Copy> VerifiedDigestCache<D> {
    pub fn new(
        cache_hits_counter: IntCounter,
        cache_misses_counter: IntCounter,
        cache_evictions_counter: IntCounter,
    ) -> Self {
        Self {
            inner: RwLock::new(LruCache::new(
                std::num::NonZeroUsize::new(VERIFIED_CERTIFICATE_CACHE_SIZE).unwrap(),
            )),
            cache_hits_counter,
            cache_misses_counter,
            cache_evictions_counter,
        }
    }

    pub fn is_cached(&self, digest: &D) -> bool {
        let inner = self.inner.read();
        if inner.contains(digest) {
            self.cache_hits_counter.inc();
            true
        } else {
            self.cache_misses_counter.inc();
            false
        }
    }

    pub fn cache_digest(&self, digest: D) {
        let mut inner = self.inner.write();
        if let Some(old) = inner.push(digest, ()) {
            if old.0 != digest {
                self.cache_evictions_counter.inc();
            }
        }
    }

    pub fn cache_digests(&self, digests: Vec<D>) {
        let mut inner = self.inner.write();
        digests.into_iter().for_each(|d| {
            if let Some(old) = inner.push(d, ()) {
                if old.0 != d {
                    self.cache_evictions_counter.inc();
                }
            }
        });
    }

    pub fn is_verified<F, G>(&self, digest: D, verify_callback: F, uncached_checks: G) -> SuiResult
    where
        F: FnOnce() -> SuiResult,
        G: FnOnce() -> SuiResult,
    {
        if !self.is_cached(&digest) {
            verify_callback()?;
            self.cache_digest(digest);
        } else {
            // Checks that are required to be performed outside the cache.
            uncached_checks()?;
        }
        Ok(())
    }

    pub fn clear(&self) {
        let mut inner = self.inner.write();
        inner.clear();
    }
}
